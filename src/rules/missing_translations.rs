use super::Rule;
use bitflags::bitflags;

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
    fn check(&self, localized_texts: &crate::LocalizedTexts) {
        for (key, translations) in localized_texts.texts.iter() {
            let mut missing_langs = MissingLanguages::empty();

            if translations.en.is_none() {
                missing_langs.insert(MissingLanguages::En);
            }

            if !missing_langs.is_empty() {
                Self::report_error(key.clone(), Some(missing_langs.error_msg()));
            }
        }
    }
}
