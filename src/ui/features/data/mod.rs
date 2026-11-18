//! Data ownership views (sb F2): the post menu's Edit, History, Bookmark and collection
//! items with the card's "edited" mark; the history sheet with its word diff; the export
//! dialog; the encrypted snapshot and restore sheets; and the Knowledge page's Saved view.
//!
//! Every view here is stateless: the shell keeps the state (`shell::data_host`) and receives
//! one [`DataEvent`] per interaction.

mod diff_view;
mod events;
mod export;
mod history;
mod post_menu;
mod saved;
mod sheet;
mod snapshot;
mod text;

pub use events::{DataAction, DataEvent, SavedTarget, ScopeKind};
pub use export::{render_export_sheet, ExportDialog};
pub use history::{render_history_sheet, HistorySheet};
pub use post_menu::{render_edited_mark, with_data_items, DataControls, DataSource};
pub use saved::{render_knowledge_switch, render_saved_page, SavedItemView, SavedView, ShelfChip};
pub use snapshot::{render_restore_sheet, render_snapshot_sheet, RestoreDialog, RestoreStep, SnapshotDialog};
pub use text::{format_time, tr};

#[cfg(test)]
mod tests;
