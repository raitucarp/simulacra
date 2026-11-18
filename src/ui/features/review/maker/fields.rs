//! The pure side of the card maker: the card kinds it offers, the sentences it can start
//! from, and the draft its fields make.

use crate::domain::post::SourceLocator;
use crate::domain::review::card::cloze::{cloze_from_words, cloze_words, plain_text};
use crate::domain::review::{CardSource, DraftKind};
use std::collections::BTreeSet;

/// The kind of card being written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MakerKind {
    Basic,
    #[default]
    Cloze,
    Concept,
}

impl MakerKind {
    pub const ALL: [Self; 3] = [Self::Basic, Self::Cloze, Self::Concept];

    /// Locale key stem (`review.maker.kind.<key>`, `review.maker.<key>.front`, …).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Cloze => "cloze",
            Self::Concept => "concept",
        }
    }

    /// Whether the kind has a second field (answer or definition).
    pub const fn has_back(self) -> bool {
        !matches!(self, Self::Cloze)
    }
}

/// A sentence of the source, with where it sits in a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MakerSentence {
    pub text: String,
    pub locator: Option<SourceLocator>,
}

/// What the maker opens with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MakerRequest {
    pub source: CardSource,
    /// Shown in the header.
    pub title: String,
    pub sentences: Vec<MakerSentence>,
    /// The text an AI draft is asked about.
    pub ai_text: String,
    /// The text the first field starts with.
    pub initial_text: String,
}

/// The words of the cloze field that can be clicked into deletions.
pub fn clickable_words(text: &str) -> Vec<String> {
    cloze_words(&plain_text(text))
}

/// The cloze text: the typed text, or its words with the picked ones turned into deletions.
pub fn cloze_text(text: &str, picked: &BTreeSet<usize>) -> String {
    if picked.is_empty() {
        return text.trim().to_string();
    }
    cloze_from_words(&clickable_words(text), picked)
}

/// The draft the fields make.
pub fn draft_of(kind: MakerKind, front: &str, back: &str, picked: &BTreeSet<usize>) -> DraftKind {
    match kind {
        MakerKind::Basic => DraftKind::Basic { front: front.trim().into(), back: back.trim().into() },
        MakerKind::Cloze => DraftKind::Cloze { text: cloze_text(front, picked) },
        MakerKind::Concept => DraftKind::Concept { term: front.trim().into(), definition: back.trim().into() },
    }
}

/// `source` narrowed to the place of `sentence` (only a document source has places).
pub fn source_at(source: &CardSource, sentence: Option<&MakerSentence>) -> CardSource {
    match (source, sentence.and_then(|s| s.locator.clone())) {
        (CardSource::Document { id, .. }, Some(locator)) => CardSource::document(id.clone(), Some(locator)),
        _ => source.clone(),
    }
}

/// A one-line preview of a draft for the pending list.
pub fn draft_preview(kind: &DraftKind) -> String {
    match kind {
        DraftKind::Basic { front, back } => format!("{front} → {back}"),
        DraftKind::Cloze { text } => plain_text(text),
        DraftKind::Concept { term, definition } => format!("{term} ↔ {definition}"),
    }
}
