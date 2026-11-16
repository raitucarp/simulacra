//! Likes, quotes and threadline replies.

use super::AppState;
use crate::domain::PostId;
use crate::error::{AppError, Result};

impl AppState {
    /// Whether a post with this id exists (index lookup, then linear scan).
    pub(super) fn post_exists(&self, post_id: &PostId) -> bool {
        self.post_index_by_id.contains_key(post_id) || self.posts.iter().any(|p| &p.id == post_id)
    }

    /// Submits a quoted thought referencing another thought with commentary and optional attachments.
    pub fn submit_quote(
        &mut self,
        target_post_id: &PostId,
        content: &str,
        link: Option<String>,
        img: Option<String>,
    ) -> Result<PostId> {
        if !self.post_exists(target_post_id) {
            return Err(AppError::DatabaseError(format!("Target post '{target_post_id}' not found")));
        }

        self.submit_post_with_attachments(
            content,
            link,
            img,
            None,
            Some(target_post_id.clone()),
        )
    }

    /// Submits a dialectic reply to a parent thought connecting them via threadlines.
    pub fn submit_reply(
        &mut self,
        parent_post_id: &PostId,
        content: &str,
        link: Option<String>,
        img: Option<String>,
    ) -> Result<PostId> {
        if !self.post_exists(parent_post_id) {
            return Err(AppError::DatabaseError(format!("Parent post '{parent_post_id}' not found")));
        }

        self.submit_post_with_attachments(
            content,
            link,
            img,
            Some(parent_post_id.clone()),
            None,
        )
    }

    /// Toggles like on a post by id and persists the change to SQLite.
    pub fn toggle_like(&mut self, post_id: &PostId) {
        let indexed = self
            .post_index_by_id
            .get(post_id)
            .copied()
            .filter(|&idx| idx < self.posts.len());
        let post = match indexed {
            Some(idx) => self.posts.get_mut(idx),
            None => self.posts.iter_mut().find(|p| &p.id == post_id),
        };

        if let Some(post) = post {
            post.toggle_like();
            if let Some(ref db) = self.db {
                let _ = db.update_post_like(post_id, post.likes, post.is_liked);
            }
        }
    }
}
