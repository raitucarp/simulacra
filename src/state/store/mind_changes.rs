//! N19: "Changed my mind" in the store. The user marks one of their posts (never a locked one)
//! as having changed their mind, linked to the earlier post it revises (never a locked one,
//! nor itself) and/or with what they used to think; or takes the mark back. A post knows the
//! posts that revised it; the active network's changes of mind are listed newest first.

use super::{require_persona, AppState};
use crate::domain::post::mind::{clean_was, MindChange};
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    fn mind_position(&self, id: &PostId) -> Result<usize> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        Ok(position)
    }

    fn save_mind(&mut self, position: usize, change: Option<MindChange>) -> Result<()> {
        self.posts[position].extras.mind_change = change.map(Box::new);
        match &self.db {
            Some(db) => db.save_post(&self.posts[position]),
            None => Ok(()),
        }
    }

    /// Marks the user's post `id` as having changed their mind about `revises` (an earlier
    /// post) and/or from `was` (what they used to think).
    pub fn mark_changed_mind(&mut self, id: &PostId, revises: Option<&PostId>, was: &str, now: Timestamp) -> Result<MindChange> {
        let position = self.mind_position(id)?;
        let target = revises.map(|target| self.get_post(target).filter(|_| target != id).ok_or(AppError::EmptyContent)).transpose()?;
        if self.posts[position].is_locked() || target.is_some_and(Post::is_locked) {
            return Err(AppError::PostLocked); // X2: neither side speaks past a lock
        }
        let was = clean_was(was).ok_or(AppError::ContentTooLong { max: crate::domain::post::mind::MAX_WAS_CHARS, actual: was.chars().count() })?;
        if revises.is_none() && was.is_empty() {
            return Err(AppError::EmptyContent);
        }
        let change = MindChange { revises: revises.cloned(), was, at: now };
        self.save_mind(position, Some(change.clone()))?;
        Ok(change)
    }

    /// Takes the mark off the user's post `id`.
    pub fn unmark_changed_mind(&mut self, id: &PostId) -> Result<()> {
        let position = self.mind_position(id)?;
        self.save_mind(position, None)
    }

    /// The posts that revised post `id`, newest first (none for a locked post, nor locked ones).
    pub fn revised_by(&self, id: &PostId) -> Vec<&Post> {
        if self.get_post(id).is_none_or(Post::is_locked) {
            return Vec::new();
        }
        self.mind_changes().into_iter().filter(|post| post.extras.mind_change.as_ref().is_some_and(|change| change.revises.as_ref() == Some(id))).collect()
    }

    /// The active network's posts that changed the user's mind, newest first (never a locked one).
    pub fn mind_changes(&self) -> Vec<&Post> {
        let mut changes: Vec<&Post> = self.current_network_posts().filter(|post| !post.is_locked() && post.extras.mind_change.is_some()).collect();
        changes.sort_by_key(|post| std::cmp::Reverse(post.created_at));
        changes
    }
}
