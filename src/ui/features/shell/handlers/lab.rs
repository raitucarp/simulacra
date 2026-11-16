//! Personas & Agents roster handlers: selection, mode switch and profile actions.

use super::super::view::AppView;
use crate::domain::ProfileId;
use crate::state::NavigationTab;
use crate::ui::features::roster::LabMode;
use gpui_kit::*;

impl AppView {
    /// Roster click: selects `profile_id`.
    pub(crate) fn lab_select_profile(&mut self, profile_id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        self.select_lab_profile(profile_id, window, cx);
    }

    /// Toggles between the Network Agents and My Personas rosters.
    pub(crate) fn lab_switch_mode(&mut self, mode: LabMode, window: &mut Window, cx: &mut Context<Self>) {
        self.state.active_tab = match mode {
            LabMode::NetworkAgents => NavigationTab::AgentLab,
            LabMode::PersonaLab => NavigationTab::PersonaLab,
        };
        self.ensure_valid_lab_selection(mode, window, cx);
        cx.notify();
    }

    /// Deletes a profile and reselects a valid entry for the current roster.
    pub(crate) fn lab_delete_profile(&mut self, profile_id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        let _ = self.state.delete_profile(&profile_id);
        let mode = self.current_lab_mode();
        self.ensure_valid_lab_selection(mode, window, cx);
        cx.notify();
    }

    /// Makes `profile_id` the active posting persona.
    pub(crate) fn lab_activate_persona(&mut self, profile_id: ProfileId, _: &mut Window, cx: &mut Context<Self>) {
        let _ = self.state.switch_profile(&profile_id);
        cx.notify();
    }

    /// Runs one simulation turn for `profile_id`.
    pub(crate) fn lab_trigger_agent(&mut self, profile_id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        self.trigger_simulation_for_agent(Some(profile_id), window, cx);
    }
}
