//! Switching which version a post shows (V9): the head moves to any kept version, and the
//! version it left is kept in its place, so no version is ever lost and none is copied.

use super::{require_persona, AppState};
use crate::domain::data::history::{edited_post, RevisionId, RevisionSubject, VersionHead};
use crate::domain::PostId;
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    /// Shows the kept version `to` of the user's own post `id`, keeping the version it showed.
    /// Returns the version now shown.
    pub fn switch_post_head(&mut self, id: &PostId, to: &RevisionId, now: Timestamp) -> Result<VersionHead> {
        let post = self.get_post(id).cloned().ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&post))?;
        let revisions = self.post_revisions(id);
        let target = revisions.iter().find(|r| &r.id == to).ok_or_else(|| AppError::DatabaseError(format!("version '{}' not found", to.as_str())))?;
        let head = VersionHead::of(post.extras.head.as_ref(), &revisions, post.created_at);
        let kept = head.into_revision(RevisionSubject::Post(id.clone()), &post.content, now);
        let shown = VersionHead::from_revision(target);
        let mut switched = edited_post(&post, &target.content);
        switched.extras.head = Some(shown.clone());
        if let Some(ref db) = self.db {
            db.commit_post_version(switched.clone(), kept, Some(target.id.clone()))?;
        }
        self.replace_post_body(switched);
        Ok(shown)
    }
}
