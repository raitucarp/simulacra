//! Archiving (Y5): an archived post leaves the feed but stays in the store, so the feed's
//! search, the full-text search, threads and links still find it. Unarchiving brings it back.

use super::{require_persona, AppState};
use crate::domain::post::PostStatus;
use crate::domain::PostId;
use crate::error::{AppError, Result};

impl AppState {
    /// Archives the user's own post `id` (agents' posts are never archived), or brings it back
    /// to the feed. `Ok(false)` when it already was.
    pub fn set_archived(&mut self, id: &PostId, archived: bool) -> Result<bool> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        let status = if archived { PostStatus::Archived } else { PostStatus::Published };
        if self.posts[position].lifecycle.status == status {
            return Ok(false);
        }
        self.posts[position].lifecycle.status = status;
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        self.index.listing_changed();
        Ok(true)
    }
}
