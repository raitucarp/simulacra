//! A study session as an immutable state machine: show a prompt, reveal, grade, move on.
//! A card answered Again comes back a few cards later (immediate re-retrieval after a
//! failure, with a short gap so the answer is not still in working memory).

use super::card::CardId;
use super::grade::Grade;

/// Cards shown before a forgotten card returns.
pub const REQUEUE_GAP: usize = 3;
/// Most times one card is re-shown after Again within a session.
pub const MAX_REQUEUES: usize = 2;

/// Whether the answer of the current card is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Stage {
    #[default]
    Prompt,
    Revealed,
}

/// One answer given in the session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionAnswer {
    pub card_id: CardId,
    pub grade: Grade,
}

/// The state of a session.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Session {
    /// Cards still to show; the first is the current one.
    pub queue: Vec<CardId>,
    pub stage: Stage,
    pub answers: Vec<SessionAnswer>,
    /// Distinct cards the session started with.
    pub planned: usize,
}

impl Session {
    pub fn new(queue: Vec<CardId>) -> Self {
        let planned = queue.len();
        Self { queue, stage: Stage::Prompt, answers: Vec::new(), planned }
    }

    pub fn current(&self) -> Option<&CardId> {
        self.queue.first()
    }

    pub fn is_finished(&self) -> bool {
        self.queue.is_empty()
    }

    /// Shows the answer of the current card.
    pub fn reveal(self) -> Self {
        Self { stage: Stage::Revealed, ..self }
    }

    /// Records `grade` for the current card and moves on; Again re-queues it.
    pub fn grade(self, grade: Grade) -> Self {
        let Some(current) = self.current().cloned() else { return self };
        let mut queue: Vec<CardId> = self.queue[1..].to_vec();
        let failures = self.answers.iter().filter(|a| a.card_id == current && a.grade == Grade::Again).count();
        if grade == Grade::Again && failures < MAX_REQUEUES {
            queue.insert(REQUEUE_GAP.min(queue.len()), current.clone());
        }
        let mut answers = self.answers;
        answers.push(SessionAnswer { card_id: current, grade });
        Self { queue, stage: Stage::Prompt, answers, planned: self.planned }
    }

    /// Distinct cards answered so far.
    pub fn done(&self) -> usize {
        let mut seen: Vec<&CardId> = self.answers.iter().map(|a| &a.card_id).collect();
        seen.sort();
        seen.dedup();
        seen.len()
    }

    /// Answers per grade, in [`Grade::ALL`] order.
    pub fn grade_counts(&self) -> [usize; 4] {
        Grade::ALL.map(|grade| self.answers.iter().filter(|a| a.grade == grade).count())
    }

    /// Share of first answers that were recalled (0–1), if any card was answered.
    pub fn first_try_recall(&self) -> Option<f64> {
        let firsts: Vec<&SessionAnswer> =
            self.answers.iter().enumerate().filter(|(i, a)| !self.answers[..*i].iter().any(|b| b.card_id == a.card_id)).map(|(_, a)| a).collect();
        (!firsts.is_empty()).then(|| firsts.iter().filter(|a| a.grade.is_recall()).count() as f64 / firsts.len() as f64)
    }
}
