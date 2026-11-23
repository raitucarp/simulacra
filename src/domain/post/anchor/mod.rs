//! N2: margin replies answer one span of the post they reply to. The reply keeps the span as
//! a [`SpanAnchor`]: the quoted words, where they started (in characters) and the version of
//! the post they were quoted from. When the post is edited the span is found again where it
//! moved, or reported gone. Spans are offered as passages: the post's sentences and lines.
//! A locked post offers none, so its sealed text is never quoted.

use crate::domain::data::history::SemVer;
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// The longest span a margin reply quotes, in characters.
pub const MAX_ANCHOR_CHARS: usize = 280;
/// Characters that end a sentence (Latin and CJK).
const SENTENCE_ENDS: [char; 7] = ['.', '!', '?', '…', '。', '！', '？'];

/// The span of the parent a margin reply answers (`extras.anchor`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SpanAnchor {
    /// The quoted words, as they were.
    pub quote: String,
    /// Where they started in the parent's text, in characters.
    pub start: usize,
    /// The version of the parent they were quoted from (`None`: unknown).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<SemVer>,
}

/// Where an anchor's span is in the parent's text now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnchorPlace {
    /// Still where it was quoted (character range).
    Exact(Range<usize>),
    /// Found elsewhere (the occurrence nearest the old place).
    Moved(Range<usize>),
    /// The words are no longer in the text.
    Gone,
}

impl SpanAnchor {
    /// The span `range` (characters) of `text`, trimmed; `None` when it is empty, out of the
    /// text or longer than [`MAX_ANCHOR_CHARS`].
    pub fn of(text: &str, range: Range<usize>, version: Option<SemVer>) -> Option<Self> {
        let chars: Vec<char> = text.chars().collect();
        let picked = chars.get(range.clone())?;
        let lead = picked.iter().take_while(|c| c.is_whitespace()).count();
        let quote: String = picked.iter().collect::<String>().trim().to_string();
        let count = quote.chars().count();
        (count > 0 && count <= MAX_ANCHOR_CHARS).then(|| Self { quote, start: range.start + lead, version })
    }

    /// Where the span is in `text` now.
    pub fn place(&self, text: &str) -> AnchorPlace {
        let chars: Vec<char> = text.chars().collect();
        let quote: Vec<char> = self.quote.chars().collect();
        let len = quote.len();
        if len == 0 || len > chars.len() {
            return AnchorPlace::Gone;
        }
        if chars.get(self.start..self.start + len) == Some(quote.as_slice()) {
            return AnchorPlace::Exact(self.start..self.start + len);
        }
        let found = (0..=chars.len() - len).filter(|&at| chars[at..at + len] == quote[..]).min_by_key(|at| at.abs_diff(self.start));
        found.map_or(AnchorPlace::Gone, |at| AnchorPlace::Moved(at..at + len))
    }
}

/// The passages of `text` a margin reply can answer: each sentence and each line, trimmed,
/// in order, as character ranges.
pub fn passages(text: &str) -> Vec<Range<usize>> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut start = 0;
    for (at, c) in chars.iter().enumerate() {
        let ends = *c == '\n' || (SENTENCE_ENDS.contains(c) && chars.get(at + 1).is_none_or(|next| next.is_whitespace() || !next.is_ascii()));
        if ends {
            push_trimmed(&chars, start..at + usize::from(*c != '\n'), &mut out);
            start = at + 1;
        }
    }
    push_trimmed(&chars, start..chars.len(), &mut out);
    out
}

fn push_trimmed(chars: &[char], range: Range<usize>, out: &mut Vec<Range<usize>>) {
    let slice = &chars[range.clone()];
    let lead = slice.iter().take_while(|c| c.is_whitespace()).count();
    let tail = slice.iter().rev().take_while(|c| c.is_whitespace()).count();
    if lead < slice.len() {
        out.push(range.start + lead..range.end - tail);
    }
}

/// The characters `range` of `text`.
pub fn span_text(text: &str, range: Range<usize>) -> String {
    text.chars().skip(range.start).take(range.len()).collect()
}

#[cfg(test)]
mod tests;
