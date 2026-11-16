//! Construction of the root view.

use super::composer_state::ComposerModalState;
use super::feed_state::FeedState;
use super::lab_state::{LabState, LlmInputs};
use super::view::{AppView, ChromeState};
use crate::i18n::Language;
use crate::state::AppState;
use gpui_kit::*;

impl AppView {
    /// Constructs a new AppView with default state, frameless window setup, and dark minimalist theme.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let initial_lang = Language::English;
        crate::i18n::set_locale(initial_lang);

        let state = AppState::new();
        let feed = FeedState::new(&state, window, cx);
        let composer_modal = ComposerModalState::new(window, cx);
        let mut subscriptions = vec![feed.subscribe_search(cx)];
        subscriptions.extend(composer_modal.subscribe(window, cx));
        let llm_inputs = LlmInputs::new(&state, window, cx);
        let lab = LabState::new(&state, window, cx);

        Self {
            theme: state.theme_tokens.clone(),
            state,
            chrome: ChromeState::new(initial_lang),
            feed,
            composer_modal,
            llm_inputs,
            lab,
            _subscriptions: subscriptions,
        }
    }
}
