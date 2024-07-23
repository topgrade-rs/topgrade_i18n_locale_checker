use parser::{LocaleKeyParser, LocaleToken};

use super::Rule;

/// A rules that enforces a locale's key matches its English translation.
///
/// This is not requested by rust-i18n (The i18n framework Topgrade uses), it is
/// simply our convention.
pub(crate) struct KeyEngMatches;

impl Rule for KeyEngMatches {
    fn check(&self, localized_texts: &crate::parse::LocalizedTexts) {
        for (key, translations) in localized_texts.texts.iter() {
            let en = &translations.en;

            if en.is_none() {
                Self::report_error(key.clone(), Some("Missing English translation".into()));
                return;
            }

            let mut parser = LocaleKeyParser::new();
            parser.parse(key);
            let expected = key_to_en(&parser);

            let en = en.as_ref().unwrap();

            if en != &expected {
                Self::report_error(key.clone(), None)
            }
        }
    }
}

mod parser {
    const LEFT_BRACE: &str = "{";
    const RIGHT_BRACE: &str = "}";

    /// A locale token in the key.
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) enum LocaleToken<'token> {
        /// It is not surrounded by a pair of braces
        WithoutBrace(&'token str),
        /// It is surrounded by a pair of braces
        WithinBrace(&'token str),
    }

    /// Key parser.
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct LocaleKeyParser<'input> {
        /// tokens
        tokens: Vec<LocaleToken<'input>>,
    }

    impl<'input> LocaleKeyParser<'input> {
        /// Create a parser with nothing.
        pub(crate) fn new() -> Self {
            Self { tokens: Vec::new() }
        }

        /// Accesses the parsed tokens.
        pub(crate) fn tokens(&self) -> &[LocaleToken<'input>] {
            &self.tokens
        }

        /// Parses the `input`, stores the parsed tokens in `self`.
        pub(crate) fn parse<'slf>(&'slf mut self, input: &'input str) {
            let len = input.len();
            let mut start_offset = 0;

            while start_offset < len {
                let opt_left_brace_location = input[start_offset..].find(LEFT_BRACE);

                match opt_left_brace_location {
                    None => {
                        self.tokens
                            .push(LocaleToken::WithoutBrace(&input[start_offset..]));
                        return;
                    }
                    Some(mut left_brace_location) => {
                        left_brace_location += start_offset;

                        let opt_right_brace_location =
                            input[left_brace_location..].find(RIGHT_BRACE);

                        match opt_right_brace_location {
                            None => {
                                self.tokens
                                    .push(LocaleToken::WithoutBrace(&input[start_offset..]));
                                return;
                            }
                            Some(mut right_brace_location) => {
                                right_brace_location += left_brace_location;
                                // handle the part without brace
                                if left_brace_location != start_offset {
                                    self.tokens.push(LocaleToken::WithoutBrace(
                                        &input[start_offset..left_brace_location],
                                    ));
                                }

                                self.tokens.push(LocaleToken::WithinBrace(
                                    &input[left_brace_location + 1..=right_brace_location - 1],
                                ));

                                start_offset = right_brace_location + 1;
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn no_brace() {
            let mut parser = LocaleKeyParser::new();
            parser.parse("without_any_brace");

            for token in parser.tokens() {
                assert!(matches!(token, LocaleToken::WithoutBrace(_)));
            }
        }

        #[test]
        fn starts_with_brace() {
            let mut parser = LocaleKeyParser::new();
            parser.parse("{brace}topgrade");

            let expected = LocaleKeyParser {
                tokens: vec![
                    LocaleToken::WithinBrace("brace"),
                    LocaleToken::WithoutBrace("topgrade"),
                ],
            };

            assert_eq!(parser, expected);
        }

        #[test]
        fn ends_with_brace() {
            let mut parser = LocaleKeyParser::new();
            parser.parse("topgrade{brace}");

            let expected = LocaleKeyParser {
                tokens: vec![
                    LocaleToken::WithoutBrace("topgrade"),
                    LocaleToken::WithinBrace("brace"),
                ],
            };

            assert_eq!(parser, expected);
        }

        #[test]
        fn brace_in_the_middle() {
            let mut parser = LocaleKeyParser::new();
            parser.parse("topgrade{brace}topgrade");

            let expected = LocaleKeyParser {
                tokens: vec![
                    LocaleToken::WithoutBrace("topgrade"),
                    LocaleToken::WithinBrace("brace"),
                    LocaleToken::WithoutBrace("topgrade"),
                ],
            };

            assert_eq!(parser, expected);
        }

        #[test]
        fn continuous_braces() {
            let mut parser = LocaleKeyParser::new();
            parser.parse("{brace}{brace}");

            let expected = LocaleKeyParser {
                tokens: vec![
                    LocaleToken::WithinBrace("brace"),
                    LocaleToken::WithinBrace("brace"),
                ],
            };

            assert_eq!(parser, expected);
        }

        #[test]
        fn continuous_braces_in_the_middle() {
            let mut parser = LocaleKeyParser::new();
            parser.parse("topgrade{brace}{brace}topgrade");

            let expected = LocaleKeyParser {
                tokens: vec![
                    LocaleToken::WithoutBrace("topgrade"),
                    LocaleToken::WithinBrace("brace"),
                    LocaleToken::WithinBrace("brace"),
                    LocaleToken::WithoutBrace("topgrade"),
                ],
            };

            assert_eq!(parser, expected);
        }

        #[test]
        fn single_left_brace() {
            let mut parser = LocaleKeyParser::new();
            parser.parse("{");

            let expected = LocaleKeyParser {
                tokens: vec![LocaleToken::WithoutBrace("{")],
            };

            assert_eq!(parser, expected);
        }

        #[test]
        fn mutliple_left_brace() {
            let mut parser = LocaleKeyParser::new();
            parser.parse("x{x{x{");

            let expected = LocaleKeyParser {
                tokens: vec![LocaleToken::WithoutBrace("x{x{x{")],
            };

            assert_eq!(parser, expected);
        }

        #[test]
        fn a_pair_in_chaos() {
            let mut parser = LocaleKeyParser::new();
            parser.parse("}{x{x}{{x{");

            let expected = LocaleKeyParser {
                tokens: vec![
                    LocaleToken::WithoutBrace("}"),
                    LocaleToken::WithinBrace("x{x"),
                    LocaleToken::WithoutBrace("{{x{"),
                ],
            };

            assert_eq!(parser, expected);
        }
    }
}

/// Helper function to convert a locale key to its English translation by
/// prepending a `%` to the tokens serrounded by `{}`.
fn key_to_en(parser: &parser::LocaleKeyParser<'_>) -> String {
    let mut ret = String::new();
    for token in parser.tokens() {
        match token {
            LocaleToken::WithinBrace(str) => {
                std::fmt::write(&mut ret, format_args!("%{{{}}}", str)).unwrap()
            }
            LocaleToken::WithoutBrace(str) => {
                std::fmt::write(&mut ret, format_args!("{}", str)).unwrap()
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use parser::LocaleKeyParser;

    use super::*;

    #[test]
    fn preprend_percent_works() {
        let mut parser = LocaleKeyParser::new();
        parser.parse("hello, {topgrade}");

        assert_eq!(key_to_en(&parser).as_str(), "hello, %{topgrade}");
    }

    #[test]
    fn preprend_percent_works_without_brace() {
        let mut parser = LocaleKeyParser::new();
        parser.parse("hello, topgrade");

        assert_eq!(key_to_en(&parser).as_str(), "hello, topgrade");
    }
}
