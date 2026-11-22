//! Application shell: the root [`AppView`] window with its title bar, three resizable
//! columns (navigation, feed / agent workspace, intelligence panel) and overlays.
//!
//! - `view` / `*_state`: the view struct and its grouped state (incl. the scheduler).
//! - `construct`: window-level construction and input subscriptions.
//! - `lab_selection` / `forge_host`: the roster selection and the Agent Forge it opens.
//! - `handlers`: event handlers by responsibility; `binder` wires them to callbacks.
//! - `render`: section builders composed by `impl Render`.

mod arena_host;
mod binder;
mod consts;
mod construct;
mod credential_state;
mod data_host; // sb F2
mod forge_host;
mod forge_suggest;
mod lab_selection;
mod feed_state;
mod graph_state;
mod handlers;
mod image_ideas_state;
mod import_state;
mod insights_host;
mod journal_host;
mod inspector_state;
mod lab_state;
mod lens_host; // sb F3
mod learning_host;
mod media_state;
mod panels;
mod post_actions_state;
mod review_host;
mod render;
mod scheduler_state;
mod reactions_host; // epic R
mod symbols_host; // epic S
mod recommend_host;
mod semantic_state;
mod spend_host; // sb P16
mod stream_list;
mod thinking_host;
mod settings_draft;
mod settings_ui;
mod vault_host;
mod view;

#[cfg(test)]
mod tests;

pub use feed_state::FeedState;
pub use graph_state::GraphState;
pub use import_state::ImportUi;
pub use lab_state::{LabState, LlmInputs};
pub use media_state::MediaHost;
pub use scheduler_state::SchedulerState;
pub use semantic_state::SemanticState;
pub use settings_draft::SettingsDraft;
pub use settings_ui::SettingsUi;
pub use view::{AppView, ChromeState};
