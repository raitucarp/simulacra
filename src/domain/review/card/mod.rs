//! Flashcards: what is asked ([`CardKind`]), where it comes from ([`CardSource`]), its tags
//! and its schedule.
//!
//! - **Basic:** a question and its answer.
//! - **Cloze:** a text with `{{c1::…}}` deletions; one card per deletion number.
//! - **Concept:** a term and its definition, asked in both directions (two cards).

pub mod cloze;
mod draft;
mod faces;
mod source;

pub use draft::{draft_problem, expand_draft, parse_tags, CardDraft, DraftKind, DraftProblem, NewCardContext};
pub use faces::{card_faces, CardFaces, FacePart};
pub use source::CardSource;

use super::schedule::CardSchedule;
use crate::domain::NetworkId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Prefix of generated card ids.
pub const CARD_ID_PREFIX: &str = "card_";

/// A card's identifier (the `review_cards.id` column).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CardId(String);

impl CardId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// A fresh random id.
    pub fn generate() -> Self {
        Self(format!("{CARD_ID_PREFIX}{}", uuid::Uuid::new_v4().simple()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Which side of a concept card is shown first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConceptDirection {
    /// Shows the term, asks for the definition.
    TermToDefinition,
    /// Shows the definition, asks for the term.
    DefinitionToTerm,
}

impl ConceptDirection {
    pub const ALL: [Self; 2] = [Self::TermToDefinition, Self::DefinitionToTerm];
}

/// What a card asks.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CardKind {
    Basic { front: String, back: String },
    Cloze { text: String, ordinal: u32 },
    Concept { term: String, definition: String, direction: ConceptDirection },
}

impl CardKind {
    /// Stable key of the variant (statistics, locale keys).
    pub const fn key(&self) -> &'static str {
        match self {
            Self::Basic { .. } => "basic",
            Self::Cloze { .. } => "cloze",
            Self::Concept { .. } => "concept",
        }
    }

    /// The answer as one line of text (quiz options and typed-answer checks).
    pub fn answer_text(&self) -> String {
        match self {
            Self::Basic { back, .. } => back.trim().to_string(),
            Self::Cloze { text, ordinal } => cloze::deleted_answer(text, *ordinal).trim().to_string(),
            Self::Concept { term, direction: ConceptDirection::DefinitionToTerm, .. } => term.trim().to_string(),
            Self::Concept { definition, .. } => definition.trim().to_string(),
        }
    }
}

/// One flashcard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub id: CardId,
    pub network_id: NetworkId,
    pub kind: CardKind,
    pub source: CardSource,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub schedule: CardSchedule,
    /// Left out of reviews until resumed.
    pub suspended: bool,
}

impl Card {
    /// Whether the card shares a tag with `other` (quiz distractors prefer these).
    pub fn shares_tag_with(&self, other: &Card) -> bool {
        self.tags.iter().any(|tag| other.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
    }
}
