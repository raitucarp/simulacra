//! Version history in the store (sb F2): the tint of the user's own posts, and keeping a
//! vault document's previous text, title and record whenever it is edited. Post edits are
//! versions (`post_versions`).

use super::{require_persona, AppState};
use crate::domain::data::history::{is_real_edit, Revision, RevisionSubject};
use crate::domain::post::Tint;
use crate::domain::{GroundingDocument, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    /// Sets the container tint of the user's own post `id` (sb C3; agents' posts are never
    /// edited). A tint is not text, so no revision is kept. `Ok(false)` when it was already set.
    pub fn set_post_tint(&mut self, id: &PostId, tint: Option<Tint>) -> Result<bool> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        if self.posts[position].extras.tint == tint {
            return Ok(false);
        }
        self.posts[position].extras.tint = tint;
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(true)
    }

    /// Swaps in `edited` (same id) and refreshes what the index derived from its body.
    pub(super) fn replace_post_body(&mut self, edited: crate::domain::Post) {
        let Some(position) = self.post_position(&edited.id) else { return };
        self.index.content_changed(&edited);
        self.posts[position] = edited;
    }

    /// Keeps `before` as a revision when `after` changed its text, title or record.
    pub(super) fn keep_document_revision(&self, before: &GroundingDocument, after: &GroundingDocument) {
        let record_changed = crate::domain::document::CatalogueRecord { updated_at: None, ..before.record.clone() } != crate::domain::document::CatalogueRecord { updated_at: None, ..after.record.clone() };
        if !is_real_edit(&before.content, &after.content) && before.title == after.title && !record_changed {
            return;
        }
        let record = serde_json::to_string(&before.record).unwrap_or_default();
        let revision = Revision::of(RevisionSubject::Document(before.id.clone()), &before.content, Timestamp::now()).with_record(&before.title, record);
        if let Some(ref db) = self.db {
            if let Err(err) = db.save_revision(revision) {
                eprintln!("simulacra: document revision not queued: {err}");
            }
        }
    }
}
