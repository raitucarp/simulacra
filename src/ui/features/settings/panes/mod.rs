//! Panes hosted by the settings rows: the AI provider pane, the scheduler status and the
//! Appearance pane (theme gallery and editor).

mod ai;
mod ai_fields;
mod appearance;
pub(super) mod common;
mod engine;
mod licences; // S13
mod runtimes; // P1
mod symbol_sets; // S13

pub(super) use ai::render_ai_pane;
pub use appearance::{AppearanceEvent, AppearancePanel};
pub(super) use engine::render_scheduler_status;
pub(super) use licences::render_licences;
pub(super) use runtimes::render_runtimes;
pub use runtimes::{checked_label, RuntimeAction, RuntimeCallback, RuntimesProps};
pub use licences::LicenceView;
pub(super) use symbol_sets::render_symbol_sets;
pub use symbol_sets::{action_set, set_label, SetDownload, SymbolSetAction, SymbolSetCallback, SymbolSetsProps};
