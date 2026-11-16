//! Application shell: the root [`AppView`] window with its title bar, three resizable
//! columns (navigation, feed / agent workspace, intelligence panel) and overlays.
//!
//! - `view` / `*_state`: the view struct and its grouped state (incl. the scheduler).
//! - `construct`: window-level construction and input subscriptions.
//! - `editor_*`: agent editor text fields, their inputs and draft-safe syncing.
//! - `handlers`: event handlers by responsibility; `binder` wires them to callbacks.
//! - `render`: section builders composed by `impl Render`.

mod binder;
mod consts;
mod construct;
mod editor_field;
mod editor_inputs;
mod editor_slots;
mod editor_sync;
mod feed_state;
mod graph_state;
mod handlers;
mod lab_state;
mod placeholders;
mod render;
mod scheduler_state;
mod stream_list;
mod settings_draft;
mod view;

#[cfg(test)]
mod tests;

pub use editor_inputs::AgentEditorFields;
pub use feed_state::FeedState;
pub use graph_state::GraphState;
pub use lab_state::{LabState, LlmInputs};
pub use scheduler_state::SchedulerState;
pub use settings_draft::SettingsDraft;
pub use view::{AppView, ChromeState};
