//! Tokenizer splitting post content into plain text, wikilinks, and hashtags.

use super::consts::{
    HASHTAG_MARKER, LINE_BREAK, WIKILINK_CLOSE_CHAR, WIKILINK_OPEN, WIKILINK_OPEN_CHAR,
};

/// One inline segment of a post body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BodyToken {
    /// Plain text run (may contain line breaks).
    Text(String),
    /// Obsidian `[[target]]` link followed by optional trailing punctuation.
    Wikilink { target: String, trailing: String },
    /// `#Tag` followed by optional trailing punctuation.
    Hashtag { tag: String, trailing: String },
}

/// Returns true if the character is an ASCII or CJK trailing punctuation mark.
#[inline]
pub(crate) fn is_trailing_punctuation(c: char) -> bool {
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

/// True when the content needs tokenized rendering (links, hashtags, or line breaks).
pub(crate) fn has_rich_markup(content: &str) -> bool {
    content.contains(WIKILINK_OPEN) || content.contains(HASHTAG_MARKER) || content.contains(LINE_BREAK)
}

/// Splits post content into ordered inline tokens.
pub(crate) fn tokenize_post_body(content: &str) -> Vec<BodyToken> {
    let chars: Vec<char> = content.chars().collect();
    let mut tokens = Vec::new();
    let mut text = String::new();
    let mut i = 0;
    while i < chars.len() {
        match scan_wikilink(&chars, i).or_else(|| scan_hashtag(&chars, i)) {
            Some((token, next)) => {
                flush_text(&mut text, &mut tokens);
                tokens.push(token);
                i = next;
            }
            None => {
                text.push(chars[i]);
                i += 1;
            }
        }
    }
    flush_text(&mut text, &mut tokens);
    tokens
}

fn flush_text(text: &mut String, tokens: &mut Vec<BodyToken>) {
    if !text.is_empty() {
        tokens.push(BodyToken::Text(std::mem::take(text)));
    }
}

/// Scans a closed `[[...]]` wikilink starting at `i`; returns the token and next index.
fn scan_wikilink(chars: &[char], i: usize) -> Option<(BodyToken, usize)> {
    let len = chars.len();
    if !(i + 1 < len && chars[i] == WIKILINK_OPEN_CHAR && chars[i + 1] == WIKILINK_OPEN_CHAR) {
        return None;
    }
    let start = i + 2;
    let end = (start..len - 1)
        .find(|&j| chars[j] == WIKILINK_CLOSE_CHAR && chars[j + 1] == WIKILINK_CLOSE_CHAR)?;
    let inner: String = chars[start..end].iter().collect();
    let (trailing, next) = scan_trailing(chars, end + 2);
    let target = inner.trim().to_string();
    Some((BodyToken::Wikilink { target, trailing }, next))
}

/// Scans a `#Tag` hashtag starting at `i` (must follow whitespace or start of text).
fn scan_hashtag(chars: &[char], i: usize) -> Option<(BodyToken, usize)> {
    if chars[i] != HASHTAG_MARKER || (i > 0 && !chars[i - 1].is_whitespace()) {
        return None;
    }
    let end = (i + 1..chars.len())
        .find(|&e| !is_hashtag_char(chars[e]))
        .unwrap_or(chars.len());
    if end <= i + 1 {
        return None;
    }
    let tag: String = chars[i..end].iter().collect();
    let (trailing, next) = scan_trailing(chars, end);
    Some((BodyToken::Hashtag { tag, trailing }, next))
}

fn is_hashtag_char(c: char) -> bool {
    !c.is_whitespace()
        && c != WIKILINK_OPEN_CHAR
        && c != WIKILINK_CLOSE_CHAR
        && !is_trailing_punctuation(c)
}

/// Collects trailing punctuation from `cursor`; returns it and the index after it.
fn scan_trailing(chars: &[char], mut cursor: usize) -> (String, usize) {
    let mut trailing = String::new();
    while cursor < chars.len() && is_trailing_punctuation(chars[cursor]) {
        trailing.push(chars[cursor]);
        cursor += 1;
    }
    (trailing, cursor)
}
