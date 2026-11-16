//! Agent & Persona Lab workspace wiring.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::agent_editor::{render_agent_workspace_view, AgentWorkspaceParams};
use gpui_kit::*;

impl AppView {
    /// Two-pane roster + editor for the active lab mode.
    pub(super) fn agent_workspace(&self, bind: &Binder) -> impl IntoElement {
        render_agent_workspace_view(AgentWorkspaceParams {
            state: &self.state,
            mode: self.current_lab_mode(),
            selected_profile_id: self.lab.selected_profile_id.as_ref(),
            active_config_tab: self.lab.active_config_tab,
            active_sub_tab: self.lab.active_sub_tab,
            inputs: self.lab.fields.as_params(),
            theme: &self.theme,
            on_select_profile: bind.boxed(Self::lab_select_profile),
            on_select_tab: bind.boxed(Self::lab_select_tab),
            on_select_sub_tab: bind.boxed(Self::lab_select_sub_tab),
            on_switch_mode: bind.boxed(Self::lab_switch_mode),
            on_update_profile: bind.boxed(Self::lab_update_profile),
            on_save_text_inputs: bind.boxed_act(Self::lab_save_text),
            on_create_profile: bind.boxed_act(Self::lab_create_profile),
            on_delete_profile: bind.boxed(Self::lab_delete_profile),
            on_pick_avatar: bind.boxed(Self::lab_pick_avatar),
            on_activate_persona: bind.boxed(Self::lab_activate_persona),
            on_trigger_agent: bind.boxed(Self::lab_trigger_agent),
        })
    }
}
