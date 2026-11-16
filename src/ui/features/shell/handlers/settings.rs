//! Settings and theme handlers: tabs, theme palette, LLM provider/model and persistence.
//! Provider and model picks live in `chrome.settings_draft` until Save; closing reverts them.

use super::super::view::AppView;
use crate::agent::LlmProvider;
use crate::theme::ThemeSettings;
use crate::ui::features::settings::SettingsTab;
use gpui_kit::*;

impl AppView {
    pub(crate) fn select_settings_tab(&mut self, tab: SettingsTab, _: &mut Window, cx: &mut Context<Self>) {
        self.chrome.active_settings_tab = tab;
        cx.notify();
    }

    /// Activates and persists the palette `palette_id`.
    pub(crate) fn select_theme(&mut self, palette_id: String, _: &mut Window, cx: &mut Context<Self>) {
        let mut settings = self.state.theme_settings.clone();
        settings.select(&palette_id);
        self.apply_theme_settings(settings, cx);
    }

    /// Persists `settings`, re-derives the tokens and pushes them to every view and the kit.
    pub(crate) fn apply_theme_settings(&mut self, settings: ThemeSettings, cx: &mut Context<Self>) {
        self.state.update_theme_settings(settings);
        self.theme = self.state.theme_tokens.clone();
        crate::theme::sync_kit_theme(&self.theme, cx);
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

    /// Persists the draft merged with the typed fields (an empty model falls back to the default).
    pub(crate) fn save_settings(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let typed = self.llm_inputs.values(cx);
        let settings = self.chrome.settings_draft.apply(&self.state.llm_settings, typed);
        self.state.update_llm_settings(settings);
        self.chrome.is_settings_open = false;
        cx.notify();
    }

    /// Closes the modal, discarding unsaved provider/model picks and typed values.
    pub(crate) fn close_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.revert_settings_draft(window, cx);
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
