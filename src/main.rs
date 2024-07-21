mod check;

use check::missing_translations::MissingTranslations;
use check::Checker;
use indexmap::IndexMap;
use serde_yaml_ng::from_reader;
use serde_yaml_ng::Value as Yaml;
use std::env::args;
use std::fs::File;

/// Topgrade uses locale file version 2
const LOCALE_FILE_VERSION: i64 = 2;

/// Translations of various languages.
#[derive(Debug)]
struct Translations {
    /// English
    en: Option<String>,
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
#[derive(Debug)]
struct LocalizedTexts {
    texts: IndexMap<String, Translations>,
}

impl LocalizedTexts {
    /// Construct a [`LocalizedTexts`] from the given parsed yaml file.
    fn new(file_yaml: Yaml) -> Self {
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

fn main() {
    let file_name = args()
        .skip(1)
        .next()
        .expect("Error: a yaml file should be specified");
    let file = File::open(&file_name).unwrap_or_else(|e| {
        panic!(
            "Error: cannot open the specified file {file_name} due to error {:?}",
            e
        )
    });

    let contents: Yaml = from_reader(&file).unwrap();

    let localized_texts = LocalizedTexts::new(contents);

    let mut checker = Checker::new();
    checker.register_rule(MissingTranslations);

    checker.check(&localized_texts);

    checker.report_to_user();
}
