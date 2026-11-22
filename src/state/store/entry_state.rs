//! Moving a post's entry between states from its "…" menu (S7): done, migrate, schedule,
//! drop, delegate (and reopen). Each move is a patch version in the post's history, keeping
//! the symbols it had before in the revision's record. A task is done only once every sub-task
//! (a reply that is a task or event) is done or dropped (S8).

use super::post_versions::VersionStep;
use super::{require_persona, AppState};
use crate::domain::data::history::{Bump, Revision};
use crate::domain::symbols::{may_settle, unsettled_children, EntryState, EntrySymbol};
use crate::domain::PostId;
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    /// Moves the entry of the user's own post `id` to `to`, keeping the move as a patch
    /// revision. Refused for agents' posts, posts without a task or event, and moves the
    /// entry's rules do not allow.
    pub fn set_entry_state(&mut self, id: &PostId, to: EntryState, now: Timestamp) -> Result<Revision> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        let before = self.posts[position].extras.symbols.clone();
        let mut after = before.clone();
        if !after.set_state(to) {
            return Err(AppError::InvalidSymbols);
        }
        let open = unsettled_children(self.replies_for_post(id).into_iter().map(|reply| &reply.extras.symbols));
        if before.entry == Some(EntrySymbol::Task) && !may_settle(to, self.replies_for_post(id).into_iter().map(|reply| &reply.extras.symbols)) {
            return Err(AppError::OpenSubtasks(open)); // S8
        }
        let record_json = serde_json::to_string(&before).unwrap_or_default();
        let mut edited = self.posts[position].clone();
        edited.extras.symbols = after;
        let revisions = self.post_revisions(id);
        self.commit_version(edited, VersionStep { bump: Bump::Patch, note: String::new(), base: None, record_json }, &revisions, now)
    }
}
