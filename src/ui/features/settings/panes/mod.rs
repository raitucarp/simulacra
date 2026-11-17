//! Panes hosted by the settings rows: the AI provider pane, the scheduler status and the
//! Appearance pane (theme gallery and editor).

mod ai;
mod ai_fields;
mod appearance;
pub(super) mod common;
mod engine;

pub(super) use ai::render_ai_pane;
pub use appearance::{AppearanceEvent, AppearancePanel};
pub(super) use engine::render_scheduler_status;
