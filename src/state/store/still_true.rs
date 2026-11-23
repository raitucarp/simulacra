//! N13: "Still true?" in the store: the user's own claims due for a check, and confirming one
//! (it comes back later, the wait doubling). Revising is an edit (a new version restarts the
//! wait); retracting is N14. Locked posts are never due.

use super::{require_persona, AppState};
use crate::domain::post::still_true::{confirmed, due_at, is_due, StillTrue};
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    /// The user's own posts in the active network due for a check at `now`, longest due first.
    pub fn due_for_check(&self, now: Timestamp) -> Vec<&Post> {
        let mut due: Vec<&Post> = self.current_network_posts().filter(|post| self.author_of(post).is_human_persona() && is_due(post, now)).collect();
        due.sort_by_key(|post| due_at(post));
        due
    }

    /// Confirms that the user's post `id` still holds at `now`.
    pub fn confirm_still_true(&mut self, id: &PostId, now: Timestamp) -> Result<StillTrue> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        if self.posts[position].is_locked() {
            return Err(AppError::PostLocked);
        }
        let check = confirmed(&self.posts[position], now);
        self.posts[position].extras.check = Some(Box::new(check.clone()));
        if let Some(db) = &self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(check)
    }
}
