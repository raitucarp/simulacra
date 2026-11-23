//! Which profile the Personas & Agents roster has selected, kept valid across networks and
//! roster modes.

use super::view::AppView;
use crate::domain::{Profile, ProfileId};
use crate::state::NavigationTab;
use crate::ui::features::roster::LabMode;
use gpui_kit::*;

impl AppView {
    /// Selects `profile_id` in the roster.
    pub fn select_lab_profile(&mut self, profile_id: ProfileId, _: &mut Window, cx: &mut Context<Self>) {
        if self.lab.selected_profile_id.as_ref() != Some(&profile_id) {
            self.reset_profile_posts(); // W7
        }
        self.lab.selected_profile_id = Some(profile_id);
        cx.notify();
    }

    /// Ensures the selection points to a profile listed in `mode` for the active network.
    pub fn ensure_valid_lab_selection(&mut self, mode: LabMode, window: &mut Window, cx: &mut Context<Self>) {
        let members = match mode {
            LabMode::NetworkAgents => self.state.current_network_agents(),
            LabMode::PersonaLab => self.state.current_network_personas(),
        };
        let valid_ids: Vec<ProfileId> = members.into_iter().map(|p| p.id.clone()).collect();
        let needs_reselect = self.lab.selected_profile_id.as_ref().is_none_or(|id| !valid_ids.contains(id));
        if needs_reselect {
            if let Some(first_id) = valid_ids.first().cloned() {
                self.select_lab_profile(first_id, window, cx);
            }
        }
    }

    /// A clone of the stored profile with `id`.
    pub(super) fn find_profile(&self, id: &ProfileId) -> Option<Profile> {
        self.state.user_profiles.iter().find(|p| &p.id == id).cloned()
    }

    /// The roster mode matching the active navigation tab (Persona Lab, else Network Agents).
    pub(super) fn current_lab_mode(&self) -> LabMode {
        if self.state.active_tab == NavigationTab::PersonaLab {
            LabMode::PersonaLab
        } else {
            LabMode::NetworkAgents
        }
    }

    /// Whether either roster (agents or personas) is the active tab.
    pub(super) fn is_lab_tab(&self) -> bool {
        matches!(self.state.active_tab, NavigationTab::AgentLab | NavigationTab::PersonaLab)
    }
}
