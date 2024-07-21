pub(crate) mod missing_translations;

use crate::LocalizedTexts;
use once_cell::sync::Lazy;
use std::collections::{hash_map::Entry, HashMap};

/// This is where errors found by [`Rule`]s are stored.
static mut ERROR_STORAGE: Lazy<HashMap<String, Vec<(String, String)>>> =
    Lazy::new(|| HashMap::new());

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

/// Represents a rule that Topgrade's locale file should obey.
///
/// Implementations should implement the [`check()`] method, and invoke
/// [`report_error()`] when find any errors.
pub(crate) trait Rule {
    /// Name of this rule.
    fn name() -> &'static str
    where
        Self: Sized, // remove it from the vtable
    {
        let full_name = std::any::type_name::<Self>();
        let maybe_start_idx = full_name.rfind(':');
        match maybe_start_idx {
            Some(start_idx) => &full_name[start_idx + 1..],
            None => "UNKNOWN",
        }
    }

    /// Implementations should invoke this when found an error.
    fn report_error(key: String, error_msg: String)
    where
        Self: Sized, // remove it from the vtable
    {
        // SAFETY:
        // It is safe to directly modify the global static variable as there is only 1 thread.
        unsafe {
            match ERROR_STORAGE.entry(Self::name().to_string()) {
                Entry::Occupied(mut o) => {
                    o.get_mut().push((key, error_msg));
                }
                Entry::Vacant(v) => {
                    v.insert(vec![(key, error_msg)]);
                }
            }
        }
    }

    /// Begin the check.
    fn check(&self, localized_texts: &LocalizedTexts);
}
