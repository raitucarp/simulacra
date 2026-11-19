//! What the Review page asks the shell to do (the shell owns the deck, the posts and the
//! vault).

use crate::domain::review::connections::ConnectionMap;
use crate::domain::review::quiz::{ChoiceQuestion, OrderPuzzle, QuizKind};
use crate::domain::review::{Anchor, AnchorSide, Answer, ItemId};
use crate::domain::PostId;
use crate::ui::features::review::learning::LearningEvent;

/// What the post picker is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickerMode {
    /// Pick a post, then write its question or answer.
    Learn,
    /// Pick several posts for a connection map.
    Map,
    /// Pick a post (or its thread) for the tutor (sb E2).
    Tutor,
    /// Pick a post to place at a palace locus (sb E2).
    Place,
    /// Pick a post or thread to link to a goal (sb E2).
    Link,
}

/// A request from the Review page.
#[derive(Debug, Clone, PartialEq)]
pub enum ReviewEvent {
    /// Apply an answer to its item, log it and save both.
    Answered(Answer),
    /// Open the post or vault document an item is anchored to.
    OpenAnchor(Anchor),
    /// Build the content of a quiz (the shell has the posts, documents and vectors).
    RequestQuiz(QuizKind),
    /// Open the post picker (the shell hands it the stream's posts).
    OpenPicker(PickerMode),
    /// Learn post: write its question (`Back`) or its answer (`Front`).
    LearnPost(PostId, AnchorSide),
    /// Edit an anchored item in the learn sheet.
    EditItem(ItemId),
    /// Seed a connection map from these posts and open it in the editor.
    MapFromPosts(Vec<PostId>),
    /// Save a connection map (a new one when `id` is none).
    SaveMap { id: Option<ItemId>, map: ConnectionMap },
    SetSuspended(ItemId, bool),
    DeleteItem(ItemId),
    /// A request of the Learn tab (sb E2).
    Learning(LearningEvent),
}

/// A quiz the shell prepared.
#[derive(Debug, Clone, PartialEq)]
pub enum QuizContent {
    Choice(Vec<ChoiceQuestion>),
    Recall(Vec<ItemId>),
    Order(OrderPuzzle),
    /// Not enough material (the locale key of the reason).
    Unavailable(&'static str),
}
