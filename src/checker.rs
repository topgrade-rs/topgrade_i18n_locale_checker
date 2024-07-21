//! This file contains the checker type.

use crate::parse::LocalizedTexts;
use crate::rules::Rule;
use crate::rules::ERROR_STORAGE;

/// This type and its methods are the code where we check the locale file.
pub(crate) struct Checker {
    /// The registered (will be applied) rule.
    rules: Vec<Box<dyn Rule>>,
}

impl Checker {
    /// Creates a new checker with 0 rule registered.
    pub(crate) fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Register a rule.
    pub(crate) fn register_rule(&mut self, rule: impl Rule + 'static) {
        self.rules.push(Box::new(rule))
    }

    /// Run the check process.
    pub(crate) fn check(&self, localized_texts: &LocalizedTexts) {
        for rule in self.rules.iter() {
            rule.check(localized_texts)
        }
    }

    /// Print the errors that are found in a human-readable way.
    pub(crate) fn report_to_user(&self) {
        println!("Errors Found:");

        // SAFETY:
        // It is safe to directly modify the global static variable as there is only 1 thread.
        unsafe {
            let n_errors: usize = ERROR_STORAGE
                .iter()
                .map(|(_key, errors)| errors.len())
                .sum();

            if n_errors == 0 {
                println!("  No error");
            } else {
                for (rule, errors) in ERROR_STORAGE.iter() {
                    println!("  {}", rule);
                    for (key, error) in errors {
                        println!("    {}: {}", key, error);
                    }
                }
            }
        }
    }
}
