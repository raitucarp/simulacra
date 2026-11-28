//! Translated strings and times of the data views.

use jiff::Timestamp;
use rust_i18n::t;

/// A local date.
const DATE_FORMAT: &str = "%Y-%m-%d";

/// The translation of `key`.
pub fn tr(key: &str) -> String {
    t!(key).to_string()
}

/// `at`'s local date (N15: the date of a post in a distilled document).
pub fn format_date(at: Timestamp) -> String {
    crate::time::local(at).strftime(DATE_FORMAT).to_string()
}

/// `at` in local time, as the date and clock settings write it.
pub fn format_time(at: Timestamp) -> String {
    crate::ui::format::date_time(at)
}
