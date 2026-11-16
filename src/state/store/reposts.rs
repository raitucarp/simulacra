//! Instant repost toggling.

use super::AppState;
use crate::domain::{NetworkId, Post, PostId, Profile, ProfileId};
use crate::error::{AppError, Result};

/// Reposts carry no commentary of their own.
const REPOST_CONTENT: &str = "";

fn target_not_found(target_post_id: &PostId) -> AppError {
    AppError::DatabaseError(format!("Target post '{target_post_id}' not found"))
}

impl AppState {
    /// Immediately publishes an instant repost of the target thought to the timeline.
    pub fn repost(&mut self, target_post_id: &PostId) -> Result<PostId> {
        let author = self.current_profile_or_err()?;

        // Check if author has already reposted this target post
        let already_reposted = self
            .posts
            .iter()
            .any(|p| p.repost_id.as_ref() == Some(target_post_id) && p.author_id == author.id);
        let is_now_reposted = !already_reposted;

        let (target_network_id, new_repost_count) =
            self.apply_repost_toggle(target_post_id, is_now_reposted)?;

        if let Some(ref db) = self.db {
            db.update_post_repost(target_post_id, new_repost_count, is_now_reposted)?;
        }

        if !is_now_reposted {
            self.remove_repost(target_post_id, &author.id);
            return Ok(target_post_id.clone());
        }
        self.publish_repost(target_post_id, target_network_id, author)
    }

    /// Updates the target's repost flag and counter; returns its network and new count.
    fn apply_repost_toggle(
        &mut self,
        target_post_id: &PostId,
        is_now_reposted: bool,
    ) -> Result<(NetworkId, usize)> {
        let target_idx = self.post_position(target_post_id).ok_or_else(|| target_not_found(target_post_id))?;

        let target = self
            .posts
            .get_mut(target_idx)
            .ok_or_else(|| target_not_found(target_post_id))?;

        target.is_reposted = is_now_reposted;
        if is_now_reposted {
            target.reposts_count += 1;
        } else if target.reposts_count > 0 {
            target.reposts_count -= 1;
        }
        Ok((target.network_id.clone(), target.reposts_count))
    }

    /// Removes the author's repost of the target from memory and SQLite.
    fn remove_repost(&mut self, target_post_id: &PostId, author_id: &ProfileId) {
        let is_match = |p: &Post| p.repost_id.as_ref() == Some(target_post_id) && &p.author_id == author_id;
        while let Some(position) = self.posts.iter().position(is_match) {
            self.remove_post_at(position);
        }
        if let Some(ref db) = self.db {
            let _ = db.delete_repost_by_target_and_author(target_post_id, author_id);
        }
    }

    /// Creates, persists and prepends a repost of the target.
    fn publish_repost(
        &mut self,
        target_post_id: &PostId,
        target_network_id: NetworkId,
        author: Profile,
    ) -> Result<PostId> {
        let post_id = Self::new_post_id();
        let repost_post = Post::create_with_attachments(
            post_id.clone(),
            target_network_id,
            author,
            REPOST_CONTENT,
            None,
            None,
            None,
            None,
            Some(target_post_id.clone()),
        )?;

        if let Some(ref db) = self.db {
            db.save_post(&repost_post)?;
        }

        self.insert_post_at_head(repost_post);
        Ok(post_id)
    }
}
