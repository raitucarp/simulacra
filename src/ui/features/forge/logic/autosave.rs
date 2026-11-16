//! When to write the in-progress draft to the settings table: on every step change that
//! follows an edit, and after every [`AUTOSAVE_EVERY_EDITS`] edits in between.

/// Edits after which the draft is autosaved even without a step change.
pub const AUTOSAVE_EVERY_EDITS: u32 = 8;

/// Counts edits since the last autosave.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AutosavePolicy {
    pending: u32,
}

impl AutosavePolicy {
    /// Records one edit; `true` when the draft should be saved now.
    pub fn record_edit(&mut self) -> bool {
        self.pending += 1;
        self.pending >= AUTOSAVE_EVERY_EDITS
    }

    /// A step change; `true` when there are unsaved edits worth saving.
    pub fn on_step_change(&self) -> bool {
        self.pending > 0
    }

    /// Whether edits happened since the last autosave.
    pub fn has_pending(&self) -> bool {
        self.pending > 0
    }

    /// The draft was just autosaved (or committed / discarded).
    pub fn mark_saved(&mut self) {
        self.pending = 0;
    }
}
