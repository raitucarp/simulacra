//! N7: pinned quotes. A post can quote a passage of a vault document verbatim; it keeps the
//! words, where they started (in characters), the document's title and a content hash of the
//! whole document as it was ([`PinnedQuote`], `extras.pinned`). When the document changes, the
//! quote is flagged: still there but the source moved on, or gone from it altogether. Passages
//! are offered as the document's sentences and lines, like a margin reply's.

use super::anchor::{AnchorPlace, SpanAnchor};
use crate::domain::embedding::content_hash;
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// The longest passage a post pins, in characters.
pub const MAX_QUOTE_CHARS: usize = 600;

/// A vault passage quoted verbatim (`extras.pinned`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PinnedQuote {
    /// The vault document quoted.
    pub doc_id: String,
    /// Its title when quoted (shown once the document is gone).
    pub title: String,
    /// The quoted words, as they were.
    pub passage: String,
    /// Where they started in the document, in characters.
    pub start: usize,
    /// The hash of the whole document when quoted.
    pub source_hash: String,
}

/// How a pinned quote stands against its document now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteDrift {
    /// The document has not changed since the quote was pinned.
    Intact,
    /// The document changed, but the passage is still in it word for word.
    SourceChanged,
    /// The document changed and the passage is no longer in it.
    PassageGone,
    /// The document is no longer in the vault.
    SourceMissing,
}

/// The hash a pinned quote keeps of its document's text.
pub fn source_hash(text: &str) -> String {
    content_hash(&[text])
}

impl PinnedQuote {
    /// The passage `range` (characters) of the document `doc_id` titled `title` with text
    /// `text`, trimmed; `None` when it is empty, out of the text or too long.
    pub fn of(doc_id: &str, title: &str, text: &str, range: Range<usize>) -> Option<Self> {
        let picked: Vec<char> = text.chars().collect::<Vec<_>>().get(range.clone())?.to_vec();
        let lead = picked.iter().take_while(|c| c.is_whitespace()).count();
        let passage = picked.iter().collect::<String>().trim().to_string();
        let count = passage.chars().count();
        let quote = Self { doc_id: doc_id.to_string(), title: title.to_string(), passage, start: range.start + lead, source_hash: source_hash(text) };
        (count > 0 && count <= MAX_QUOTE_CHARS).then_some(quote)
    }

    /// How the quote stands against the document's text now (`None`: the document is gone).
    pub fn drift(&self, text: Option<&str>) -> QuoteDrift {
        match text {
            None => QuoteDrift::SourceMissing,
            Some(text) if source_hash(text) == self.source_hash => QuoteDrift::Intact,
            Some(text) if text.contains(self.passage.as_str()) => QuoteDrift::SourceChanged,
            Some(_) => QuoteDrift::PassageGone,
        }
    }

    /// The quote re-pinned to the document's text now: the same words, found again nearest
    /// where they were, with the new hash; `None` when the words are no longer in it.
    #[must_use]
    pub fn repinned(&self, text: &str) -> Option<Self> {
        let anchor = SpanAnchor { quote: self.passage.clone(), start: self.start, version: None };
        match anchor.place(text) {
            AnchorPlace::Exact(range) | AnchorPlace::Moved(range) => Some(Self { start: range.start, source_hash: source_hash(text), ..self.clone() }),
            AnchorPlace::Gone => None,
        }
    }
}

impl QuoteDrift {
    /// Whether the quote should be flagged.
    pub fn flagged(self) -> bool {
        self != Self::Intact
    }

    /// Stable key (locale keys `posts.pinned.drift.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Intact => "intact",
            Self::SourceChanged => "changed",
            Self::PassageGone => "gone",
            Self::SourceMissing => "missing",
        }
    }
}

#[cfg(test)]
mod tests;
