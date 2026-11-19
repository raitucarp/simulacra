//! Translated strings and times of the data views.

use jiff::Timestamp;
use rust_i18n::t;

/// Local date and time as the history and restore sheets show them.
const TIME_FORMAT: &str = "%Y-%m-%d %H:%M";

/// The translation of `key`.
pub fn tr(key: &str) -> String {
    t!(key).to_string()
}

/// `at` in local time.
pub fn format_time(at: Timestamp) -> String {
    crate::time::local(at).strftime(TIME_FORMAT).to_string()
}
