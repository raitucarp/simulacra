//! A quiz in progress, as immutable state: multiple choice (confidence, pick, next) and free
//! recall (type, check, self-grade, next). Each answer becomes an [`Answer`] for the deck.

use super::choice::ChoiceQuestion;
use super::recall::{judge_recall, RecallVerdict};
use crate::domain::review::log::{Answer, ReviewMode};
use crate::domain::review::{CardId, Confidence, Grade};

/// A multiple-choice round.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChoiceRun {
    pub questions: Vec<ChoiceQuestion>,
    pub index: usize,
    /// Confidence rated before answering (optional).
    pub confidence: Option<Confidence>,
    /// The option picked for the current question.
    pub picked: Option<usize>,
    pub right: usize,
}

impl ChoiceRun {
    pub fn new(questions: Vec<ChoiceQuestion>) -> Self {
        Self { questions, ..Self::default() }
    }

    pub fn current(&self) -> Option<&ChoiceQuestion> {
        self.questions.get(self.index)
    }

    pub fn is_finished(&self) -> bool {
        self.index >= self.questions.len()
    }

    pub fn with_confidence(self, confidence: Confidence) -> Self {
        if self.picked.is_some() { self } else { Self { confidence: Some(confidence), ..self } }
    }

    /// Picks option `option`; the answer to record, if this was the first pick.
    pub fn pick(self, option: usize) -> (Self, Option<Answer>) {
        let Some(question) = self.current().filter(|q| self.picked.is_none() && option < q.options.len()) else { return (self, None) };
        let correct = option == question.correct;
        let answer = Answer {
            card_id: question.card_id.clone(),
            grade: Confidence::grade_for(self.confidence, correct),
            mode: ReviewMode::Choice,
            confidence: self.confidence,
            correct: Some(correct),
        };
        let right = self.right + usize::from(correct);
        (Self { picked: Some(option), right, ..self }, Some(answer))
    }

    /// The next question (only once the current one is answered).
    pub fn next(self) -> Self {
        if self.picked.is_none() {
            return self;
        }
        Self { index: self.index + 1, picked: None, confidence: None, ..self }
    }
}

/// A free-recall round.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RecallRun {
    pub cards: Vec<CardId>,
    pub index: usize,
    /// The typed answer once checked, with its verdict.
    pub checked: Option<(String, RecallVerdict)>,
    pub grades: Vec<Grade>,
}

impl RecallRun {
    pub fn new(cards: Vec<CardId>) -> Self {
        Self { cards, ..Self::default() }
    }

    pub fn current(&self) -> Option<&CardId> {
        self.cards.get(self.index)
    }

    pub fn is_finished(&self) -> bool {
        self.index >= self.cards.len()
    }

    /// Compares `typed` with `expected`.
    pub fn check(self, typed: &str, expected: &str) -> Self {
        if self.checked.is_some() || self.is_finished() {
            return self;
        }
        Self { checked: Some((typed.trim().to_string(), judge_recall(expected, typed))), ..self }
    }

    /// The learner's own grade after checking; the answer to record.
    pub fn grade(self, grade: Grade) -> (Self, Option<Answer>) {
        let (Some(card_id), Some(verdict)) = (self.current().cloned(), self.checked.as_ref().map(|(_, verdict)| *verdict)) else { return (self, None) };
        let answer = Answer { card_id, grade, mode: ReviewMode::Recall, confidence: None, correct: Some(verdict != RecallVerdict::Wrong) };
        let mut grades = self.grades;
        grades.push(grade);
        (Self { index: self.index + 1, checked: None, grades, ..self }, Some(answer))
    }
}
