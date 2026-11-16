//! Keeps the agent editor inputs and the stored profiles in step without losing drafts.

use super::editor_field::EditorTextField;
use super::view::AppView;
use crate::domain::{Profile, ProfileId};
use crate::state::NavigationTab;
use crate::ui::features::agent_editor::LabMode;
use gpui_kit::*;

impl AppView {
    /// Overwrites every editor text input with the values of `profile`, discarding any draft.
    pub fn sync_agent_editor_inputs(
        &mut self,
        profile: &Profile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for (field, slot) in self.lab.fields.fields() {
            slot.set(&field.display_value(profile), window, cx);
        }
    }

    /// Refreshes only the inputs whose saved value differs between `previous` and `updated`.
    ///
    /// Used after an immediate (chip / stepper / preset) edit so that a field the edit changed
    /// on purpose — e.g. pronouns reset by a gender chip, or a cleared avatar — is shown, while
    /// every other input keeps the user's unsaved draft.
    pub(super) fn sync_changed_editor_inputs(
        &mut self,
        previous: &Profile,
        updated: &Profile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for (field, slot) in self.lab.fields.fields() {
            let next = field.display_value(updated);
            if field.display_value(previous) != next {
                slot.set(&next, window, cx);
            }
        }
    }

    /// Replaces the text of a single editor input.
    pub(super) fn set_editor_field(
        &self,
        field: EditorTextField,
        value: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.lab.fields.slot(field).set(value, window, cx);
    }

    /// Reads all text input fields and persists them to the currently selected agent/persona profile.
    ///
    /// Does nothing when the inputs hold no changes, so it is safe to call before any
    /// navigation that would otherwise drop the drafts.
    pub fn save_agent_editor_text_inputs(&mut self, cx: &mut Context<Self>) {
        let Some(target_id) = self.lab.selected_profile_id.clone() else {
            return;
        };
        let Some(stored) = self.find_profile(&target_id) else {
            return;
        };
        let mut prof = stored.clone();
        for (field, slot) in self.lab.fields.fields() {
            field.apply(&mut prof, &slot.read(cx));
        }
        if prof == stored {
            return;
        }
        let _ = self.state.update_profile(prof);
        cx.notify();
    }

    /// Selects a specific agent or persona in the Two-Pane Lab and syncs all text inputs.
    ///
    /// Unsaved drafts of the previously selected profile are saved first, so every caller
    /// (roster, mode switch, network switch, persona cycling) is safe from losing edits.
    pub fn select_lab_profile(
        &mut self,
        profile_id: ProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.save_agent_editor_text_inputs(cx);
        self.lab.selected_profile_id = Some(profile_id.clone());
        if let Some(prof) = self.find_profile(&profile_id) {
            self.sync_agent_editor_inputs(&prof, window, cx);
        }
        cx.notify();
    }

    /// Ensures `selected_profile_id` points to a valid entry for the active mode and network.
    pub fn ensure_valid_lab_selection(
        &mut self,
        mode: LabMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let members = match mode {
            LabMode::NetworkAgents => self.state.current_network_agents(),
            LabMode::PersonaLab => self.state.current_network_personas(),
        };
        let valid_ids: Vec<ProfileId> = members.into_iter().map(|p| p.id.clone()).collect();
        let needs_reselect = match &self.lab.selected_profile_id {
            Some(id) => !valid_ids.contains(id),
            None => true,
        };
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

    /// The lab mode matching the active navigation tab (Persona Lab, else Network Agents).
    pub(super) fn current_lab_mode(&self) -> LabMode {
        if self.state.active_tab == NavigationTab::PersonaLab {
            LabMode::PersonaLab
        } else {
            LabMode::NetworkAgents
        }
    }

    /// Whether either lab (agents or personas) is the active tab.
    pub(super) fn is_lab_tab(&self) -> bool {
        matches!(self.state.active_tab, NavigationTab::AgentLab | NavigationTab::PersonaLab)
    }
}
