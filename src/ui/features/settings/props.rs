//! The data the settings window shows, built by the shell on every render.

use super::callbacks::{SelectPresetModelCallback, SelectProviderCallback};
use crate::agent::LlmSettings;
use crate::domain::preferences::Preferences;
use crate::theme::{Density, TypeScale};
use gpui_kit::component::input::InputState;
use gpui_kit::{Entity, FocusHandle};

/// A human persona the user can author as.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonaOption {
    pub id: String,
    pub name: String,
}

/// Live status of the autonomous scheduler.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SchedulerStatus {
    /// The scheduler switch.
    pub enabled: bool,
    pub heartbeat_seconds: u64,
    pub running: usize,
    pub queued: usize,
    /// The most recent model failure as `(provider, message)`, already redacted.
    pub last_error: Option<(String, String)>,
}

/// Read-only data of the window.
#[derive(Debug, Clone, PartialEq)]
pub struct SettingsData {
    pub prefs: Preferences,
    /// The theme scales owned by the Appearance pane.
    pub density: Density,
    pub type_scale: TypeScale,
    pub personas: Vec<PersonaOption>,
    pub database_path: String,
    pub media_path: String,
    pub scheduler: SchedulerStatus,
    /// The search box text.
    pub query: String,
    /// The AI provider draft differs from the saved settings.
    pub ai_dirty: bool,
    /// "Reset all settings" was clicked once and waits for the confirming click.
    pub confirm_reset: bool,
}

/// Text inputs owned by the shell.
#[derive(Clone)]
pub struct SettingsInputs {
    pub search: Entity<InputState>,
    pub embedding_model: Entity<InputState>,
    pub style_prefix: Entity<InputState>,
    /// Focus of the category rail (arrow keys move between categories).
    pub rail_focus: FocusHandle,
}

/// The AI provider pane: the settings it shows, its inputs and its callbacks.
pub struct AiPaneProps {
    pub settings: LlmSettings,
    pub api_key: Entity<InputState>,
    pub model: Entity<InputState>,
    pub endpoint: Entity<InputState>,
    pub on_select_provider: SelectProviderCallback,
    pub on_select_preset: SelectPresetModelCallback,
}
