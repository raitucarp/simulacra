//! N18: open questions. A post of the [`PostKind::Question`] kind stays open until the user
//! marks it answered ([`Answered`], `extras.answered`), with the post that answers it when
//! there is one; it can be reopened. A page lists every question still open. A locked post is
//! never made a question, and every list shows a locked one only as its placeholder.

use crate::domain::post::PostKind;
use crate::domain::{Post, PostId};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// A question marked answered (`extras.answered`, boxed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Answered {
    pub at: Timestamp,
    /// The post that answers it, when one does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<PostId>,
}

/// Where a question stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestionState {
    Open,
    Answered,
}

/// Where `post` stands as a question (`None`: it is not one).
pub fn question_state(post: &Post) -> Option<QuestionState> {
    (post.kind == PostKind::Question).then(|| if post.extras.answered.is_some() { QuestionState::Answered } else { QuestionState::Open })
}

/// Whether `post` is a question still open.
pub fn is_open(post: &Post) -> bool {
    question_state(post) == Some(QuestionState::Open)
}

/// How an agent reads the question's state with the post.
pub fn agent_note(post: &Post) -> Option<&'static str> {
    match question_state(post)? {
        QuestionState::Open => Some("[An open question: the author has not marked it answered yet]"),
        QuestionState::Answered => Some("[A question the author has marked answered]"),
    }
}

#[cfg(test)]
mod tests;
