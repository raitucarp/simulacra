//! The review log: one entry per answer, kept for statistics, daily limits and calibration.

use super::item::{ItemId, ReviewItem};
use super::grade::{Confidence, Grade};
use super::schedule::{review, Phase, Policy};
use crate::domain::NetworkId;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// How the card was practised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewMode {
    /// A session review: recall, reveal, self-grade (wire key kept from the flashcards).
    #[serde(rename = "flashcard")]
    Review,
    /// Pick the answer among distractors.
    Choice,
    /// Type the answer, compare, self-grade.
    Recall,
    /// Draw a connection map, graded by how exactly it matches.
    Connections,
    /// Walk a memory palace: recall what is placed at each locus, reveal, self-grade (sb E2).
    Palace,
}

impl ReviewMode {
    pub const ALL: [Self; 5] = [Self::Review, Self::Choice, Self::Recall, Self::Connections, Self::Palace];

    /// Stable wire key. Never rename one.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Review => "flashcard",
            Self::Choice => "choice",
            Self::Recall => "recall",
            Self::Connections => "connections",
            Self::Palace => "palace",
        }
    }
}

/// What the learner answered, before it is applied to a item.
#[derive(Debug, Clone, PartialEq)]
pub struct Answer {
    pub item_id: ItemId,
    pub grade: Grade,
    pub mode: ReviewMode,
    pub confidence: Option<Confidence>,
    /// Whether an objectively checked answer was right (quizzes only).
    pub correct: Option<bool>,
}

/// One logged review.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewEntry {
    #[serde(alias = "card_id")]
    pub item_id: ItemId,
    pub network_id: NetworkId,
    pub reviewed_at: DateTime<Utc>,
    pub day: NaiveDate,
    pub grade: Grade,
    pub mode: ReviewMode,
    #[serde(default)]
    pub confidence: Option<Confidence>,
    #[serde(default)]
    pub correct: Option<bool>,
    /// The card's phase before this review.
    pub phase_before: Phase,
    /// Days given until the next review.
    pub interval: u32,
    pub stability: f64,
    pub difficulty: f64,
    /// Predicted probability of recall when answered (none for a new card).
    #[serde(default)]
    pub retrievability: Option<f64>,
}

impl ReviewEntry {
    /// Whether this was a review of a learnt card (the base of true retention).
    pub fn is_mature_review(&self) -> bool {
        matches!(self.phase_before, Phase::Review)
    }
}

/// `item` after `answer` on `today`, with its log entry.
pub fn apply_answer(item: &ReviewItem, answer: &Answer, now: DateTime<Utc>, today: NaiveDate, policy: &Policy) -> (ReviewItem, ReviewEntry) {
    let schedule = review(&item.schedule, answer.grade, today, policy);
    let memory = schedule.memory.unwrap_or(super::fsrs::Memory { stability: 0.0, difficulty: 0.0 });
    let entry = ReviewEntry {
        item_id: item.id.clone(),
        network_id: item.network_id.clone(),
        reviewed_at: now,
        day: today,
        grade: answer.grade,
        mode: answer.mode,
        confidence: answer.confidence,
        correct: answer.correct,
        phase_before: item.schedule.phase,
        interval: schedule.interval,
        stability: memory.stability,
        difficulty: memory.difficulty,
        retrievability: item.schedule.retrievability(today, &policy.params),
    };
    (ReviewItem { schedule, ..item.clone() }, entry)
}
