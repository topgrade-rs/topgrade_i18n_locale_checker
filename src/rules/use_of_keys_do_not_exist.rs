//! A rule that checks if Topgrade uses any locale keys that do not exist.

use super::Rule;
use crate::locale_file_parser::LocalizedTexts;
use crate::locale_key_collector::LocaleKey;
use std::collections::HashMap;

/// Checks if Topgrade uses any locale keys that do not exist.
pub(crate) struct UseOfKeysDoNotExist;

impl Rule for UseOfKeysDoNotExist {
    fn check(
        &self,
        localized_texts: &LocalizedTexts,
        locale_keys: &[LocaleKey],
        errors: &mut HashMap<String, Vec<(String, Option<String>)>>,
    ) {
        for locale_key in locale_keys {
            if !localized_texts.texts.contains_key(&locale_key.key) {
                Self::report_error(
                    format!(
                        "file '{}' / line '{}' / column '{}' / key '{}'",
                        locale_key.file.display(),
                        locale_key.line,
                        locale_key.column,
                        locale_key.key
                    ),
                    None,
                    errors,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use std::path::Path;

    use super::*;

    #[test]
    fn test_rule_works() {
        let localized_texts = LocalizedTexts {
            texts: IndexMap::new(),
        };
        let locale_keys = vec![LocaleKey {
            key: "Restarting".into(),
            file: Path::new("foo.rs"),
            line: 1,
            column: 1,
        }];
        let mut errors = HashMap::new();
        let rule = UseOfKeysDoNotExist;
        rule.check(&localized_texts, &locale_keys, &mut errors);
        let expected_errors = HashMap::from([(
            <UseOfKeysDoNotExist as Rule>::name().into(),
            vec![(
                "file 'foo.rs' / line '1' / column '1' / key 'Restarting'".into(),
                None,
            )],
        )]);
        assert_eq!(errors, expected_errors);

        let localized_texts = LocalizedTexts {
            texts: IndexMap::from([(
                "Restarting".into(),
                Translations {
                    en: Some("Restarting".into()),
                },
            )]),
        };
        let locale_keys = vec![LocaleKey {
            key: "Restarting".into(),
            file: Path::new("foo.rs"),
            line: 1,
            column: 1,
        }];
        let mut errors = HashMap::new();
        let rule = UseOfKeysDoNotExist;
        rule.check(&localized_texts, &locale_keys, &mut errors);
        let expected_errors = HashMap::new();
        assert_eq!(errors, expected_errors);
    }
}
