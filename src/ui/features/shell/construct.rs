//! Construction of the root view.

use super::feed_state::FeedState;
use super::graph_state::GraphState;
use super::lab_state::{LabState, LlmInputs};
use super::scheduler_state::SchedulerState;
use super::view::{AppView, ChromeState};
use crate::i18n::Language;
use crate::state::AppState;
use crate::ui::features::composer::ComposerModalPanel;
use gpui_kit::*;

impl AppView {
    /// Constructs a new AppView with default state, frameless window setup, and dark minimalist theme.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let initial_lang = Language::English;
        crate::i18n::set_locale(initial_lang);

        let state = AppState::new();
        let feed = FeedState::new(&state, window, cx);
        let profile = state.current_profile().cloned();
        let theme = state.theme_tokens.clone();
        let composer_modal = cx.new(|cx| ComposerModalPanel::new(theme, profile, window, cx));
        let (graph, graph_subscription) = GraphState::new(&state, window, cx);
        let subscriptions = vec![
            feed.subscribe_search(cx),
            cx.subscribe_in(&composer_modal, window, Self::publish_composer_draft),
            graph_subscription,
        ];
        let llm_inputs = LlmInputs::new(&state, window, cx);
        let lab = LabState::new(&state, window, cx);
        let scheduler = SchedulerState::new(state.scheduler_enabled());

        let mut view = Self {
            theme: state.theme_tokens.clone(),
            state,
            chrome: ChromeState::new(initial_lang),
            feed,
            composer_modal,
            llm_inputs,
            lab,
            scheduler,
            graph,
            _subscriptions: subscriptions,
        };
        view.start_scheduler_heartbeat(window, cx);
        view.watch_vault_failures(window, cx);
        view
    }
}
