//! N18: open questions in the store. The user makes one of their posts a question (never a
//! locked post, nor a feeling) or an ordinary thought again, marks it answered (with the post
//! that answers it, when one does) and reopens it. The open questions of the active network
//! are listed oldest first.

use super::{require_persona, AppState};
use crate::domain::post::open_question::{is_open, Answered};
use crate::domain::post::PostKind;
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    fn question_position(&self, id: &PostId) -> Result<usize> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        if self.posts[position].is_locked() {
            return Err(AppError::PostLocked); // X2
        }
        Ok(position)
    }

    fn save_question(&self, position: usize) -> Result<()> {
        match &self.db {
            Some(db) => db.save_post(&self.posts[position]),
            None => Ok(()),
        }
    }

    /// Makes the user's post `id` a question (`true`) or an ordinary thought again.
    pub fn set_question(&mut self, id: &PostId, question: bool) -> Result<()> {
        let position = self.question_position(id)?;
        let post = &mut self.posts[position];
        if post.kind.is_feeling() {
            return Err(AppError::EmptyContent); // a feeling stays a feeling
        }
        post.kind = if question { PostKind::Question } else { PostKind::Standard };
        post.extras.answered = None;
        self.save_question(position)
    }

    /// Marks the user's question `id` answered, by `answer` when a post answers it.
    pub fn resolve_question(&mut self, id: &PostId, answer: Option<&PostId>, now: Timestamp) -> Result<()> {
        let position = self.question_position(id)?;
        if self.posts[position].kind != PostKind::Question || answer.is_some_and(|answer| answer == id || self.get_post(answer).is_none()) {
            return Err(AppError::EmptyContent);
        }
        self.posts[position].extras.answered = Some(Box::new(Answered { at: now, answer: answer.cloned() }));
        self.save_question(position)
    }

    /// Opens the user's answered question `id` again.
    pub fn reopen_question(&mut self, id: &PostId) -> Result<()> {
        let position = self.question_position(id)?;
        self.posts[position].extras.answered = None;
        self.save_question(position)
    }

    /// The questions of the active network still open, oldest first.
    pub fn open_questions(&self) -> Vec<&Post> {
        let mut open: Vec<&Post> = self.current_network_posts().filter(|post| is_open(post)).collect();
        open.sort_by_key(|post| post.created_at);
        open
    }
}
