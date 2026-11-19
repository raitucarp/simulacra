//! The Socratic tutor: a dialogue that asks and never lectures. It elicits the learner's
//! model, probes definitions, asks for examples and counterexamples, exposes contradictions
//! and climbs Bloom's levels; each answer is assessed and the next question adapts.
//!
//! - [`source`]: what the dialogue is about (a post, a thread, a passage, a document, a concept).
//! - [`moves`]: the Socratic moves a question makes.
//! - [`progress`]: the level and move rules (climb, stay, step down; when to close).
//! - [`machine`]: the session as an immutable state machine.
//! - [`prompt`] / [`transcript`]: the requests; [`reply`] / [`summary`]: their parsing.

mod exchange;
mod json;
pub mod machine;
pub mod moves;
pub mod progress;
pub mod prompt;
pub mod reply;
pub mod source;
pub mod summary;
pub mod transcript;

pub use exchange::{Assessment, Exchange, TutorQuestion, Verdict};
pub use moves::SocraticMove;
pub use reply::{parse_turn_reply, ReplyProblem, TurnReply};
pub use source::TutorSource;
pub use summary::{gap_drafts, parse_summary, TutorSummary};

use super::bloom::BloomLevel;
use crate::domain::NetworkId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Prefix of tutor session ids.
pub const SESSION_ID_PREFIX: &str = "tutor";

/// A tutor session's identifier (the `learning_tutor_sessions.id` column).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(pub String);

impl SessionId {
    pub fn generate() -> Self {
        Self(super::fresh_id(SESSION_ID_PREFIX))
    }
}

/// One tutoring dialogue (persisted, resumable).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TutorSession {
    pub id: SessionId,
    pub network_id: NetworkId,
    pub source: TutorSource,
    /// What the dialogue is about, as listed.
    pub title: String,
    /// The source's text as the tutor reads it (quoted data).
    pub material: String,
    pub exchanges: Vec<Exchange>,
    /// The learner (or the rules) asked to close: the summary is due.
    #[serde(default)]
    pub closing: bool,
    #[serde(default)]
    pub summary: Option<TutorSummary>,
    /// Review items made from the summary's gaps.
    #[serde(default)]
    pub items_created: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// What a session waits for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TutorPhase {
    /// The opening question (a request).
    NeedsQuestion,
    /// The learner's answer.
    AwaitingAnswer,
    /// The assessment and the next question (a request).
    NeedsAssessment,
    /// The closing summary (a request).
    NeedsSummary,
    Finished,
}

impl TutorSession {
    /// What the session waits for now.
    pub fn phase(&self) -> TutorPhase {
        let last = self.exchanges.last();
        match (self.summary.is_some(), self.closing, last) {
            (true, _, _) => TutorPhase::Finished,
            (false, true, _) => TutorPhase::NeedsSummary,
            (false, false, None) => TutorPhase::NeedsQuestion,
            (false, false, Some(exchange)) if exchange.answer.is_none() => TutorPhase::AwaitingAnswer,
            (false, false, Some(_)) => TutorPhase::NeedsAssessment,
        }
    }

    /// Whether the next step is a request to the model.
    pub fn needs_request(&self) -> bool {
        matches!(self.phase(), TutorPhase::NeedsQuestion | TutorPhase::NeedsAssessment | TutorPhase::NeedsSummary)
    }

    /// Answers given so far.
    pub fn answered(&self) -> usize {
        self.exchanges.iter().filter(|exchange| exchange.answer.is_some()).count()
    }

    /// The highest level shown so far (see [`progress::mastery`]).
    pub fn mastery(&self) -> Option<BloomLevel> {
        progress::mastery(&self.exchanges)
    }
}
