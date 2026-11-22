//! Editing a locked post (X5). The new text is sealed by the caller (off the UI thread) and
//! the new version holds only the placeholder and that seal; the version it replaces keeps
//! its own seal in its record (`PostRecord::lock`), so every version stays encrypted and
//! switching back to one brings its seal back.

use super::post_versions::VersionStep;
use super::{require_persona, AppState, PostEdit};
use crate::domain::data::history::{edited_post, Bump, Revision};
use crate::domain::post::lock::{PostLock, LOCKED_PLACEHOLDER};
use crate::domain::{first_thread_problem, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    /// Saves `edit` of the user's own locked post `id` as a new version sealed as `sealed`
    /// (the edit's text, sealed by the caller). `previous` is the opened text of the version
    /// shown, which the suggested bump is measured from; `edit.look` may change the tint and
    /// symbols, never add attachments.
    pub fn edit_locked_post(&mut self, id: &PostId, edit: &PostEdit, sealed: PostLock, previous: &str, now: Timestamp) -> Result<Revision> {
        let post = self.get_post(id).cloned().ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&post))?;
        if !post.is_locked() {
            return Err(AppError::LockFailed);
        }
        if let Some(problem) = first_thread_problem(&[&edit.content], false) {
            return Err(AppError::DatabaseError(format!("{:?}", problem.issue)));
        }
        let revisions = self.post_revisions(id);
        let base = edit.base.as_ref().and_then(|base| revisions.iter().find(|r| &r.id == base));
        let step = VersionStep { bump: edit.bump.unwrap_or_else(|| Bump::between(previous, &edit.content)), note: edit.note.clone(), base };
        let mut edited = edited_post(&post, LOCKED_PLACEHOLDER);
        if let Some(look) = &edit.look {
            super::symbols::check_symbols(self.author_of(&post), &look.symbols)?;
            (edited.extras.tint, edited.extras.symbols) = (look.tint, look.symbols.clone());
        }
        edited.extras.lock = Some(sealed);
        if !edited.is_sealed_or_plain() || edited.has_attachments() {
            return Err(AppError::LockFailed);
        }
        self.commit_version(edited, step, &revisions, now)
    }
}
