pub(crate) mod key_and_eng_matches;
pub(crate) mod missing_translations;
pub(crate) mod use_of_keys_do_not_exist;

use crate::LocalizedTexts;
use once_cell::sync::Lazy;
use std::collections::{hash_map::Entry, HashMap};

/// This is where errors found by [`Rule`]s are stored.
///
/// `HashMap<RuleName, Vec<(Key, OptionalErrorMessage)>>`
#[allow(clippy::type_complexity)] // just ignore it
pub(crate) static mut ERROR_STORAGE: Lazy<HashMap<String, Vec<(String, Option<String>)>>> =
    Lazy::new(HashMap::new);

/// Represents a rule that Topgrade's locale file should obey.
///
/// Implementations should implement the [`check()`] method, and invoke
/// [`report_error()`] when find any errors.
pub(crate) trait Rule {
    /// Name of this rule.
    fn name() -> &'static str
    where
        Self: Sized, // remove it from the vtable to make `trait Rule` object safe.
    {
        let full_name = std::any::type_name::<Self>();
        let maybe_start_idx = full_name.rfind(':');
        match maybe_start_idx {
            Some(start_idx) => &full_name[start_idx + 1..],
            None => "Unknown rule name",
        }
    }

    /// Implementations should invoke this when found an error.
    ///
    /// When `error_msg` is `Some`, it will be stored and reported to users as well.
    fn report_error(key: String, error_msg: Option<String>)
    where
        Self: Sized, // remove it from the vtable to make `trait Rule` object safe.
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
    fn check(
        &self,
        localized_texts: &LocalizedTexts,
        locale_keys: &[crate::locale_key_collector::LocaleKey],
    );
}
