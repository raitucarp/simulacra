//! The one dispatcher of registry commands: shortcuts (the keymap's `RunCommand` action),
//! the palette, the sidebar and its "New" menu all end up in [`AppView::run_command`].

use super::super::view::AppView;
use crate::state::NavigationTab;
use crate::ui::features::command_palette::{spec, CommandId, PaletteMode};
use crate::ui::features::settings::SettingsTab;
use crate::ui::keymap::RunCommand;
use gpui_kit::*;

impl AppView {
    /// Root listener of the keymap's [`RunCommand`] action.
    pub(in crate::ui::features::shell) fn on_run_command(&mut self, action: &RunCommand, window: &mut Window, cx: &mut Context<Self>) {
        self.run_command(action.0, window, cx);
    }

    /// Runs `id`; planned commands do nothing until a later wave implements them.
    pub(crate) fn run_command(&mut self, id: CommandId, window: &mut Window, cx: &mut Context<Self>) {
        if !spec(id).is_runnable() {
            return;
        }
        match id {
            CommandId::GoTimeline => self.select_nav(NavigationTab::Timeline, window, cx),
            CommandId::GoKnowledge => self.select_nav(NavigationTab::Knowledge, window, cx),
            CommandId::GoAgents => self.select_nav(NavigationTab::AgentLab, window, cx),
            CommandId::GoPersonas => self.select_nav(NavigationTab::PersonaLab, window, cx),
            CommandId::GoMindGraph => self.select_nav(NavigationTab::MindGraph, window, cx),
            CommandId::GoReview => self.select_nav(NavigationTab::Review, window, cx),
            CommandId::NewThread => self.new_thread(window, cx),
            CommandId::NewNote => self.open_quick_capture(window, cx),
            CommandId::ImportFile => self.pick_import_files(window, cx),
            CommandId::OpenInbox => self.open_inbox(window, cx),
            CommandId::NewAgent => self.open_forge_new(window, cx),
            CommandId::CreateNetwork => self.open_create_network(window, cx),
            CommandId::SwitchNetwork => self.toggle_workspace(window, cx),
            CommandId::SwitchPersona => self.switch_persona(window, cx),
            CommandId::PinThread => self.pin_open_thread(window, cx),
            CommandId::OpenSettings => self.open_settings_tab(SettingsTab::General, window, cx),
            CommandId::OpenAppearance => self.open_settings_tab(SettingsTab::Appearance, window, cx),
            CommandId::ToggleSidebar => self.toggle_left_sidebar(window, cx),
            CommandId::ToggleRightSidebar => self.toggle_right_sidebar(window, cx),
            CommandId::ShowShortcuts => self.toggle_shortcut_sheet(window, cx),
            CommandId::OpenPalette => self.open_palette(PaletteMode::Everything, window, cx),
            CommandId::OpenCommands => self.open_palette(PaletteMode::Commands, window, cx),
            CommandId::ReplayWelcome => self.replay_welcome(window, cx),
            CommandId::SemanticSearch => self.open_palette(PaletteMode::Semantic, window, cx),
            CommandId::FindSimilar => self.find_similar_to_open_thought(window, cx),
            CommandId::PostActions => self.open_actions_for_open_thought(window, cx),
            CommandId::ToggleOutline => self.toggle_inspector_outline(window, cx),
            CommandId::MakeFlashcard => self.make_flashcard_open_thought(window, cx),
        }
    }

    /// Opens the settings modal on `tab`.
    pub(crate) fn open_settings_tab(&mut self, tab: SettingsTab, window: &mut Window, cx: &mut Context<Self>) {
        if !self.chrome.is_settings_open {
            self.toggle_settings(window, cx);
        }
        self.select_settings_tab(tab, window, cx);
    }
}
