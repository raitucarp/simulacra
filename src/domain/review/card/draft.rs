//! A card as the learner writes it, its problems, and the cards it becomes (one per cloze
//! deletion, two per concept).

use super::cloze::cloze_ordinals;
use super::{Card, CardId, CardKind, CardSource, ConceptDirection};
use crate::domain::review::schedule::CardSchedule;
use crate::domain::NetworkId;
use chrono::{DateTime, NaiveDate, Utc};

/// What the learner wrote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DraftKind {
    Basic { front: String, back: String },
    Cloze { text: String },
    Concept { term: String, definition: String },
}

/// A written card with its source and tags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardDraft {
    pub kind: DraftKind,
    pub source: CardSource,
    pub tags: Vec<String>,
}

/// Why a draft cannot be saved yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftProblem {
    MissingPrompt,
    MissingAnswer,
    /// A cloze text without any `{{c1::…}}` deletion.
    NoDeletion,
}

impl DraftProblem {
    /// Locale key of the message.
    pub const fn message_key(self) -> &'static str {
        match self {
            Self::MissingPrompt => "review.maker.problem_prompt",
            Self::MissingAnswer => "review.maker.problem_answer",
            Self::NoDeletion => "review.maker.problem_cloze",
        }
    }
}

/// The first problem of `kind`, if any.
pub fn draft_problem(kind: &DraftKind) -> Option<DraftProblem> {
    let blank = |text: &str| text.trim().is_empty();
    match kind {
        DraftKind::Basic { front, .. } | DraftKind::Concept { term: front, .. } if blank(front) => Some(DraftProblem::MissingPrompt),
        DraftKind::Basic { back, .. } | DraftKind::Concept { definition: back, .. } if blank(back) => Some(DraftProblem::MissingAnswer),
        DraftKind::Cloze { text } if blank(text) => Some(DraftProblem::MissingPrompt),
        DraftKind::Cloze { text } if cloze_ordinals(text).is_empty() => Some(DraftProblem::NoDeletion),
        _ => None,
    }
}

/// The network, the moment and the review day new cards are created in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewCardContext {
    pub network_id: NetworkId,
    pub created_at: DateTime<Utc>,
    pub today: NaiveDate,
}

/// The card kinds a draft becomes.
fn kinds_of(kind: &DraftKind) -> Vec<CardKind> {
    match kind {
        DraftKind::Basic { front, back } => vec![CardKind::Basic { front: front.trim().into(), back: back.trim().into() }],
        DraftKind::Cloze { text } => {
            cloze_ordinals(text).into_iter().map(|ordinal| CardKind::Cloze { text: text.trim().into(), ordinal }).collect()
        }
        DraftKind::Concept { term, definition } => ConceptDirection::ALL
            .into_iter()
            .map(|direction| CardKind::Concept { term: term.trim().into(), definition: definition.trim().into(), direction })
            .collect(),
    }
}

/// The new cards of `draft` (none while it has a problem), ids taken from `next_id`.
pub fn expand_draft(draft: &CardDraft, context: &NewCardContext, mut next_id: impl FnMut() -> CardId) -> Vec<Card> {
    if draft_problem(&draft.kind).is_some() {
        return Vec::new();
    }
    kinds_of(&draft.kind)
        .into_iter()
        .map(|kind| Card {
            id: next_id(),
            network_id: context.network_id.clone(),
            kind,
            source: draft.source.clone(),
            tags: draft.tags.clone(),
            created_at: context.created_at,
            schedule: CardSchedule::new_card(context.today),
            suspended: false,
        })
        .collect()
}

/// Tags typed as a comma-separated list: trimmed, without a leading `#`, deduplicated.
pub fn parse_tags(text: &str) -> Vec<String> {
    text.split(',').map(|tag| tag.trim().trim_start_matches('#').to_string()).filter(|tag| !tag.is_empty()).fold(Vec::new(), |mut tags, tag| {
        if !tags.iter().any(|t: &String| t.eq_ignore_ascii_case(&tag)) {
            tags.push(tag);
        }
        tags
    })
}
