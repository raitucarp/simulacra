//! The root window view and its window-chrome state.

use super::feed_state::FeedState;
use super::graph_state::GraphState;
use super::lab_state::{LabState, LlmInputs};
use super::scheduler_state::SchedulerState;
use crate::i18n::Language;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::composer::ComposerModalPanel;
use crate::ui::features::settings::SettingsTab;
use crate::ui::features::sidebar_right::RightSidebarTab;
use gpui_kit::{Entity, Subscription};

/// Main window root view container (title bar, three resizable columns and overlays).
pub struct AppView {
    pub state: AppState,
    pub theme: ThemeTokens,
    pub chrome: ChromeState,
    pub feed: FeedState,
    pub composer_modal: Entity<ComposerModalPanel>,
    pub llm_inputs: LlmInputs,
    pub lab: LabState,
    pub scheduler: SchedulerState,
    pub graph: GraphState,
    pub(super) _subscriptions: Vec<Subscription>,
}

/// Open/closed state of the window chrome: menus, modals and side columns.
pub struct ChromeState {
    pub current_language: Language,
    pub is_lang_menu_open: bool,
    pub is_settings_open: bool,
    pub active_settings_tab: SettingsTab,
    pub is_workspace_open: bool,
    pub is_left_sidebar_collapsed: bool,
    pub is_right_sidebar_open: bool,
    pub right_sidebar_tab: RightSidebarTab,
}

impl ChromeState {
    /// Initial chrome: expanded left column, open right column, every menu closed.
    pub(super) fn new(current_language: Language) -> Self {
        Self {
            current_language,
            is_lang_menu_open: false,
            is_settings_open: false,
            active_settings_tab: SettingsTab::AiIntelligence,
            is_workspace_open: false,
            is_left_sidebar_collapsed: false,
            is_right_sidebar_open: true,
            right_sidebar_tab: RightSidebarTab::MindCadence,
        }
    }
}
