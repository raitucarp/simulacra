//! Construction of the root view.

use super::feed_state::FeedState;
use super::graph_state::GraphState;
use super::image_ideas_state::ImageIdeasUi;
use super::import_state::ImportUi;
use super::inspector_state::InspectorState;
use super::lab_state::{LabState, LlmInputs};
use super::media_state::MediaHost;
use super::panels::ShellPanels;
use super::post_actions_state::PostActionsUi;
use super::scheduler_state::SchedulerState;
use super::semantic_state::SemanticState;
use super::settings_ui::SettingsUi;
use super::vault_host::VaultHost;
use super::review_host::ReviewHost;
use super::insights_host::InsightsHost;
use super::journal_host::JournalHost;
use super::thinking_host::ThinkingHost;
use super::data_host::DataHost;
use super::view::{AppView, ChromeState};
use crate::state::AppState;
use crate::ui::features::command_palette::PaletteHost;
use crate::ui::features::composer::ComposerModalPanel;
use crate::ui::features::settings::AppearancePanel;
use crate::ui::features::sidebar_left::SidebarState;
use gpui_kit::*;

/// Interface language at startup.

impl AppView {
    /// Constructs a new AppView, loading the store from the default vault synchronously.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self::with_state(AppState::new(), window, cx)
    }

    /// Constructs the AppView around an already loaded store (the splash screen loads it in
    /// stages): points the render globals at it, restores the saved panel layouts and builds
    /// every section.
    pub fn with_state(mut state: AppState, window: &mut Window, cx: &mut Context<Self>) -> Self {
        super::handlers::appearance::resolve_startup_appearance(&mut state, window);
        let initial_lang = state.preferences().general.language;
        register_render_globals(&state);
        crate::ui::components::persisted_split::init_panel_layouts(state.db.as_ref(), cx);
        let feed = FeedState::new(&state, window, cx);
        let profile = state.current_profile().cloned();
        let composer_modal = cx.new(|cx| ComposerModalPanel::new(state.theme_tokens.clone(), profile, window, cx));
        let ((graph, graph_subscription), (vault, vault_subscription)) = (GraphState::new(&state, window, cx), VaultHost::new(&state, window, cx));
        let appearance = cx.new(|cx| AppearancePanel::new(state.theme_settings.clone(), window, cx));
        let (settings_ui, settings_subscriptions) = SettingsUi::new(state.preferences(), window, cx);
        let (import, import_subscriptions) = ImportUi::new(window, cx);
        let ((review, review_subscriptions), (insights, insights_subscription), (journal, journal_subscription), (thinking, thinking_subscription)) = (ReviewHost::new(&state, window, cx), InsightsHost::new(&state, window, cx), JournalHost::new(&state, window, cx), ThinkingHost::new(&state, window, cx));
        let mut subscriptions = vec![
            feed.subscribe_search(cx),
            cx.subscribe_in(&composer_modal, window, Self::publish_composer_draft),
            graph_subscription, vault_subscription, insights_subscription, journal_subscription, thinking_subscription,
            cx.subscribe_in(&appearance, window, Self::on_appearance_event),
            Self::observe_own_changes(cx), Self::observe_window_width(window, cx), Self::refocus_on_focus_loss(window, cx),
            cx.observe_window_appearance(window, |this, window, cx| this.follow_system_appearance(window, cx)),
        ];
        subscriptions.extend(settings_subscriptions.into_iter().chain(import_subscriptions).chain(review_subscriptions));
        let llm_inputs = LlmInputs::new(&state, window, cx);
        let (lab, onboarding, data) = (LabState::new(&state), state.should_offer_onboarding(), DataHost::new(&state, window, cx)); // sb F2: data
        let scheduler = SchedulerState::new(state.scheduler_enabled(), state.preferences().agents.max_concurrent as usize);
        let (sidebar, palette) = (SidebarState::load(state.db.as_ref(), window.viewport_size().width), PaletteHost::new(state.db.as_ref(), cx));
        let mut view = Self {
            theme: state.theme_tokens.clone(), post_actions: PostActionsUi::new(window, cx), image_ideas: ImageIdeasUi::new(window, cx), inspector: InspectorState::load(state.db.as_ref(), cx),
            state,
            chrome: ChromeState { is_onboarding_open: onboarding, ..ChromeState::new(initial_lang) },
            feed, composer_modal, appearance, llm_inputs, settings_ui,
            lab, scheduler, graph, vault, import, review, insights, journal, thinking, data, semantic: SemanticState::new(cx), media: MediaHost::new(cx),
            forge: None, sidebar, palette,
            panels: ShellPanels::new(cx),
            _subscriptions: subscriptions,
        };
        view.start_background_work(window, cx);
        view.open_startup_screen(window, cx);
        view
    }

    /// Shows the startup screen chosen in Settings › General.
    fn open_startup_screen(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let screen = self.state.preferences().general.startup_screen;
        let tab = crate::state::NavigationTab::of_startup(screen);
        if tab != self.state.active_tab {
            self.select_nav(tab, window, cx);
        }
    }

    /// The long-running tasks every window needs (key bindings are the central keymap's).
    fn start_background_work(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        crate::ui::features::import::keys::init_key_bindings(cx);
        crate::ui::features::feed::inspector_outline::init_key_bindings(cx);
        self.import_command_line_files(window, cx);
        self.start_scheduler_heartbeat(window, cx);
        self.watch_vault_failures(window, cx);
    }
}

/// Points the render-side globals at the store: the interface language, the vault folder
/// (relative avatar photos) and the active palette (themed illustrations).
fn register_render_globals(state: &AppState) {
    crate::i18n::set_locale(state.preferences().general.language);
    crate::storage::avatars::register_vault_dir(state.vault_dir());
    let _ = crate::ui::illustration::set_active_palette(&state.theme_settings.effective_palette().colors);
}
