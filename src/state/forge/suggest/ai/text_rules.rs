//! Shape rules for AI-proposed text where the field has one: a handle is an id, an initial is
//! a character or two, a tritype is three digits, a name has a sensible length.

use super::super::param::Param;
use super::value::MAX_TEXT_CHARS;
use crate::domain::profile::HANDLE_PREFIX;
use crate::state::forge::validate::{HANDLE_MAX_CHARS, HANDLE_MIN_CHARS, NAME_MAX_CHARS, NAME_MIN_CHARS};
use crate::state::forge::TextField;

/// Longest avatar initial (a letter pair or one symbol).
const MAX_INITIAL_CHARS: usize = 2;
/// Digits of a tritype.
const TRITYPE_DIGITS: usize = 3;
const HANDLE_JOINER: char = '_';

fn is_handle(text: &str) -> bool {
    let body = text.strip_prefix(HANDLE_PREFIX).unwrap_or(text);
    (HANDLE_MIN_CHARS..=HANDLE_MAX_CHARS).contains(&body.chars().count())
        && body.chars().all(|c| c.is_ascii_alphanumeric() || c == HANDLE_JOINER)
}

/// Whether `text` (trimmed, non-empty) is a usable value of `param`.
pub fn fits(param: Param, text: &str) -> bool {
    let length = text.chars().count();
    match param {
        Param::Text(TextField::Handle) => is_handle(text),
        Param::Text(TextField::AvatarInitial) => length <= MAX_INITIAL_CHARS,
        Param::Text(TextField::Tritype) => length == TRITYPE_DIGITS && text.chars().all(|c| c.is_ascii_digit()),
        Param::Text(TextField::Name) => (NAME_MIN_CHARS..=NAME_MAX_CHARS).contains(&length),
        _ => length <= MAX_TEXT_CHARS,
    }
}
