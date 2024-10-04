//! A rule that checks if Topgrade uses any locale keys that do not exist.

use super::Rule;
use std::collections::HashMap;

/// Checks if Topgrade uses any locale keys that do not exist.
pub(crate) struct UseOfKeysDoNotExist;

impl Rule for UseOfKeysDoNotExist {
    fn check(
        &self,
        localized_texts: &crate::locale_file_parser::LocalizedTexts,
        locale_keys: &[crate::locale_key_collector::LocaleKey],
        erros: &mut HashMap<String, Vec<(String, Option<String>)>>,
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
                    erros,
                );
            }
        }
    }
}
