//! The tutor session as an immutable state machine. Every transition takes the session and
//! returns the next one; a transition that does not fit the current phase leaves it as it was.
//!
//! `start` → opening question → (answer → assessment + next question)* → closing → summary.

use super::progress::{current_level, misses_in_a_row, next_level, next_move, should_close};
use super::reply::TurnReply;
use super::summary::TutorSummary;
use super::{Assessment, Exchange, SessionId, SocraticMove, TutorPhase, TutorQuestion, TutorSession, TutorSource};
use crate::domain::learning::bloom::BloomLevel;
use crate::domain::NetworkId;
use jiff::Timestamp;

/// Longest answer kept, in characters.
pub const MAX_ANSWER_CHARS: usize = 2_000;

/// What a new session is about.
#[derive(Debug, Clone, PartialEq)]
pub struct SessionStart {
    pub network_id: NetworkId,
    pub source: TutorSource,
    pub title: String,
    pub material: String,
}

impl TutorSession {
    /// A new session waiting for its opening question.
    pub fn start(id: SessionId, start: SessionStart, now: Timestamp) -> Self {
        let SessionStart { network_id, source, title, material } = start;
        Self { id, network_id, source, title, material, exchanges: Vec::new(), closing: false, summary: None, items_created: 0, created_at: now, updated_at: now }
    }

    fn touched(self, now: Timestamp) -> Self {
        Self { updated_at: now, ..self }
    }

    /// The opening question: it elicits the learner's model at the bottom rung.
    pub fn with_opening(self, question: &str, now: Timestamp) -> Self {
        if self.phase() != TutorPhase::NeedsQuestion {
            return self;
        }
        let opening = TutorQuestion { text: question.trim().to_string(), level: BloomLevel::LOWEST, kind: SocraticMove::Elicit };
        let mut exchanges = self.exchanges.clone();
        exchanges.push(Exchange { question: opening, answer: None, assessment: None });
        Self { exchanges, ..self }.touched(now)
    }

    /// The learner's answer to the open question (blank answers are ignored).
    pub fn with_answer(self, answer: &str, now: Timestamp) -> Self {
        let text: String = answer.trim().chars().take(MAX_ANSWER_CHARS).collect();
        if self.phase() != TutorPhase::AwaitingAnswer || text.is_empty() {
            return self;
        }
        let mut exchanges = self.exchanges.clone();
        if let Some(last) = exchanges.last_mut() {
            last.answer = Some(text);
        }
        Self { exchanges, ..self }.touched(now)
    }

    /// The assessment of the last answer and the next question, placed by the level rules;
    /// the session closes instead when it has done its work.
    pub fn with_turn(self, reply: TurnReply, now: Timestamp) -> Self {
        if self.phase() != TutorPhase::NeedsAssessment {
            return self;
        }
        let assessment = reply.assessment.unwrap_or_else(Assessment::unclear);
        let mut exchanges = self.exchanges.clone();
        let current = current_level(&exchanges);
        if let Some(last) = exchanges.last_mut() {
            last.assessment = Some(assessment.clone());
        }
        if should_close(&exchanges) {
            return Self { exchanges, closing: true, ..self }.touched(now);
        }
        let level = next_level(current, &assessment, misses_in_a_row(&exchanges, current));
        let kind = exchanges.last().map_or(SocraticMove::for_level(level), |previous| next_move(level, previous, &assessment));
        exchanges.push(Exchange { question: TutorQuestion { text: reply.question.trim().to_string(), level, kind }, answer: None, assessment: None });
        Self { exchanges, ..self }.touched(now)
    }

    /// Closes the dialogue: an unanswered question is dropped and the summary is due. A
    /// session without a single answer has nothing to sum up and stays as it is.
    pub fn close(self, now: Timestamp) -> Self {
        let exchanges: Vec<Exchange> = self.exchanges.iter().filter(|exchange| exchange.answer.is_some()).cloned().collect();
        if self.summary.is_some() || exchanges.is_empty() {
            return self;
        }
        Self { exchanges, closing: true, ..self }.touched(now)
    }

    /// The closing summary.
    pub fn with_summary(self, summary: TutorSummary, now: Timestamp) -> Self {
        if self.phase() != TutorPhase::NeedsSummary {
            return self;
        }
        Self { summary: Some(summary), ..self }.touched(now)
    }

    /// `count` review items were made from the summary's gaps.
    pub fn with_items_created(self, count: usize, now: Timestamp) -> Self {
        Self { items_created: self.items_created + count, ..self }.touched(now)
    }
}
