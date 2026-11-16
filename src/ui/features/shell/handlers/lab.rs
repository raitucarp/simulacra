//! Agent & Persona Lab handlers: selection, tabs, immediate edits and profile lifecycle.

use super::super::view::AppView;
use crate::domain::{Profile, ProfileId};
use crate::state::NavigationTab;
use crate::ui::features::agent_editor::{AgentConfigTab, LabMode};
use gpui_kit::*;

impl AppView {
    /// Roster click: saves drafts, then loads `profile_id` into the editor.
    pub(crate) fn lab_select_profile(&mut self, profile_id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        self.save_agent_editor_text_inputs(cx);
        self.select_lab_profile(profile_id, window, cx);
    }

    /// Switches the configuration category, resetting the sub-tab when it changes.
    pub(crate) fn lab_select_tab(&mut self, tab: AgentConfigTab, _: &mut Window, cx: &mut Context<Self>) {
        self.save_agent_editor_text_inputs(cx);
        if self.lab.active_config_tab != tab {
            self.lab.active_config_tab = tab;
            self.lab.active_sub_tab = 0;
        }
        cx.notify();
    }

    pub(crate) fn lab_select_sub_tab(&mut self, sub_idx: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.save_agent_editor_text_inputs(cx);
        self.lab.active_sub_tab = sub_idx;
        cx.notify();
    }

    /// Toggles between the Network Agents and Persona Lab rosters.
    pub(crate) fn lab_switch_mode(&mut self, mode: LabMode, window: &mut Window, cx: &mut Context<Self>) {
        self.save_agent_editor_text_inputs(cx);
        self.state.active_tab = match mode {
            LabMode::NetworkAgents => NavigationTab::AgentLab,
            LabMode::PersonaLab => NavigationTab::PersonaLab,
        };
        self.ensure_valid_lab_selection(mode, window, cx);
        cx.notify();
    }

    /// Commits an immediate (chip / stepper / preset) edit, refreshing only changed inputs.
    pub(crate) fn lab_update_profile(&mut self, updated: Profile, window: &mut Window, cx: &mut Context<Self>) {
        let previous = self.find_profile(&updated.id);
        let _ = self.state.update_profile(updated.clone());
        match previous {
            Some(previous) => self.sync_changed_editor_inputs(&previous, &updated, window, cx),
            None => self.sync_agent_editor_inputs(&updated, window, cx),
        }
        cx.notify();
    }

    pub(crate) fn lab_save_text(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.save_agent_editor_text_inputs(cx);
    }

    /// Deletes a profile and reselects a valid entry for the current lab.
    pub(crate) fn lab_delete_profile(&mut self, profile_id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        let _ = self.state.delete_profile(&profile_id);
        let mode = self.current_lab_mode();
        self.ensure_valid_lab_selection(mode, window, cx);
        cx.notify();
    }

    /// Makes `profile_id` the active posting persona.
    pub(crate) fn lab_activate_persona(&mut self, profile_id: ProfileId, _: &mut Window, cx: &mut Context<Self>) {
        self.save_agent_editor_text_inputs(cx);
        let _ = self.state.switch_profile(&profile_id);
        cx.notify();
    }

    /// Runs one simulation turn for `profile_id`.
    pub(crate) fn lab_trigger_agent(&mut self, profile_id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        self.save_agent_editor_text_inputs(cx);
        self.trigger_simulation_for_agent(Some(profile_id), window, cx);
    }
}
