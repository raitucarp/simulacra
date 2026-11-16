//! Bounded snapshot history behind the draft's undo / redo.

use crate::domain::Profile;
use crate::state::forge::fields::TextField;
use std::collections::VecDeque;

/// Maximum number of undo snapshots kept; the oldest is dropped beyond this.
pub const HISTORY_LIMIT: usize = 64;

/// Undo / redo stacks of whole-profile snapshots.
#[derive(Debug, Clone, Default)]
pub(super) struct History {
    undo: VecDeque<Profile>,
    redo: Vec<Profile>,
    /// Text field whose run of edits is still merging into the newest snapshot.
    coalescing: Option<TextField>,
}

impl History {
    /// Records `before` as an undo point, unless it continues the current text run.
    pub(super) fn record(&mut self, before: Profile, coalesce: Option<TextField>) {
        self.redo.clear();
        if coalesce.is_some() && coalesce == self.coalescing {
            return;
        }
        self.undo.push_back(before);
        if self.undo.len() > HISTORY_LIMIT {
            self.undo.pop_front();
        }
        self.coalescing = coalesce;
    }

    /// Ends the current text run so the next edit opens a new undo step.
    pub(super) fn seal(&mut self) {
        self.coalescing = None;
    }

    /// Restores the newest snapshot into `current`; returns whether there was one.
    pub(super) fn undo(&mut self, current: &mut Profile) -> bool {
        self.seal();
        let Some(previous) = self.undo.pop_back() else { return false };
        self.redo.push(std::mem::replace(current, previous));
        true
    }

    /// Re-applies the newest undone snapshot; returns whether there was one.
    pub(super) fn redo(&mut self, current: &mut Profile) -> bool {
        self.seal();
        let Some(next) = self.redo.pop() else { return false };
        self.undo.push_back(std::mem::replace(current, next));
        true
    }

    pub(super) fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub(super) fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub(super) fn clear(&mut self) {
        *self = Self::default();
    }
}
