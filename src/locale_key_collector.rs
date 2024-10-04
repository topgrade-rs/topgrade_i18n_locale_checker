//! This files contains a `LocaleKeyCollector` type that finds the invocation
//! of `rust_i18n::t!()` in Topgrade's source code and extracts the locale
//! key.

use proc_macro2::TokenTree;
use std::borrow::Cow;
use std::path::Path;
use syn::spanned::Spanned;
use syn::visit::Visit;

/// A collector that finds the invocation of `rust_i18n::t!()` macro and collects
/// its locale key.
#[derive(Debug)]
pub(crate) struct LocaleKeyCollector<'path> {
    /// Collected locale keys.
    locale_keys: Vec<LocaleKey<'path>>,
}

impl<'path> LocaleKeyCollector<'path> {
    /// Creates a new collector with keys set empty.
    pub(crate) fn new() -> Self {
        Self {
            locale_keys: Vec::new(),
        }
    }

    /// Collects the invocation of `t!()` from `files`.
    pub(crate) fn collect(&mut self, files: &'path [Cow<'path, Path>]) {
        for file in files {
            let str = std::fs::read_to_string(file)
                .unwrap_or_else(|err| panic!("failed to read file {}: {}", file.display(), err));
            let parsed_file = syn::parse_file(&str)
                .unwrap_or_else(|e| panic!("failed to parse file {} due to {}", file.display(), e));

            let mut single_file_collector = SingleFileLocalenKeyCollector {
                file,
                locale_keys: Vec::new(),
            };

            single_file_collector.visit_file(&parsed_file);

            self.locale_keys
                .extend(single_file_collector.locale_keys.into_iter());
        }
    }

    /// Gets the reference to the collected locale keys.
    pub(crate) fn locale_keys(&self) -> &[LocaleKey<'path>] {
        &self.locale_keys
    }
}

// FileVisitor -> visit_file -> visit_macro -> (TranslationKey)

/// Collector that is responsible for a single file.
///
/// # NOTE
/// This is a workaround to enable us to have the file path info while
/// invoking `visit_macro()`.
struct SingleFileLocalenKeyCollector<'path> {
    /// File path.
    file: &'path Path,
    /// Keys collected from `file`.
    locale_keys: Vec<LocaleKey<'path>>,
}

impl<'ast, 'path> Visit<'ast> for SingleFileLocalenKeyCollector<'path> {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        let path_segments = &i.path.segments;
        let path_segments_len = path_segments.len();

        let last_segment = path_segments
            .last()
            .expect("macro invocation should have at least 1 path segment");
        if last_segment.ident == "t" {
            // invocation: t!()
            if path_segments_len == 1 {
                self.locale_keys.push(LocaleKey::new(i, self.file));
            }

            if path_segments_len == 2 {
                let first_segment = path_segments.get(0).expect("len == 2");
                // invocation: rust_i18n::t!()
                if first_segment.ident == "rust_i18n" {
                    self.locale_keys.push(LocaleKey::new(i, self.file));
                }
            }
        }

        syn::visit::visit_macro(self, i);
    }
}

/// Info about a locale key.
#[derive(Debug, PartialEq)]
pub(crate) struct LocaleKey<'path> {
    /// Locale key.
    pub(crate) key: String,
    /// path of the file where the `t!()` macro is invoked.
    pub(crate) file: &'path Path,
    /// Line number of the start of invocation, starts from 1.
    pub(crate) line: usize,
    /// Column number of the start of invocation, starts from 0.
    pub(crate) column: usize,
}

impl<'path> LocaleKey<'path> {
    /// Constructs a `LocaleKey` from the given info.
    fn new(mac: &syn::Macro, file: &'path Path) -> Self {
        let token_stream = mac.tokens.clone();

        let mut token_tree_iter = token_stream.into_iter();
        let translation_key = token_tree_iter
            .next()
            .expect("t!() needs at least 1 argument");
        let key = match translation_key {
            TokenTree::Literal(literal) => literal.to_string().trim_matches('"').to_string(),
            _ => panic!("The first argument to t!() should be a string literal"),
        };

        let span = mac.span();
        let start = span.start();
        let line = start.line;
        let column = start.column;

        Self {
            key,
            file,
            line,
            column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_single_file_collector_works() {
        let file_contents = r#"t!("first_key");
 rust_i18n::t!("second_key");
foo::bar::t!("not a key");
::foo::bar::t!("not a key");
"#;
        let path = PathBuf::from("foo.rs");
        let mut collector = SingleFileLocalenKeyCollector {
            file: &path,
            locale_keys: Vec::new(),
        };
        collector.visit_file(&syn::parse_file(&file_contents).unwrap());

        assert_eq!(
            collector.locale_keys,
            vec![
                LocaleKey {
                    key: "first_key".to_string(),
                    file: Path::new("foo.rs"),
                    line: 1,
                    column: 0
                },
                LocaleKey {
                    key: "second_key".to_string(),
                    file: Path::new("foo.rs"),
                    line: 2,
                    column: 1
                },
            ]
        );
    }

    #[test]
    #[should_panic(expected = "The first argument to t!() should be a string literal")]
    fn test_single_file_collector_locale_key_is_not_string_literal() {
        let file_contents = r#"
t!(key);
"#;
        let path = PathBuf::from("foo.rs");
        let mut collector = SingleFileLocalenKeyCollector {
            file: &path,
            locale_keys: Vec::new(),
        };
        collector.visit_file(&syn::parse_file(&file_contents).unwrap());
    }
}
