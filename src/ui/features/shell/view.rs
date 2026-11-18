//! The root window view and its window-chrome state.

use super::feed_state::FeedState;
use super::forge_host::ForgeSession;
use super::graph_state::GraphState;
use super::import_state::ImportUi;
use super::lab_state::{LabState, LlmInputs};
use super::panels::ShellPanels;
use super::scheduler_state::SchedulerState;
use super::semantic_state::SemanticState;
use super::settings_draft::SettingsDraft;
use super::settings_ui::SettingsUi;
use super::vault_host::VaultHost;
use super::review_host::ReviewHost;
use crate::i18n::Language;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::command_palette::PaletteHost;
use crate::ui::features::composer::ComposerModalPanel;
use crate::ui::features::settings::{AppearancePanel, SettingsTab};
use crate::ui::features::sidebar_left::SidebarState;
use crate::ui::features::sidebar_right::RightSidebarTab;
use gpui_kit::{Entity, Subscription};

/// Main window root view container (title bar, three resizable columns and overlays).
pub struct AppView {
    pub state: AppState,
    pub theme: ThemeTokens,
    pub chrome: ChromeState,
    pub feed: FeedState,
    pub composer_modal: Entity<ComposerModalPanel>,
    /// Appearance settings pane (theme gallery and editor).
    pub appearance: Entity<AppearancePanel>,
    pub llm_inputs: LlmInputs,
    /// Search box, text inputs and flags of the settings window.
    pub settings_ui: SettingsUi,
    pub lab: LabState,
    pub scheduler: SchedulerState,
    pub graph: GraphState,
    /// The Knowledge Vault view.
    pub vault: VaultHost,
    /// The import sheet, quick capture and file-drop state.
    pub import: ImportUi,
    /// Vector index, indexing queue and the "find similar" sheet (sb B3).
    pub semantic: SemanticState,
    /// The review deck, the Review page and the card maker (sb C1).
    pub review: ReviewHost,
    /// The Agent Forge while it is open (it replaces the center and right columns).
    pub forge: Option<ForgeSession>,
    /// Left sidebar prefs (width, sections, pins, recents) and the window width.
    pub sidebar: SidebarState,
    /// Command palette, its history and the shortcut sheet.
    pub palette: PaletteHost,
    /// Cached views of the sidebars and the center column.
    pub(super) panels: ShellPanels,
    pub(super) _subscriptions: Vec<Subscription>,
}

/// Open/closed state of the window chrome: menus, modals and side columns.
pub struct ChromeState {
    pub current_language: Language,
    pub is_lang_menu_open: bool,
    pub is_settings_open: bool,
    pub active_settings_tab: SettingsTab,
    /// Unsaved provider/model picks of the open settings modal.
    pub settings_draft: SettingsDraft,
    pub is_workspace_open: bool,
    pub is_right_sidebar_open: bool,
    /// The right panel's drawer is open (compact layout class, where it is not a column).
    pub is_right_drawer_open: bool,
    pub right_sidebar_tab: RightSidebarTab,
    /// The first-run welcome overlay is showing.
    pub is_onboarding_open: bool,
}

impl ChromeState {
    /// Initial chrome: open right column, every menu closed.
    pub(super) fn new(current_language: Language) -> Self {
        Self {
            current_language,
            is_lang_menu_open: false,
            is_settings_open: false,
            active_settings_tab: SettingsTab::General,
            settings_draft: SettingsDraft::default(),
            is_workspace_open: false,
            is_right_sidebar_open: true,
            is_right_drawer_open: false,
            right_sidebar_tab: RightSidebarTab::MindCadence,
            is_onboarding_open: false,
        }
    }
}
