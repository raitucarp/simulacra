//! Atomic inline tokens: `[[wikilinks]]`, `#hashtags` and web links, each with trailing punctuation.

use super::BodyToken;
use crate::domain::post::parse::HASHTAG_SIGIL as HASHTAG_MARKER;

/// Opening bracket character of a wikilink delimiter.
const WIKILINK_OPEN_CHAR: char = '[';
/// Closing bracket character of a wikilink delimiter.
const WIKILINK_CLOSE_CHAR: char = ']';
/// Characters that end a hashtag because they write marks or escapes.
const MARK_CHARS: [char; 4] = ['*', '|', '`', '\\'];
/// Underscore: allowed inside a tag, but a doubled one is an underline delimiter.
const UNDERSCORE: char = '_';

/// Returns true if the character is an ASCII or CJK trailing punctuation mark.
#[inline]
pub fn is_trailing_punctuation(c: char) -> bool {
    matches!(
        c,
        ',' | '.'
            | '!'
            | '?'
            | ';'
            | ':'
            | '，' // U+FF0C fullwidth comma
            | '。' // U+3002 ideographic full stop
            | '！' // U+FF01 fullwidth exclamation
            | '？' // U+FF1F fullwidth question
            | '；' // U+FF1B fullwidth semicolon
            | '：' // U+FF1A fullwidth colon
            | '…'  // U+2026 horizontal ellipsis
    )
}

/// Scans a closed `[[...]]` wikilink starting at `i`; returns the token and next index.
pub(super) fn scan_wikilink(chars: &[char], i: usize) -> Option<(BodyToken, usize)> {
    let len = chars.len();
    if !(i + 1 < len && chars[i] == WIKILINK_OPEN_CHAR && chars[i + 1] == WIKILINK_OPEN_CHAR) {
        return None;
    }
    let start = i + 2;
    let end = (start..len.saturating_sub(1))
        .find(|&j| chars[j] == WIKILINK_CLOSE_CHAR && chars[j + 1] == WIKILINK_CLOSE_CHAR)?;
    let inner: String = chars[start..end].iter().collect();
    let (trailing, next) = scan_trailing(chars, end + 2);
    let target = inner.trim().to_string();
    Some((BodyToken::Wikilink { target, trailing }, next))
}

/// Scans a `#Tag` hashtag starting at `i`: it must follow whitespace, a mark delimiter or
/// the start of the text.
pub(super) fn scan_hashtag(chars: &[char], i: usize) -> Option<(BodyToken, usize)> {
    let starts_word = i == 0 || chars[i - 1].is_whitespace() || is_mark_char(chars[i - 1]);
    if chars[i] != HASHTAG_MARKER || !starts_word {
        return None;
    }
    let end = (i + 1..chars.len()).find(|&e| !is_hashtag_char(chars, e)).unwrap_or(chars.len());
    if end <= i + 1 {
        return None;
    }
    let tag: String = chars[i..end].iter().collect();
    let (trailing, next) = scan_trailing(chars, end);
    Some((BodyToken::Hashtag { tag, trailing }, next))
}

/// Scans an `http(s)` URL that starts a word at `i` (L1), as written; returns the token and
/// the index after its trailing punctuation.
pub(super) fn scan_url(chars: &[char], i: usize) -> Option<(BodyToken, usize)> {
    let starts_word = i == 0 || !chars[i - 1].is_alphanumeric();
    if !starts_word || !matches!(chars[i], 'h' | 'H') {
        return None;
    }
    let tail: String = chars[i..].iter().take_while(|c| !c.is_whitespace()).collect();
    let url = crate::domain::media::written_url(&tail)?;
    let (trailing, next) = scan_trailing(chars, i + url.chars().count());
    Some((BodyToken::Url { url: url.to_string(), trailing }, next))
}

fn is_mark_char(c: char) -> bool {
    MARK_CHARS.contains(&c) || c == UNDERSCORE
}

/// Whether `chars[e]` continues a hashtag (a doubled underscore ends it).
fn is_hashtag_char(chars: &[char], e: usize) -> bool {
    let c = chars[e];
    let doubled_underscore = c == UNDERSCORE && chars.get(e + 1) == Some(&UNDERSCORE);
    !c.is_whitespace()
        && c != WIKILINK_OPEN_CHAR
        && c != WIKILINK_CLOSE_CHAR
        && !MARK_CHARS.contains(&c)
        && !doubled_underscore
        && !is_trailing_punctuation(c)
}

/// Collects trailing punctuation from `cursor`; returns it and the index after it.
pub(super) fn scan_trailing(chars: &[char], mut cursor: usize) -> (String, usize) {
    let mut trailing = String::new();
    while cursor < chars.len() && is_trailing_punctuation(chars[cursor]) {
        trailing.push(chars[cursor]);
        cursor += 1;
    }
    (trailing, cursor)
}
