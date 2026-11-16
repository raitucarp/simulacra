//! Translation helpers for the forge: runtime keys, optional keys with fallbacks, and
//! `%{name}` interpolation of argument lists built at runtime (validation issues).

use crate::domain::TraitOption;
use crate::state::forge::field_label_key;
use rust_i18n::t;

const ARG_OPEN: &str = "%{";
const ARG_CLOSE: &str = "}";

/// The translation of `key` (the key itself when missing, like `t!`).
pub fn tr(key: &str) -> String {
    t!(key).to_string()
}

/// The translation of `key`, or `None` when no locale (nor the fallback) defines it.
pub fn tr_opt(key: &str) -> Option<String> {
    crate::_rust_i18n_try_translate(&rust_i18n::locale(), key).map(|text| text.to_string())
}

/// The translation of `key`, or `fallback` when it is missing.
pub fn tr_or(key: &str, fallback: &str) -> String {
    tr_opt(key).unwrap_or_else(|| fallback.to_string())
}

/// The translation of `key` with every `%{name}` replaced by its value from `args`.
pub fn tr_with(key: &str, args: &[(&str, String)]) -> String {
    args.iter().fold(tr(key), |text, (name, value)| text.replace(&format!("{ARG_OPEN}{name}{ARG_CLOSE}"), value))
}

/// Display name of a trait option (its built-in English label when not translated).
pub fn trait_label<T: TraitOption>(option: T) -> String {
    tr_or(&option.label_key(), option.label())
}

/// Translated description of a trait option, if one exists.
pub fn trait_description<T: TraitOption>(option: T) -> Option<String> {
    tr_opt(&option.description_key())
}

/// Display name of the field with serde name `path`.
pub fn field_label(path: &str) -> String {
    tr(&field_label_key(path))
}
