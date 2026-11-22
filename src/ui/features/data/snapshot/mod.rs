//! Encrypted snapshots: the sheet that seals the whole vault with a passphrase (typed twice,
//! with a strength hint and the plain warning that a lost passphrase cannot be recovered),
//! and the restore sheet (pick a file, verify it with its passphrase, then confirm).

mod create;
mod meter;
mod restore;

pub use create::render_snapshot_sheet;
pub use restore::render_restore_sheet;
pub(crate) use meter::strength_meter; // X3: the composer's lock row

use crate::domain::data::snapshot::Strength;
use gpui_kit::component::input::InputState;
use gpui_kit::Entity;
use std::path::PathBuf;

/// The snapshot sheet's state.
#[derive(Clone)]
pub struct SnapshotDialog {
    pub passphrase: Entity<InputState>,
    pub repeat: Entity<InputState>,
    pub strength: Strength,
    /// Both fields hold the same passphrase.
    pub matches: bool,
    pub long_enough: bool,
    pub busy: bool,
    pub problem: Option<String>,
}

impl SnapshotDialog {
    pub fn new(passphrase: Entity<InputState>, repeat: Entity<InputState>) -> Self {
        Self { passphrase, repeat, strength: Strength::Empty, matches: false, long_enough: false, busy: false, problem: None }
    }

    /// Whether "Create snapshot" may run.
    pub fn ready(&self) -> bool {
        self.matches && self.long_enough && !self.busy
    }
}

/// Where a restore stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreStep {
    /// Pick a snapshot file and type its passphrase.
    Choose,
    /// Decrypted and checked; waiting for the user to confirm the swap.
    Verified { created_at: String, posts: usize, profiles: usize, documents: usize },
}

/// The restore sheet's state.
#[derive(Clone)]
pub struct RestoreDialog {
    pub step: RestoreStep,
    pub file: Option<PathBuf>,
    pub passphrase: Entity<InputState>,
    pub busy: bool,
    pub problem: Option<String>,
}

impl RestoreDialog {
    pub fn new(passphrase: Entity<InputState>) -> Self {
        Self { step: RestoreStep::Choose, file: None, passphrase, busy: false, problem: None }
    }
}
