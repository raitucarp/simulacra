//! Lenient matching of free-form option text (from an LLM or a person) to trait options.

use crate::domain::TraitOption;

/// Separators an LLM or a human may use in place of the snake_case `_`.
const KEY_SEPARATORS: [char; 2] = ['-', ' '];
const KEY_SEPARATOR: &str = "_";

/// Normalizes free-form option text towards a snake_case key.
fn normalize(raw: &str) -> String {
    raw.trim().to_lowercase().replace(KEY_SEPARATORS, KEY_SEPARATOR)
}

/// The first word of a label, lower-cased (`"5w4 · The Iconoclast"` → `"5w4"`).
fn first_word(label: &str) -> String {
    label.split_whitespace().next().unwrap_or_default().to_lowercase()
}

/// The option of `T` whose key, label, short label or label's first word matches `raw`
/// (case-insensitively).
pub(super) fn find_option<T: TraitOption>(raw: &str) -> Option<T> {
    let wanted = normalize(raw);
    T::ALL.iter().copied().find(|option| {
        option.key() == wanted
            || normalize(option.label()) == wanted
            || normalize(option.short_label()) == wanted
            || first_word(option.label()) == wanted
    })
}
