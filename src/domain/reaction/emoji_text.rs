//! What may be stored as a reaction's emoji: one short run of pictographic text, never words.

/// Longest emoji sequence accepted, in characters (family and couple sequences with skin
/// tones run to ten; flags of subdivisions to seven).
pub const MAX_EMOJI_CHARS: usize = 16;

/// `text` trimmed, when it can be a reaction's emoji: not empty, at most [`MAX_EMOJI_CHARS`]
/// characters, and without spaces, ASCII letters or ASCII punctuation (keycaps such as `1️⃣`,
/// which start with an ASCII digit or `#`/`*` followed by U+FE0F, are allowed).
pub fn normalize_emoji(text: &str) -> Option<String> {
    let text = text.trim();
    let count = text.chars().count();
    if count == 0 || count > MAX_EMOJI_CHARS {
        return None;
    }
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        let keycap_base = matches!(c, '0'..='9' | '#' | '*') && chars.peek() == Some(&'\u{FE0F}');
        if c.is_whitespace() || (c.is_ascii() && !keycap_base) {
            return None;
        }
    }
    Some(text.to_string())
}
