//! Translation of runtime-built i18n keys.

use rust_i18n::t;

/// The translation of `key` in the active locale.
pub(super) fn tr(key: &str) -> String {
    t!(key).to_string()
}
