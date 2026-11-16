//! Translation helpers for the forge: runtime keys, optional keys with fallbacks, and
//! `%{name}` interpolation of argument lists built at runtime (validation issues, cadence gaps).

use crate::agent::schedule::cadence_rule;
use crate::domain::{PostingCadence, TraitOption};
use crate::state::forge::field_label_key;
use rust_i18n::t;

const ARG_OPEN: &str = "%{";
const ARG_CLOSE: &str = "}";
/// Placeholder of a posting cadence's minimum gap in its label and description.
pub const ARG_MINUTES: &str = "minutes";

/// The translation of `key` (the key itself when missing, like `t!`).
pub fn tr(key: &str) -> String {
    t!(key).to_string()
}

/// The translation of `key`, or `None` when no locale (nor the fallback) defines it.
pub fn tr_opt(key: &str) -> Option<String> {
    crate::_rust_i18n_try_translate(&rust_i18n::locale(), key).map(|text| text.to_string())
}

/// `text` with every `%{name}` replaced by its value from `args`.
fn interpolate(text: String, args: &[(&str, String)]) -> String {
    args.iter().fold(text, |text, (name, value)| text.replace(&format!("{ARG_OPEN}{name}{ARG_CLOSE}"), value))
}

/// The translation of `key` with every `%{name}` replaced by its value from `args`.
pub fn tr_with(key: &str, args: &[(&str, String)]) -> String {
    interpolate(tr(key), args)
}

/// Runtime arguments of a trait label or description key: a posting cadence's `%{minutes}`
/// is the scheduler's real minimum gap for it.
fn trait_args(key: &str) -> Vec<(&'static str, String)> {
    PostingCadence::ALL
        .into_iter()
        .find(|cadence| key == cadence.label_key() || key == cadence.description_key())
        .and_then(cadence_rule)
        .map(|rule| vec![(ARG_MINUTES, rule.min_interval_minutes.to_string())])
        .unwrap_or_default()
}

/// The translation of trait label or description `key` with its runtime arguments filled in.
pub fn tr_trait(key: &str) -> Option<String> {
    tr_opt(key).map(|text| interpolate(text, &trait_args(key)))
}

/// Display name of a trait option (its built-in English label when not translated).
pub fn trait_label<T: TraitOption>(option: T) -> String {
    tr_trait(&option.label_key()).unwrap_or_else(|| option.label().to_string())
}

/// Translated description of a trait option, if one exists.
pub fn trait_description<T: TraitOption>(option: T) -> Option<String> {
    tr_trait(&option.description_key())
}

/// Display name of the field with serde name `path`.
pub fn field_label(path: &str) -> String {
    tr(&field_label_key(path))
}
