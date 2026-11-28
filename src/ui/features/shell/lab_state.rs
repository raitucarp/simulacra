//! Personas & Agents roster selection and the LLM settings inputs.

use super::consts::LLM_ENDPOINT_PLACEHOLDER;
use super::credential_state::CredentialInputs;
use crate::domain::ProfileId;
use crate::state::AppState;
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use rust_i18n::t;

/// The profile selected in the roster.
pub struct LabState {
    pub selected_profile_id: Option<ProfileId>,
    /// W7: how the shown profile's Posts tab is shown.
    pub posts: crate::ui::features::roster::ProfilePosts,
    /// A5: the runs the user started from agents' profiles.
    pub runs: crate::agent::schedule::ManualRuns,
}

impl LabState {
    /// Selects the first network agent (or the first profile).
    pub(super) fn new(state: &AppState) -> Self {
        let first = state.current_network_agents().first().copied().or_else(|| state.user_profiles.first());
        Self { selected_profile_id: first.map(|profile| profile.id.clone()), posts: Default::default(), runs: Default::default() }
    }
}

/// Text inputs of the AI Intelligence settings pane.
pub struct LlmInputs {
    pub api_key: Entity<InputState>,
    pub model: Entity<InputState>,
    pub endpoint: Entity<InputState>,
    /// The credentials form below the global provider.
    pub credential: CredentialInputs,
}

impl LlmInputs {
    /// Creates the inputs pre-filled from the persisted LLM settings.
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut App) -> Self {
        let settings = &state.llm_settings;
        let mut input = |value: &str, placeholder: String, masked: bool| {
            let value = value.to_string();
            cx.new(|cx| InputState::new(window, cx).masked(masked).default_value(value).placeholder(placeholder))
        };
        Self {
            // The key is drawn as dots until its eye button shows it.
            api_key: input(&settings.api_key, t!("settings.api_key_placeholder").to_string(), true),
            model: input(&settings.model, t!("settings.model_placeholder").to_string(), false),
            endpoint: input(&settings.custom_endpoint, LLM_ENDPOINT_PLACEHOLDER.to_string(), false),
            credential: CredentialInputs::new(window, cx),
        }
    }

    /// The `(api key, model, endpoint)` currently typed.
    pub(super) fn values(&self, cx: &App) -> (String, String, String) {
        let read = |input: &Entity<InputState>| input.read(cx).value().to_string();
        (read(&self.api_key), read(&self.model), read(&self.endpoint))
    }
}
