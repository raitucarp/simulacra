//! Agent & Persona Lab selection state and the LLM settings inputs.

use super::consts::{
    FALLBACK_LAB_PROFILE, LLM_API_KEY_PLACEHOLDER, LLM_ENDPOINT_PLACEHOLDER, LLM_MODEL_PLACEHOLDER,
};
use super::editor_inputs::AgentEditorFields;
use crate::domain::{Profile, ProfileId};
use crate::state::AppState;
use crate::ui::features::agent_editor::AgentConfigTab;
use gpui_kit::component::input::InputState;
use gpui_kit::*;

/// Two-pane lab selection plus the editor inputs bound to the selected profile.
pub struct LabState {
    pub selected_profile_id: Option<ProfileId>,
    pub active_config_tab: AgentConfigTab,
    pub active_sub_tab: usize,
    pub fields: AgentEditorFields,
}

impl LabState {
    /// Selects the first network agent (or first profile) and fills the editor from it.
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut App) -> Self {
        let (id, name, handle, occupation) = FALLBACK_LAB_PROFILE;
        let profile = state
            .current_network_agents()
            .first()
            .copied()
            .or_else(|| state.user_profiles.first())
            .cloned()
            .unwrap_or_else(|| Profile::new_agent(id, name, handle, occupation));
        Self {
            selected_profile_id: Some(profile.id.clone()),
            active_config_tab: AgentConfigTab::IdentityAndBio,
            active_sub_tab: 0,
            fields: AgentEditorFields::new(&profile, window, cx),
        }
    }
}

/// Text inputs of the AI Intelligence settings pane.
pub struct LlmInputs {
    pub api_key: Entity<InputState>,
    pub model: Entity<InputState>,
    pub endpoint: Entity<InputState>,
}

impl LlmInputs {
    /// Creates the inputs pre-filled from the persisted LLM settings.
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut App) -> Self {
        let settings = &state.llm_settings;
        let mut input = |value: &str, placeholder: &'static str| {
            let value = value.to_string();
            cx.new(|cx| InputState::new(window, cx).default_value(value).placeholder(placeholder))
        };
        Self {
            api_key: input(&settings.api_key, LLM_API_KEY_PLACEHOLDER),
            model: input(&settings.model, LLM_MODEL_PLACEHOLDER),
            endpoint: input(&settings.custom_endpoint, LLM_ENDPOINT_PLACEHOLDER),
        }
    }

    /// The `(api key, model, endpoint)` currently typed.
    pub(super) fn values(&self, cx: &App) -> (String, String, String) {
        let read = |input: &Entity<InputState>| input.read(cx).value().to_string();
        (read(&self.api_key), read(&self.model), read(&self.endpoint))
    }
}
