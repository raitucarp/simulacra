//! Network and navigation handlers: workspace switching, nav tabs and sidebar shortcuts.

use super::super::view::AppView;
use crate::domain::{NetworkId, ProfileId};
use crate::state::NavigationTab;
use crate::ui::features::roster::LabMode;
use crate::ui::features::composer::ComposerModalMode;
use crate::ui::features::settings::SettingsTab;
use gpui_kit::*;

impl AppView {
    /// Switches to `net_id` and keeps the lab selection valid in the new network.
    pub(crate) fn select_network(&mut self, net_id: NetworkId, window: &mut Window, cx: &mut Context<Self>) {
        let _ = self.state.switch_network(&net_id);
        self.chrome.is_workspace_open = false;
        let mode = self.current_lab_mode();
        self.ensure_valid_lab_selection(mode, window, cx);
        cx.notify();
    }

    /// Navigates to `tab`; "Appearance" opens the settings modal instead.
    pub(crate) fn select_nav(&mut self, tab: NavigationTab, window: &mut Window, cx: &mut Context<Self>) {
        if tab == NavigationTab::Appearance {
            self.revert_settings_draft(window, cx);
            self.chrome.is_settings_open = true;
            self.chrome.active_settings_tab = SettingsTab::Appearance;
            cx.notify();
            return;
        }
        self.close_forge(window, cx);
        self.state.active_tab = tab;
        if tab == NavigationTab::AgentLab {
            self.ensure_valid_lab_selection(LabMode::NetworkAgents, window, cx);
        } else if tab == NavigationTab::PersonaLab {
            // Default to selecting the currently active user persona when opening Persona Lab
            let active_id = self.state.active_profile_id.clone();
            self.select_lab_profile(active_id, window, cx);
        }
        cx.notify();
    }

    /// Opens the composer modal for a new thread.
    pub(crate) fn new_thread(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open_composer_modal(ComposerModalMode::NewThread, window, cx);
    }

    /// Opens `agent_id` in the Agent Lab (Mind Graph agent nodes).
    pub(crate) fn select_sidebar_agent(&mut self, agent_id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        self.close_forge(window, cx);
        self.state.active_tab = NavigationTab::AgentLab;
        self.select_lab_profile(agent_id, window, cx);
    }

    /// Cycles the active persona, following it in the Persona Lab when open.
    pub(crate) fn switch_persona(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(new_id) = self.state.cycle_active_persona() {
            if self.state.active_tab == NavigationTab::PersonaLab {
                self.select_lab_profile(new_id, window, cx);
            }
            cx.notify();
        }
    }
}
