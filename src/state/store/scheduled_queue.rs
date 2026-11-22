//! The scheduled queue (Y4): editing a waiting post, cancelling it, or publishing it now.
//!
//! A scheduled thread moves as one: publishing or rescheduling any of its posts takes the
//! whole thread along, and cancelling a post cancels the later posts that answer it.

use super::scheduled::soonest_first;
use super::{require_persona, AppState};
use crate::domain::data::history::edited_post;
use crate::domain::post::Tint;
use crate::domain::{first_thread_problem, Post, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

fn not_scheduled(id: &PostId) -> AppError {
    AppError::DatabaseError(format!("scheduled post '{id}' not found"))
}

/// A scheduled post's new text, tint and moment (`None` publishes it at once).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledEdit {
    pub content: String,
    pub tint: Option<Tint>,
    pub publish_at: Option<Timestamp>,
}

impl AppState {
    /// The scheduled post `id`.
    pub fn scheduled_post(&self, id: &PostId) -> Option<&Post> {
        self.scheduled.iter().find(|post| &post.id == id)
    }

    /// `id` and the scheduled posts answering it, directly or further down its thread.
    fn scheduled_descendants(&self, id: &PostId) -> Vec<PostId> {
        let mut found = vec![id.clone()];
        let mut at = 0;
        while let Some(parent) = found.get(at).cloned() {
            found.extend(self.scheduled.iter().filter(|p| p.reply_to_id.as_ref() == Some(&parent)).map(|p| p.id.clone()));
            at += 1;
        }
        found
    }

    /// Every scheduled post of `id`'s thread (its scheduled root and everything under it).
    fn scheduled_thread_of(&self, id: &PostId) -> Vec<PostId> {
        let mut root = id.clone();
        while let Some(parent) = self.scheduled_post(&root).and_then(|p| p.reply_to_id.clone()).filter(|p| self.scheduled_post(p).is_some()) {
            root = parent;
        }
        self.scheduled_descendants(&root)
    }

    /// Takes the posts `ids` out of the queue, in publishing order.
    fn take_scheduled(&mut self, ids: &[PostId]) -> Vec<Post> {
        let (taken, kept): (Vec<Post>, Vec<Post>) = std::mem::take(&mut self.scheduled).into_iter().partition(|p| ids.contains(&p.id));
        self.scheduled = kept;
        soonest_first(taken)
    }

    /// Cancels the scheduled post `id` with the later posts answering it: they are deleted and
    /// never go out. Returns the cancelled ids.
    pub fn cancel_scheduled(&mut self, id: &PostId) -> Result<Vec<PostId>> {
        self.scheduled_post(id).ok_or_else(|| not_scheduled(id))?;
        let ids = self.scheduled_descendants(id);
        for post in self.take_scheduled(&ids) {
            if let Some(ref db) = self.db {
                db.delete_post_with_reposts(&post.id)?;
            }
        }
        Ok(ids)
    }

    /// Publishes `id`'s whole scheduled thread now, in thread order.
    pub fn publish_scheduled_now(&mut self, id: &PostId, now: Timestamp) -> Result<Vec<PostId>> {
        self.scheduled_post(id).ok_or_else(|| not_scheduled(id))?;
        let ids = self.scheduled_thread_of(id);
        self.take_scheduled(&ids).into_iter().map(|post| self.release_scheduled(post, now)).collect()
    }

    /// Rewrites the scheduled post `id` (the user's own, validated like a new post) and moves
    /// its thread to `edit.publish_at`, which must be after `now`; without a moment the thread
    /// is published at once.
    pub fn edit_scheduled(&mut self, id: &PostId, edit: ScheduledEdit, now: Timestamp) -> Result<()> {
        let post = self.scheduled_post(id).cloned().ok_or_else(|| not_scheduled(id))?;
        require_persona(self.author_of(&post))?;
        if post.is_locked() {
            return Err(AppError::PostLocked); // X2: plain text never replaces a sealed one
        }
        if let Some(problem) = first_thread_problem(&[edit.content.as_str()], post.has_attachments() || !post.extras.images.is_empty()) {
            return Err(AppError::DatabaseError(format!("{:?}", problem.issue)));
        }
        if edit.publish_at.is_some_and(|at| at <= now) {
            return Err(AppError::ScheduleNotInFuture);
        }
        let mut edited = edited_post(&post, &edit.content);
        edited.extras.tint = edit.tint;
        let thread = self.scheduled_thread_of(id);
        let posts = self.take_scheduled(&thread);
        let at = edit.publish_at;
        for mut post in posts.into_iter().map(|p| if &p.id == id { edited.clone() } else { p }) {
            post.lifecycle.publish_at = at.or(post.lifecycle.publish_at);
            self.hold_scheduled(post)?;
        }
        if at.is_none() {
            self.publish_scheduled_now(id, now)?;
        }
        Ok(())
    }
}
