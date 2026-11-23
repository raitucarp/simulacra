//! Personas & Agents roster handlers: selection, mode switch and profile actions.

use super::super::view::AppView;
use crate::domain::post::TriggerMode;
use crate::domain::ProfileId;
use crate::state::NavigationTab;
use crate::ui::features::roster::LabMode;
use crate::ui::features::sidebar_left::NavRef;
use gpui_kit::*;

impl AppView {
    /// Roster click: selects `profile_id`.
    pub(crate) fn lab_select_profile(&mut self, profile_id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        self.note_recent(NavRef::Profile(profile_id.clone()));
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

    /// A4: "Write a post…" opens the trigger dialog for `profile_id` (nothing runs yet).
    pub(crate) fn lab_trigger_agent(&mut self, profile_id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        self.open_trigger_dialog(profile_id, window, cx);
    }

    /// A4: the dialog's "Write": runs `mode` for `agent`.
    pub(crate) fn start_manual_trigger(&mut self, agent: ProfileId, mode: TriggerMode, window: &mut Window, cx: &mut Context<Self>) {
        self.trigger_simulation_for_agent(Some(agent), mode, window, cx);
        cx.notify();
    }
}
