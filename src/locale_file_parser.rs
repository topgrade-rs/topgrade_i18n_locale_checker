//! This file contains type [`LocalizedTexts`] which represents a parsed locale
//! file.

use indexmap::IndexMap;
use serde_yaml_ng::Value as Yaml;

/// Topgrade uses locale file version 2
const LOCALE_FILE_VERSION: i64 = 2;

/// Translations of various languages.
#[derive(Debug, PartialEq)]
pub(crate) struct Translations {
    /// English
    pub(crate) en: Option<String>,
}

impl Translations {
    /// Construct a [`Translation`] from the given `translation_mapping`.
    fn new(translation_yaml: Yaml) -> Self {
        match translation_yaml {
            Yaml::Null => Self { en: None },

            Yaml::Mapping(mut translation_mapping) => {
                let en = {
                    let opt_en_yaml = translation_mapping.remove("en");
                    opt_en_yaml.map(|opt_yaml| match opt_yaml {
                        Yaml::String(en) => en,
                        _ => panic!("Error: translation should be string"),
                    })
                };

                Self { en }
            }

            _ => panic!("Error: invalid format for translation"),
        }
    }
}

/// Represents all the localized texts used by Topgrade.
#[derive(Debug, PartialEq)]
pub(crate) struct LocalizedTexts {
    /// Locale key => All the translations.
    pub(crate) texts: IndexMap<String, Translations>,
}

impl LocalizedTexts {
    /// Construct a [`LocalizedTexts`] from the given parsed yaml file.
    pub(crate) fn new(file_yaml: Yaml) -> Self {
        let mut file_mapping = match file_yaml {
            Yaml::Mapping(mapping) => mapping,
            _ => panic!("The outer level container should be a mapping"),
        };

        let locale_file_version = file_mapping
            .remove("_version")
            .unwrap_or_else(|| panic!("Error: local file version key `_version` not found"))
            .as_i64()
            .expect("Error: locale file version number should be a number");
        if locale_file_version != LOCALE_FILE_VERSION {
            panic!("Error: locale file version should be 2");
        }

        let mut texts = IndexMap::with_capacity(file_mapping.len());
        for (key, translations_yaml) in file_mapping {
            let key = match key {
                Yaml::String(key) => key,
                _ => panic!("Error: locale translation key should be a string"),
            };

            let translations = Translations::new(translations_yaml);

            texts.insert(key, translations);
        }

        Self { texts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Error: locale translation key should be a string")]
    fn test_key_should_be_string() {
        let yaml_str = r#"
_version: 2
1: 
  en: "en"
"#;
        let yaml: Yaml = serde_yaml_ng::from_str(yaml_str).unwrap();
        LocalizedTexts::new(yaml);
    }

    #[test]
    #[should_panic(expected = "Error: locale file version should be 2")]
    fn test_should_have_version_2() {
        let yaml_str = r#"
_version: 1
"with_no_en":
"with_en":
  en: "with_en""#;
        let yaml: Yaml = serde_yaml_ng::from_str(yaml_str).unwrap();
        LocalizedTexts::new(yaml);
    }

    #[test]
    #[should_panic(expected = "Error: local file version key `_version` not found")]
    fn test_version_not_found() {
        let yaml_str = r#"
"with_no_en":
"with_en":
  en: "with_en""#;
        let yaml: Yaml = serde_yaml_ng::from_str(yaml_str).unwrap();
        LocalizedTexts::new(yaml);
    }

    #[test]
    fn test_localized_texts() {
        let yaml_str = r#"
_version: 2
"with_no_en":
"with_en":
  en: "with_en""#;
        let yaml: Yaml = serde_yaml_ng::from_str(yaml_str).unwrap();
        let parsed = LocalizedTexts::new(yaml);

        let expected = LocalizedTexts {
            texts: IndexMap::from_iter(vec![
                ("with_no_en".to_string(), Translations { en: None }),
                (
                    "with_en".to_string(),
                    Translations {
                        en: Some("with_en".to_string()),
                    },
                ),
            ]),
        };

        assert_eq!(parsed, expected);
    }
}
