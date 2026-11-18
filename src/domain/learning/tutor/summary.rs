//! The closing summary: what the learner showed, what was missing, and review questions for
//! the gaps. The questions become review items anchored to the dialogue's source (the source
//! is the answer; the learner's question is the prompt), never free-floating cards.

use super::json::{list_field, object_in};
use super::reply::{checked_question, ReplyProblem, MAX_POINTS, MAX_POINT_CHARS};
use super::TutorSession;
use crate::domain::learning::bloom::BloomLevel;
use crate::domain::review::{AnchorSide, ItemDraft};
use serde::{Deserialize, Serialize};

/// Review questions offered at most.
pub const MAX_REVIEW_QUESTIONS: usize = 5;

/// What a dialogue showed and missed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TutorSummary {
    pub shown: Vec<String>,
    pub missing: Vec<String>,
    /// Questions for review items on the gaps.
    pub review: Vec<String>,
    /// The highest level shown in the dialogue.
    pub mastery: Option<BloomLevel>,
}

/// Parses the summary reply (a JSON object with `shown`, `missing` and `review`); review
/// entries that are not questions are dropped.
pub fn parse_summary(text: &str, mastery: Option<BloomLevel>) -> Result<TutorSummary, ReplyProblem> {
    let value = object_in(text).ok_or(ReplyProblem::Unreadable)?;
    let shown = list_field(&value, "shown", MAX_POINTS, MAX_POINT_CHARS);
    let missing = list_field(&value, "missing", MAX_POINTS, MAX_POINT_CHARS);
    let review: Vec<String> = list_field(&value, "review", MAX_REVIEW_QUESTIONS, MAX_POINT_CHARS).iter().filter_map(|q| checked_question(q).ok()).collect();
    if shown.is_empty() && missing.is_empty() && review.is_empty() {
        return Err(ReplyProblem::Unreadable);
    }
    Ok(TutorSummary { shown, missing, review, mastery })
}

/// Drafts of the review items for the chosen questions (indexes into `summary.review`),
/// anchored to the session's source with the source as the answer. None for a typed concept.
pub fn gap_drafts(session: &TutorSession, chosen: &[usize]) -> Vec<ItemDraft> {
    let (Some(summary), Some(anchor)) = (session.summary.as_ref(), session.source.review_anchor(&session.material)) else { return Vec::new() };
    chosen
        .iter()
        .filter_map(|index| summary.review.get(*index))
        .map(|question| ItemDraft { anchor: anchor.clone(), side: AnchorSide::Back, written: question.clone(), tags: Vec::new() })
        .collect()
}
