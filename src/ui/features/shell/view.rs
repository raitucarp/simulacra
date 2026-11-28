//! The root window view and its window-chrome state.

use super::feed_state::FeedState;
use super::forge_host::ForgeSession;
use super::graph_state::GraphState;
use super::image_ideas_state::ImageIdeasUi;
use super::import_state::ImportUi;
use super::inspector_state::InspectorState;
use super::lab_state::{LabState, LlmInputs};
use super::media_state::MediaHost;
use super::panels::ShellPanels;
use super::post_actions_state::PostActionsUi;
use super::scheduler_state::SchedulerState;
use super::recommend_host::RecommendHost;
use super::semantic_state::SemanticState;
use super::settings_draft::SettingsDraft;
use super::settings_ui::SettingsUi;
use super::vault_host::VaultHost;
use super::review_host::ReviewHost;
use super::insights_host::InsightsHost;
use super::journal_host::JournalHost;
use super::arena_host::ArenaHost;
use super::thinking_host::ThinkingHost;
use super::data_host::DataHost;
use crate::i18n::Language;
use crate::state::{AppState, NavigationTab};
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
    /// The "For you" order and the cached recommendations from post embeddings.
    pub recommend: RecommendHost,
    /// The lightbox and the media pump's last plan (sb C3).
    pub media: MediaHost,
    /// The review deck, the Review page and the card maker (sb C1).
    pub review: ReviewHost,
    /// The Insights page and its background report (sb D2).
    pub insights: InsightsHost,
    /// The private Journal page and its records (sb E3).
    pub journal: JournalHost,
    /// Debates, thought experiments and agent beliefs (sb F1).
    pub arena: ArenaHost,
    /// The Agent Forge while it is open (it replaces the center and right columns).
    pub forge: Option<ForgeSession>,
    /// Left sidebar prefs (width, sections, pins, recents) and the window width.
    pub sidebar: SidebarState,
    /// Command palette, its history and the shortcut sheet.
    pub palette: PaletteHost,
    /// sb C2: the post actions sheet and the inspector's cards / outline view.
    pub post_actions: PostActionsUi,
    pub inspector: InspectorState,
    /// sb D3: the image ideas sheet.
    pub image_ideas: ImageIdeasUi,
    /// sb E1: argument maps, credences, tensions and the atomicity sheet.
    pub thinking: ThinkingHost,
    /// Epic R: the emoji picker and what it remembers.
    pub reactions: super::reactions_host::ReactionsHost,
    /// Epic S: what the symbol picker remembers.
    pub symbols: super::symbols_host::SymbolsHost,
    /// WS5: illustration pack installs, downloads and removals.
    pub packs: super::packs_host::PacksHost,
    /// sb F2: history, export, snapshots, collections and bookmarks.
    pub data: DataHost,
    /// sb P16: what "Ask before spending" allowed this session.
    pub spend: super::spend_host::SpendHost,
    /// D9: what the Tasks page is narrowed to.
    pub tasks: super::tasks_host::TasksHost,
    /// Q4: the Search page, made when it first opens.
    pub search: Option<super::search_host::SearchHost>,
    /// Epic P: Settings › Code (the runtime detection).
    pub code: super::code_host::CodeHost,
    /// Cached views of the sidebars and the center column.
    pub(super) panels: ShellPanels,
    pub(super) _subscriptions: Vec<Subscription>,
}

/// Open/closed state of the window chrome: menus, modals and side columns.
pub struct ChromeState {
    pub current_language: Language,
    pub is_settings_open: bool,
    pub active_settings_tab: SettingsTab,
    /// Unsaved provider/model picks of the open settings modal.
    pub settings_draft: SettingsDraft,
    /// The network menu, opened from the switcher or the "Switch network" command (M1).
    pub workspace_menu: crate::ui::menus::OpenMenu,
    /// The right panel column per place where the user toggled it; elsewhere it follows the
    /// default (see `AppView::is_right_panel_shown`).
    pub right_panel_choices: Vec<(NavigationTab, bool)>,
    /// The right panel's drawer is open (compact layout class, where it is not a column).
    pub is_right_drawer_open: bool,
    pub right_sidebar_tab: RightSidebarTab,
    /// The first-run welcome overlay is showing.
    pub is_onboarding_open: bool,
}

impl ChromeState {
    /// Initial chrome: the right column at its defaults, every menu closed.
    pub(super) fn new(current_language: Language) -> Self {
        Self {
            current_language,
            is_settings_open: false,
            active_settings_tab: SettingsTab::General,
            settings_draft: SettingsDraft::default(),
            workspace_menu: Default::default(),
            right_panel_choices: Vec::new(),
            is_right_drawer_open: false,
            right_sidebar_tab: RightSidebarTab::MindCadence,
            is_onboarding_open: false,
        }
    }
}
