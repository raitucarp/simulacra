//! Construction of the root view.

use super::feed_state::FeedState;
use super::graph_state::GraphState;
use super::lab_state::{LabState, LlmInputs};
use super::panels::ShellPanels;
use super::scheduler_state::SchedulerState;
use super::view::{AppView, ChromeState};
use crate::i18n::Language;
use crate::state::AppState;
use crate::ui::features::composer::ComposerModalPanel;
use crate::ui::features::settings::AppearancePanel;
use gpui_kit::*;

impl AppView {
    /// Constructs a new AppView with default state, frameless window setup, and dark minimalist theme.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let initial_lang = Language::English;
        crate::i18n::set_locale(initial_lang);

        let state = load_state();
        let feed = FeedState::new(&state, window, cx);
        let profile = state.current_profile().cloned();
        let theme = state.theme_tokens.clone();
        let composer_modal = cx.new(|cx| ComposerModalPanel::new(theme, profile, window, cx));
        let (graph, graph_subscription) = GraphState::new(&state, window, cx);
        let appearance = cx.new(|cx| AppearancePanel::new(state.theme_settings.clone(), window, cx));
        let subscriptions = vec![
            feed.subscribe_search(cx),
            cx.subscribe_in(&composer_modal, window, Self::publish_composer_draft),
            graph_subscription,
            cx.subscribe_in(&appearance, window, Self::on_appearance_event),
            Self::observe_own_changes(cx),
        ];
        let llm_inputs = LlmInputs::new(&state, window, cx);
        let lab = LabState::new(&state);
        let scheduler = SchedulerState::new(state.scheduler_enabled());

        let mut view = Self {
            theme: state.theme_tokens.clone(),
            state,
            chrome: ChromeState::new(initial_lang),
            feed,
            composer_modal,
            appearance,
            llm_inputs,
            lab,
            scheduler,
            graph,
            forge: None,
            panels: ShellPanels::new(cx),
            _subscriptions: subscriptions,
        };
        view.start_background_work(window, cx);
        view
    }

    /// Key bindings and the long-running tasks every window needs.
    fn start_background_work(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        crate::ui::features::forge::view::init_key_bindings(cx);
        self.start_scheduler_heartbeat(window, cx);
        self.watch_vault_failures(window, cx);
    }
}

/// Opens the store and points the render-side globals at it: the vault folder (relative
/// avatar photos) and the active palette (themed illustrations).
fn load_state() -> AppState {
    let state = AppState::new();
    crate::storage::avatars::register_vault_dir(state.vault_dir());
    let _ = crate::ui::illustration::set_active_palette(&state.theme_settings.effective_palette().colors);
    state
}
