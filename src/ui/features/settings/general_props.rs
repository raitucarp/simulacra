//! Live data and callbacks shown by the General pane (scheduler switch, status, vault path).

use super::callbacks::ToggleSchedulerCallback;

/// Inputs of the General settings pane.
#[derive(Default)]
pub struct GeneralPaneProps {
    /// Whether the autonomous scheduler is on.
    pub scheduler_enabled: bool,
    /// Heartbeat period in seconds, shown in the switch hint.
    pub heartbeat_seconds: u64,
    /// Generations currently running.
    pub running: usize,
    /// Agents waiting for a generation slot.
    pub queued: usize,
    /// The most recent LLM failure as `(provider, message)`, already redacted.
    pub last_error: Option<(String, String)>,
    /// Resolved database file path.
    pub db_path: String,
    /// Native name of the active interface language.
    pub language_label: &'static str,
    /// Invoked with the requested switch value.
    pub on_toggle_scheduler: Option<ToggleSchedulerCallback>,
}
