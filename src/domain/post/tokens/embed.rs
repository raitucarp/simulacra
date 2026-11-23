//! N1: `((post_id))` embeds another post; `((post_id@1.2.0))` pins the version it shows.
//! The target is a post id (letters, digits, `_` and `-`); any other target, or a version
//! that is not `major.minor.patch`, leaves the whole construct as text. `\(` writes a
//! literal parenthesis.

use super::BodyToken;
use crate::domain::data::history::SemVer;

/// Opening and closing bracket characters of an embed (each written twice).
const EMBED_OPEN_CHAR: char = '(';
const EMBED_CLOSE_CHAR: char = ')';
/// Separates the target from the pinned version.
const EMBED_VERSION_SEPARATOR: char = '@';
/// Opening delimiter of an embed, as written.
pub const EMBED_OPEN: &str = "((";
/// Closing delimiter of an embed, as written.
const EMBED_CLOSE: &str = "))";

/// Scans a closed `((target))` or `((target@version))` starting at `i`; returns the token and
/// the index after its trailing punctuation.
pub(super) fn scan_embed(chars: &[char], i: usize) -> Option<(BodyToken, usize)> {
    if chars.get(i) != Some(&EMBED_OPEN_CHAR) || chars.get(i + 1) != Some(&EMBED_OPEN_CHAR) {
        return None;
    }
    let start = i + 2;
    let len = (start..chars.len()).take_while(|&j| !is_stop(chars[j])).count();
    let end = start + len;
    if len == 0 || chars.get(end) != Some(&EMBED_CLOSE_CHAR) || chars.get(end + 1) != Some(&EMBED_CLOSE_CHAR) {
        return None;
    }
    let inner: String = chars[start..end].iter().collect();
    let (target, version) = split_version(&inner)?;
    let (trailing, next) = super::atoms::scan_trailing(chars, end + 2);
    Some((BodyToken::Embed { target, version, trailing }, next))
}

/// The target and the pinned version of an embed's inside; `None` when the version is malformed.
fn split_version(inner: &str) -> Option<(String, Option<SemVer>)> {
    let (target, version) = match inner.rsplit_once(EMBED_VERSION_SEPARATOR) {
        Some((target, version)) => (target, Some(SemVer::parse(version)?)),
        None => (inner, None),
    };
    (!target.is_empty() && target.chars().all(is_id_char)).then(|| (target.to_string(), version))
}

/// Characters of a post id: letters, digits, `_` and `-`.
fn is_id_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

fn is_stop(c: char) -> bool {
    c.is_whitespace() || c == EMBED_OPEN_CHAR || c == EMBED_CLOSE_CHAR
}

/// The embed written for `target`, pinned to `version` when given.
pub fn embed_markup(target: &str, version: Option<SemVer>) -> String {
    match version {
        Some(version) => format!("{EMBED_OPEN}{target}{EMBED_VERSION_SEPARATOR}{version}{EMBED_CLOSE}"),
        None => format!("{EMBED_OPEN}{target}{EMBED_CLOSE}"),
    }
}
