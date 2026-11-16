//! Floating popovers anchored to the title bar and sidebar (language, workspace).

pub mod language;
pub mod workspace;

pub use language::LanguagePopover;
pub use workspace::{WorkspacePopover, WorkspaceTrigger};
