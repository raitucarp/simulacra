//! What the Review page asks the shell to do (the shell owns the deck and the vault).

use crate::domain::review::quiz::{ChoiceQuestion, OrderPuzzle, QuizKind};
use crate::domain::review::{Answer, CardId, CardSource};

/// A request from the Review page.
#[derive(Debug, Clone, PartialEq)]
pub enum ReviewEvent {
    /// Apply an answer to its card, log it and save both.
    Answered(Answer),
    /// Open the post or vault document a card cites.
    OpenSource(CardSource),
    /// Build the content of a quiz (the shell has the posts, documents and vectors).
    RequestQuiz(QuizKind),
    /// Open the card maker for a fresh card.
    NewCard,
    SetSuspended(CardId, bool),
    DeleteCard(CardId),
}

/// A quiz the shell prepared.
#[derive(Debug, Clone, PartialEq)]
pub enum QuizContent {
    Choice(Vec<ChoiceQuestion>),
    Recall(Vec<CardId>),
    Order(OrderPuzzle),
    /// Not enough material (the locale key of the reason).
    Unavailable(&'static str),
}
