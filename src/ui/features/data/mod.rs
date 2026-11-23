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
mod history_compare; mod history_log; mod history_rows; mod version_tree; // V5: the version tree
mod legend; // S9
mod lifecycle_menu; // Y5, Y6
mod margin; // N2
mod post_menu;
mod edited_mark; // V8
mod replied; // V12
mod review; // S16
mod saved;
mod scheduled; // Y4
mod sheet;
mod state_menu; // S7
mod snapshot;
mod text;
mod unlock; // X4

pub use events::{DataAction, DataEvent, SavedTarget, ScopeKind};
pub use export::{render_export_sheet, ExportDialog};
pub use legend::{render_legend_sheet, LegendView};
pub use margin::{render_margin_sheet, MarginDialog};
pub use history::{render_history_sheet, HistorySheet};
pub use edited_mark::render_edited_mark;
pub use post_menu::{with_data_items, DataControls, DataSource};
pub use replied::{render_replied_hint, replied_text};
pub use review::{render_review_sheet, tally_text, ReviewView};
pub use saved::{render_knowledge_switch, render_saved_page, SavedItemView, SavedView, ShelfChip};
pub use scheduled::{render_scheduled_sheet, ScheduledRow};
pub use sheet::closes_on_escape;
pub use snapshot::{render_restore_sheet, render_snapshot_sheet, RestoreDialog, RestoreStep, SnapshotDialog};
pub(crate) use snapshot::strength_meter;
pub use text::{format_time, tr};
pub use unlock::{render_unlock_sheet, UnlockAction, UnlockDialog, UnlockEvent};

#[cfg(test)]
mod tests;
