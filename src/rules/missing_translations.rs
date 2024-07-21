use super::Rule;

/// A rule that checks if there is any key that misses some translations.
pub(crate) struct MissingTranslations;

impl Rule for MissingTranslations {
    fn check(&self, localized_texts: &crate::LocalizedTexts) {
        for (key, translations) in localized_texts.texts.iter() {
            if translations.en.is_none() {
                Self::report_error(key.clone())
            }
        }
    }
}
