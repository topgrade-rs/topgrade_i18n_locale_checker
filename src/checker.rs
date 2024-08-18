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

    /// Return true if there is no error.
    pub(crate) fn has_error(&self) -> bool {
        // SAFETY:
        // It is safe to directly modify the global static variable as there is only 1 thread.
        let n_errors: usize = unsafe {
            ERROR_STORAGE
                .iter()
                .map(|(_key, errors)| errors.len())
                .sum()
        };

        n_errors != 0
    }

    /// Print the errors that are found in a human-readable way.
    pub(crate) fn report_to_user(&self) {
        // SAFETY:
        // It is safe to directly modify the global static variable as there is only 1 thread.
        unsafe {
            if !self.has_error() {
                println!("No error found!");
            } else {
                println!("Errors Found:");

                for (rule, errors) in ERROR_STORAGE.iter() {
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
}
