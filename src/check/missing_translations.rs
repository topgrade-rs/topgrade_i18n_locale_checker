use std::fmt::Display;

use super::Rule;

/// A rule that checks if there is any key that misses some translations.
pub(crate) struct MissingTranslations;

enum MissingLang {
    En,
}

impl Display for MissingLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::En => write!(f, "Missing English(en) translation"),
        }
    }
}

impl Rule for MissingTranslations {
    fn check(&self, localized_texts: &crate::LocalizedTexts) {
        for (key, translations) in localized_texts.texts.iter() {
            if translations.en.is_none() {
                Self::report_error(key.clone(), MissingLang::En.to_string())
            }
        }
    }
}
