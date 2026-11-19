//! Settings and theme handlers: tabs, theme palette, LLM provider/model and persistence.
//! Provider and model picks live in `chrome.settings_draft` until Save; closing reverts them.

use super::super::view::AppView;
use crate::agent::LlmProvider;
use crate::theme::ThemeSettings;
use crate::ui::features::settings::{AppearanceEvent, AppearancePanel, SettingsTab};
use crate::domain::preferences::ToastKind;
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Shows a category (from the rail), clearing the search.
    pub(crate) fn select_settings_tab(&mut self, tab: SettingsTab, window: &mut Window, cx: &mut Context<Self>) {
        self.open_settings_section(tab, window, cx);
    }

    /// Applies every theme change made in the Appearance pane.
    pub(crate) fn on_appearance_event(
        &mut self,
        _: &Entity<AppearancePanel>,
        event: &AppearanceEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let AppearanceEvent::Changed(settings) = event;
        self.apply_theme_settings(settings.clone(), cx);
    }

    /// Persists `settings`, re-derives the tokens and pushes them to every view and the kit.
    pub(crate) fn apply_theme_settings(&mut self, settings: ThemeSettings, cx: &mut Context<Self>) {
        self.state.update_theme_settings(settings);
        self.theme = self.state.theme_tokens.clone();
        crate::theme::sync_kit_theme(&self.theme, cx);
        let palette = self.state.theme_settings.effective_palette();
        for stale in crate::ui::illustration::set_active_palette(&palette.colors) {
            stale.remove_asset(cx);
        }
        self.feed.stream.remeasure();
        cx.notify();
    }

    /// Picks a provider in the draft and resets the model input to the provider default.
    pub(crate) fn select_provider(&mut self, provider: LlmProvider, window: &mut Window, cx: &mut Context<Self>) {
        let def_model = self.chrome.settings_draft.select_provider(&self.state.llm_settings, provider);
        self.llm_inputs.model.update(cx, |st, cx| st.set_value(def_model, window, cx));
        cx.notify();
    }

    /// Fills the model input with a preset identifier (draft only).
    pub(crate) fn select_preset_model(&mut self, preset: &'static str, window: &mut Window, cx: &mut Context<Self>) {
        self.llm_inputs.model.update(cx, |st, cx| st.set_value(preset, window, cx));
        self.chrome.settings_draft.select_model(&self.state.llm_settings, preset);
        cx.notify();
    }

    /// Persists the draft merged with the typed fields (an empty model falls back to the default)
    /// and forgets past LLM failures, so the scheduler retries and reports afresh. The window
    /// stays open.
    pub(crate) fn save_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let typed = self.llm_inputs.values(cx);
        let settings = self.chrome.settings_draft.apply(&self.state.llm_settings, typed);
        let mut credentials = self.state.credentials.clone();
        credentials.sync_default(&settings);
        self.state.update_llm_settings(settings);
        self.state.update_credentials(credentials);
        self.reset_llm_failures();
        let note = Notification::success(t!("prefs.ui.ai_saved").to_string());
        self.toast(ToastKind::Confirmation, note, window, cx);
        cx.notify();
    }

    /// Closes the modal, discarding unsaved provider/model picks and typed values.
    pub(crate) fn close_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.revert_settings_draft(window, cx);
        self.settings_ui.confirm_reset = false;
        self.chrome.is_settings_open = false;
        cx.notify();
    }

    /// Drops the draft and refills the inputs from the persisted settings.
    pub(crate) fn revert_settings_draft(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.chrome.settings_draft.discard();
        let saved = &self.state.llm_settings;
        let fields = [
            (&self.llm_inputs.api_key, saved.api_key.clone()),
            (&self.llm_inputs.model, saved.model.clone()),
            (&self.llm_inputs.endpoint, saved.custom_endpoint.clone()),
        ];
        for (input, value) in fields {
            input.update(cx, |st, cx| st.set_value(value, window, cx));
        }
    }
}
