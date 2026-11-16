//! Applying edits to the working copy, and undo / redo over them.

use super::AgentDraft;
use crate::domain::Profile;
use crate::state::forge::edit::DraftEdit;

impl AgentDraft {
    /// Applies one edit as an undo step (consecutive edits to one text field merge).
    pub fn edit(&mut self, edit: DraftEdit) -> bool {
        let before = self.profile.clone();
        let changed = edit.apply(&mut self.profile);
        if changed {
            self.history.record(before, edit.coalesce_key());
        }
        changed
    }

    /// Applies several edits as a single undo step.
    pub fn edit_all(&mut self, edits: &[DraftEdit]) -> bool {
        self.transform(|profile| {
            for edit in edits {
                edit.apply(profile);
            }
        })
    }

    /// Runs `change` on the working copy as a single undo step; returns whether anything changed.
    pub(crate) fn transform(&mut self, change: impl FnOnce(&mut Profile)) -> bool {
        let before = self.profile.clone();
        change(&mut self.profile);
        let changed = self.profile != before;
        if changed {
            self.history.record(before, None);
        }
        changed
    }

    pub fn undo(&mut self) -> bool {
        self.history.undo(&mut self.profile)
    }

    pub fn redo(&mut self) -> bool {
        self.history.redo(&mut self.profile)
    }

    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// Ends the current text-typing run (e.g. when an input loses focus).
    pub fn seal_history(&mut self) {
        self.history.seal();
    }
}
