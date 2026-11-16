//! Deleting a thought (with its instant reposts) while keeping related counters in step.

use super::AppState;
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};

/// Whether `post` is a content-less instant repost of `target`.
fn is_instant_repost_of(post: &Post, target: &PostId) -> bool {
    post.repost_id.as_ref() == Some(target) && post.content.is_empty()
}

impl AppState {
    /// Deletes `post_id` and its instant reposts from memory and SQLite. The parent's reply
    /// counter and a reposted target's counter are decremented; replies and quotes stay.
    pub fn delete_post(&mut self, post_id: &PostId) -> Result<()> {
        let post = self
            .get_post(post_id)
            .cloned()
            .ok_or_else(|| AppError::DatabaseError(format!("post '{post_id}' not found")))?;
        if let Some(ref db) = self.db {
            db.delete_post_with_reposts(post_id)?;
        }
        let doomed = |p: &Post| p.id == *post_id || is_instant_repost_of(p, post_id);
        while let Some(position) = self.posts.iter().position(doomed) {
            self.remove_post_at(position);
        }
        self.release_parent_counter(&post);
        self.release_repost_counter(&post);
        Ok(())
    }

    /// Decrements the reply counter of the deleted reply's parent.
    fn release_parent_counter(&mut self, post: &Post) {
        let Some(parent_id) = post.reply_to_id.as_ref() else { return };
        let Some(parent) = self.post_mut(parent_id) else { return };
        parent.replies_count = parent.replies_count.saturating_sub(1);
        let count = parent.replies_count;
        if let Some(ref db) = self.db {
            let _ = db.update_post_replies_count(parent_id, count);
        }
    }

    /// Decrements the repost counter (and clears the flag) of a deleted instant repost's target.
    fn release_repost_counter(&mut self, post: &Post) {
        let Some(target_id) = post.repost_id.as_ref().filter(|_| post.content.is_empty()) else {
            return;
        };
        let Some(target) = self.post_mut(target_id) else { return };
        target.reposts_count = target.reposts_count.saturating_sub(1);
        target.is_reposted = false;
        let count = target.reposts_count;
        if let Some(ref db) = self.db {
            let _ = db.update_post_repost(target_id, count, false);
        }
    }

    /// Mutable access to a post by id through the index.
    fn post_mut(&mut self, id: &PostId) -> Option<&mut Post> {
        let idx = self.post_position(id)?;
        self.posts.get_mut(idx)
    }
}
