//! Public render parameters for the workspace and their internal split form.

use super::callbacks::*;
use super::config_tab::{AgentConfigTab, LabMode};
use super::inputs::AgentEditorInputs;
use crate::domain::ProfileId;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use std::rc::Rc;

/// Parameters for rendering the full Agent & Persona Lab Workspace.
pub struct AgentWorkspaceParams<'a> {
    pub state: &'a AppState,
    pub mode: LabMode,
    pub selected_profile_id: Option<&'a ProfileId>,
    pub active_config_tab: AgentConfigTab,
    pub active_sub_tab: usize,
    pub inputs: AgentEditorInputs<'a>,
    pub theme: &'a ThemeTokens,
    pub on_select_profile: SelectProfileCallback,
    pub on_select_tab: SelectConfigTabCallback,
    pub on_select_sub_tab: SelectSubTabCallback,
    pub on_switch_mode: SwitchLabModeCallback,
    pub on_update_profile: UpdateProfileCallback,
    pub on_save_text_inputs: SaveTextInputsCallback,
    pub on_create_profile: CreateProfileCallback,
    pub on_delete_profile: DeleteProfileCallback,
    pub on_pick_avatar: PickAvatarCallback,
    pub on_activate_persona: ActivatePersonaCallback,
    pub on_trigger_agent: TriggerAgentSimCallback,
}

/// The non-callback half of [`AgentWorkspaceParams`].
pub(crate) struct WorkspaceScope<'a> {
    pub state: &'a AppState,
    pub mode: LabMode,
    pub selected_profile_id: Option<&'a ProfileId>,
    pub active_config_tab: AgentConfigTab,
    pub active_sub_tab: usize,
    pub inputs: AgentEditorInputs<'a>,
    pub theme: &'a ThemeTokens,
}

impl<'a> AgentWorkspaceParams<'a> {
    /// Separates plain data from callbacks, wrapping each callback in an `Rc`.
    pub(crate) fn split(self) -> (WorkspaceScope<'a>, SharedCallbacks) {
        let callbacks = SharedCallbacks {
            select_profile: Rc::new(self.on_select_profile),
            select_tab: Rc::new(self.on_select_tab),
            select_sub_tab: Rc::new(self.on_select_sub_tab),
            switch_mode: Rc::new(self.on_switch_mode),
            update_profile: Rc::new(self.on_update_profile),
            save_text: Rc::new(self.on_save_text_inputs),
            create: Rc::new(self.on_create_profile),
            delete: Rc::new(self.on_delete_profile),
            pick_avatar: Rc::new(self.on_pick_avatar),
            activate_persona: Rc::new(self.on_activate_persona),
            trigger_agent: Rc::new(self.on_trigger_agent),
        };
        let scope = WorkspaceScope {
            state: self.state,
            mode: self.mode,
            selected_profile_id: self.selected_profile_id,
            active_config_tab: self.active_config_tab,
            active_sub_tab: self.active_sub_tab,
            inputs: self.inputs,
            theme: self.theme,
        };
        (scope, callbacks)
    }
}
