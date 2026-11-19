//! The pure side of the learn sheet: what it opens on (a post, or a vault document with its
//! sentences and paragraphs), and the item draft its fields make.

use super::super::data::PostView;
use crate::domain::post::SourceLocator;
use crate::domain::review::{Anchor, AnchorSide, ItemDraft, ItemId};

/// A passage of a document, with where it sits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passage {
    pub text: String,
    pub locator: Option<SourceLocator>,
}

/// Whether passages are picked by sentence or by paragraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Granularity {
    #[default]
    Sentence,
    Paragraph,
}

impl Granularity {
    pub const ALL: [Self; 2] = [Self::Sentence, Self::Paragraph];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Sentence => "sentences",
            Self::Paragraph => "paragraphs",
        }
    }
}

/// What the sheet learns from.
#[derive(Debug, Clone, PartialEq)]
pub enum LearnSource {
    Post(PostView),
    Document { id: String, title: String, sentences: Vec<Passage>, paragraphs: Vec<Passage> },
}

/// What the sheet opens with.
#[derive(Debug, Clone, PartialEq)]
pub struct LearnRequest {
    pub source: LearnSource,
    /// The item being edited (none for a new one).
    pub existing: Option<ItemId>,
    pub side: AnchorSide,
    pub written: String,
    pub tags: Vec<String>,
    /// The passage an edited item is anchored to.
    pub passage: Option<Passage>,
}

impl LearnRequest {
    /// A fresh request on `source`.
    pub fn new(source: LearnSource, side: AnchorSide) -> Self {
        Self { source, existing: None, side, written: String::new(), tags: Vec::new(), passage: None }
    }

    /// The passages offered at `granularity` (none for a post).
    pub fn passages(&self, granularity: Granularity) -> &[Passage] {
        match (&self.source, granularity) {
            (LearnSource::Document { sentences, .. }, Granularity::Sentence) => sentences,
            (LearnSource::Document { paragraphs, .. }, Granularity::Paragraph) => paragraphs,
            (LearnSource::Post(_), _) => &[],
        }
    }
}

/// The anchor of the fields: the post, or the chosen passage of the document.
pub fn anchor_of(source: &LearnSource, passage: Option<&Passage>) -> Option<Anchor> {
    match (source, passage) {
        (LearnSource::Post(view), _) => Some(Anchor::post(&view.post.id)),
        (LearnSource::Document { id, .. }, Some(passage)) => Some(Anchor::passage(id.clone(), passage.locator.clone(), passage.text.clone())),
        (LearnSource::Document { .. }, None) => None,
    }
}

/// The draft of the fields, if a passage is chosen where one is needed.
pub fn draft_of(source: &LearnSource, passage: Option<&Passage>, side: AnchorSide, written: &str, tags: Vec<String>) -> Option<ItemDraft> {
    anchor_of(source, passage).map(|anchor| ItemDraft { anchor, side, written: written.trim().to_string(), tags })
}

/// The text an AI suggestion is asked about.
pub fn suggestion_text(source: &LearnSource, passage: Option<&Passage>) -> Option<String> {
    match (source, passage) {
        (LearnSource::Post(view), _) => Some(view.post.content.clone()),
        (_, Some(passage)) => Some(passage.text.clone()),
        _ => None,
    }
}
