use super::Rule;
use crate::locale_file_parser::LocalizedTexts;
use crate::locale_key_collector::LocaleKey;
use bitflags::bitflags;
use std::collections::HashMap;

bitflags! {
    /// A bitflag represent the missing languages, every language would take 1 bit.
    ///
    /// When a language is missing, its bit will be set to 1.
    #[derive(Debug, PartialEq, Eq)]
    struct MissingLanguages: u8 {
        const En = 0b_0000_0001;
    }
}

impl MissingLanguages {
    /// Returns a error message describing the missing languages.
    fn error_msg(&self) -> String {
        let mut str = String::from("Missing translations for ");
        str.push('[');
        for lang in self.iter() {
            if lang == MissingLanguages::En {
                str.push_str("English")
            }
        }
        str.push(']');

        str
    }
}

/// A rule that checks if there is any key that misses some translations.
pub(crate) struct MissingTranslations;

impl Rule for MissingTranslations {
    fn check(
        &self,
        localized_texts: &LocalizedTexts,
        _locale_keys: &[LocaleKey],
        errors: &mut HashMap<String, Vec<(String, Option<String>)>>,
    ) {
        for (key, translations) in localized_texts.texts.iter() {
            let mut missing_langs = MissingLanguages::empty();

            if translations.en.is_none() {
                missing_langs.insert(MissingLanguages::En);
            }

            if !missing_langs.is_empty() {
                Self::report_error(key.clone(), Some(missing_langs.error_msg()), errors);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::locale_file_parser::Translations;
    use indexmap::IndexMap;

    #[test]
    fn test_missing_en() {
        let localized_texts = LocalizedTexts {
            texts: IndexMap::from([
                ("Restarting {app}".into(), Translations { en: None }),
                ("Restarting {topgrade}".into(), Translations { en: None }),
                (
                    "Restarting {ba}".into(),
                    Translations {
                        en: Some("Restarting %{ba}".into()),
                    },
                ),
            ]),
        };
        let mut errors = HashMap::new();
        let rule = MissingTranslations;
        rule.check(&localized_texts, &[], &mut errors);
        let expected_errors = HashMap::from([(
            <MissingTranslations as Rule>::name().to_string(),
            vec![
                (
                    "Restarting {app}".to_string(),
                    Some("Missing translations for [English]".into()),
                ),
                (
                    "Restarting {topgrade}".to_string(),
                    Some("Missing translations for [English]".into()),
                ),
            ],
        )]);
        assert_eq!(errors, expected_errors);
    }

    #[test]
    fn test_no_missing_translations() {
        let localized_texts = LocalizedTexts {
            texts: IndexMap::from([
                (
                    "Restarting {app}".into(),
                    Translations {
                        en: Some("whatever".into()),
                    },
                ),
                (
                    "Restarting {topgrade}".into(),
                    Translations {
                        en: Some("wahtever".into()),
                    },
                ),
                (
                    "Restarting {ba}".into(),
                    Translations {
                        en: Some("Restarting %{ba}".into()),
                    },
                ),
            ]),
        };
        let mut errors = HashMap::new();
        let rule = MissingTranslations;
        rule.check(&localized_texts, &[], &mut errors);
        let expected_errors = HashMap::new();
        assert_eq!(errors, expected_errors);
    }
}
