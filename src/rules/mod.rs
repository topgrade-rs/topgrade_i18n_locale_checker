pub(crate) mod key_and_en_matches;
pub(crate) mod missing_translations;

use crate::LocalizedTexts;
use once_cell::sync::Lazy;
use std::collections::{hash_map::Entry, HashMap};

/// This is where errors found by [`Rule`]s are stored.
pub(crate) static mut ERROR_STORAGE: Lazy<HashMap<String, Vec<String>>> = Lazy::new(HashMap::new);

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
    fn report_error(key: String)
    where
        Self: Sized, // remove it from the vtable
    {
        // SAFETY:
        // It is safe to directly modify the global static variable as there is only 1 thread.
        unsafe {
            match ERROR_STORAGE.entry(Self::name().to_string()) {
                Entry::Occupied(mut o) => {
                    o.get_mut().push(key);
                }
                Entry::Vacant(v) => {
                    v.insert(vec![key]);
                }
            }
        }
    }

    /// Begin the check.
    fn check(&self, localized_texts: &LocalizedTexts);
}
