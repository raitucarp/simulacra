//! Application shell: the root [`AppView`] window with its title bar, three resizable
//! columns (navigation, feed / agent workspace, intelligence panel) and overlays.
//!
//! - `view` / `*_state`: the view struct and its grouped state (incl. the scheduler).
//! - `construct`: window-level construction and input subscriptions.
//! - `lab_selection` / `forge_host`: the roster selection and the Agent Forge it opens.
//! - `handlers`: event handlers by responsibility; `binder` wires them to callbacks.
//! - `render`: section builders composed by `impl Render`.

mod binder;
mod consts;
mod construct;
mod credential_state;
mod forge_host;
mod lab_selection;
mod feed_state;
mod graph_state;
mod handlers;
mod lab_state;
mod panels;
mod render;
mod scheduler_state;
mod semantic_state;
mod stream_list;
mod settings_draft;
mod settings_ui;
mod view;

#[cfg(test)]
mod tests;

pub use feed_state::FeedState;
pub use graph_state::GraphState;
pub use lab_state::{LabState, LlmInputs};
pub use scheduler_state::SchedulerState;
pub use semantic_state::SemanticState;
pub use settings_draft::SettingsDraft;
pub use settings_ui::SettingsUi;
pub use view::{AppView, ChromeState};
