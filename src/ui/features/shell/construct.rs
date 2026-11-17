//! Construction of the root view.

use super::feed_state::FeedState;
use super::graph_state::GraphState;
use super::lab_state::{LabState, LlmInputs};
use super::panels::ShellPanels;
use super::scheduler_state::SchedulerState;
use super::settings_ui::SettingsUi;
use super::view::{AppView, ChromeState};
use crate::state::AppState;
use crate::ui::features::composer::ComposerModalPanel;
use crate::ui::features::settings::AppearancePanel;
use gpui_kit::*;

impl AppView {
    /// Constructs a new AppView, loading the store from the default vault synchronously.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self::with_state(AppState::new(), window, cx)
    }

    /// Constructs the AppView around an already loaded store (the splash screen loads it in
    /// stages): points the render globals at it, restores the saved panel layouts and builds
    /// every section.
    pub fn with_state(state: AppState, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let initial_lang = state.preferences().general.language;
        register_render_globals(&state);
        crate::ui::components::persisted_split::init_panel_layouts(state.db.as_ref(), cx);
        let feed = FeedState::new(&state, window, cx);
        let profile = state.current_profile().cloned();
        let theme = state.theme_tokens.clone();
        let composer_modal = cx.new(|cx| ComposerModalPanel::new(theme, profile, window, cx));
        let (graph, graph_subscription) = GraphState::new(&state, window, cx);
        let appearance = cx.new(|cx| AppearancePanel::new(state.theme_settings.clone(), window, cx));
        let (settings_ui, settings_subscriptions) = SettingsUi::new(state.preferences(), window, cx);
        let mut subscriptions = vec![
            feed.subscribe_search(cx),
            cx.subscribe_in(&composer_modal, window, Self::publish_composer_draft),
            graph_subscription,
            cx.subscribe_in(&appearance, window, Self::on_appearance_event),
            Self::observe_own_changes(cx),
        ];
        subscriptions.extend(settings_subscriptions);
        let llm_inputs = LlmInputs::new(&state, window, cx);
        let (lab, onboarding) = (LabState::new(&state), state.should_offer_onboarding());
        let scheduler = SchedulerState::new(state.scheduler_enabled(), state.preferences().agents.max_concurrent as usize);
        let mut view = Self {
            theme: state.theme_tokens.clone(),
            state,
            chrome: ChromeState { is_onboarding_open: onboarding, ..ChromeState::new(initial_lang) },
            feed, composer_modal, appearance, llm_inputs, settings_ui,
            lab, scheduler, graph,
            forge: None,
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

    /// Key bindings and the long-running tasks every window needs.
    fn start_background_work(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        crate::ui::features::forge::view::init_key_bindings(cx);
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
