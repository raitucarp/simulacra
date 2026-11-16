//! The sticky "Unsaved changes · Discard / Save" bar.

use crate::state::forge::{Severity, ValidationIssue};

/// What the save bar shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveBarState {
    /// Nothing changed since the draft opened or was last saved.
    Hidden,
    /// Unsaved changes that may be saved.
    Ready,
    /// Unsaved changes, but `errors` blocking issues keep Save disabled.
    Blocked { errors: usize },
}

impl SaveBarState {
    pub fn is_visible(self) -> bool {
        self != Self::Hidden
    }

    pub fn can_save(self) -> bool {
        self == Self::Ready
    }
}

/// The bar for a draft that is `dirty` with these validation `issues`.
pub fn save_bar_state(dirty: bool, issues: &[ValidationIssue]) -> SaveBarState {
    if !dirty {
        return SaveBarState::Hidden;
    }
    match issues.iter().filter(|issue| issue.severity == Severity::Error).count() {
        0 => SaveBarState::Ready,
        errors => SaveBarState::Blocked { errors },
    }
}
