//! This file contains the checker type.

use crate::locale_file_parser::LocalizedTexts;
use crate::locale_key_collector::LocaleKey;
use crate::rules::Rule;
use std::collections::HashMap;

/// This type and its methods are the code where we check the locale file.
pub(crate) struct Checker {
    /// The registered (will be applied) rule
    rules: Vec<Box<dyn Rule>>,
    /// `HashMap<RuleName, Vec<(Key, OptionalErrorMessage)>>`
    errors: HashMap<String, Vec<(String, Option<String>)>>,
}

impl Checker {
    /// Creates a new checker with 0 rule registered.
    pub(crate) fn new() -> Self {
        Self {
            rules: Vec::new(),
            errors: HashMap::new(),
        }
    }

    /// Register a rule.
    pub(crate) fn register_rule(&mut self, rule: impl Rule + 'static) {
        self.rules.push(Box::new(rule))
    }

    /// Run the check process.
    pub(crate) fn check(&mut self, localized_texts: &LocalizedTexts, locale_keys: &[LocaleKey]) {
        for rule in self.rules.iter() {
            rule.check(localized_texts, locale_keys, &mut self.errors)
        }
    }

    /// Returns the number of errors stored in `self.errors`.
    fn n_errors(&self) -> usize {
        self.errors.values().map(|errors| errors.len()).sum()
    }

    /// Return true if there is no error.
    pub(crate) fn has_error(&self) -> bool {
        self.n_errors() != 0
    }

    /// Print the errors that are found in a human-readable way.
    pub(crate) fn report_to_user(&self) {
        if !self.has_error() {
            println!("No error found!");
        } else {
            println!("Errors Found:");

            for (rule, errors) in self.errors.iter() {
                println!("  {}", rule);
                for (key, opt_error_msg) in errors {
                    print!("    {}", key);
                    match opt_error_msg {
                        Some(error_msg) => println!(": {}", error_msg),
                        None => println!(),
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
    fn test_n_errors_and_has_error() {
        let mut checker = Checker::new();
        assert_eq!(checker.n_errors(), 0);
        assert!(!checker.has_error());
        checker.errors.insert("rule_name".to_string(), Vec::new());
        assert_eq!(checker.n_errors(), 0);
        assert!(!checker.has_error());

        checker
            .errors
            .insert("rule_name2".into(), vec![("locale_key".into(), None)]);
        assert_eq!(checker.n_errors(), 1);
        assert!(checker.has_error());

        let errors_of_rule_name = checker.errors.get_mut("rule_name").unwrap();
        errors_of_rule_name.push(("locale_key".into(), None));
        assert_eq!(checker.n_errors(), 2);
        assert!(checker.has_error());
    }
}
