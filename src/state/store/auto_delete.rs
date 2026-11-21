//! Auto-delete (Y7): the user sets a moment their own post deletes itself; the lifecycle
//! heartbeat then deletes it with everything kept about it.

use super::{require_persona, AppState};
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    /// Sets when the user's own post `id` deletes itself (`None` keeps it).
    pub fn set_auto_delete(&mut self, id: &PostId, at: Option<Timestamp>) -> Result<()> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        self.posts[position].lifecycle.delete_at = at;
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(())
    }

    /// Deletes every post whose moment has come at `now`, returning their ids.
    pub fn delete_due(&mut self, now: Timestamp) -> Result<Vec<PostId>> {
        let due = |post: &Post| post.lifecycle.delete_at.is_some_and(|at| at <= now);
        let ids: Vec<PostId> = self.posts.iter().filter(|post| due(post)).map(|post| post.id.clone()).collect();
        for id in &ids {
            self.purge_post(id)?;
        }
        Ok(ids)
    }

    /// Deletes post `id` (with its instant reposts, see [`AppState::delete_post`]) and what
    /// is kept about it (see [`crate::storage::Database::purge_post_traces`]).
    fn purge_post(&mut self, id: &PostId) -> Result<()> {
        self.delete_post(id)?;
        if let Some(ref db) = self.db {
            db.purge_post_traces(id)?;
        }
        Ok(())
    }
}
