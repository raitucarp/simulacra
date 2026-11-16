//! Settings and theme handlers: tabs, theme palette, LLM provider/model and persistence.

use super::super::view::AppView;
use crate::agent::LlmProvider;
use crate::theme::ThemeTokens;
use crate::ui::features::settings::SettingsTab;
use gpui_kit::*;

impl AppView {
    pub(crate) fn select_settings_tab(&mut self, tab: SettingsTab, _: &mut Window, cx: &mut Context<Self>) {
        self.chrome.active_settings_tab = tab;
        cx.notify();
    }

    /// Applies and persists a theme palette.
    pub(crate) fn select_theme(&mut self, new_theme: ThemeTokens, _: &mut Window, cx: &mut Context<Self>) {
        self.state.update_theme_tokens(new_theme.clone());
        self.theme = new_theme;
        cx.notify();
    }

    /// Switches provider and resets the model to the provider default.
    pub(crate) fn select_provider(&mut self, provider: LlmProvider, window: &mut Window, cx: &mut Context<Self>) {
        self.state.llm_settings.provider = provider;
        let def_model = provider.default_model();
        self.llm_inputs.model.update(cx, |st, cx| st.set_value(def_model, window, cx));
        self.state.llm_settings.model = def_model.to_string();
        cx.notify();
    }

    /// Fills the model input with a preset identifier.
    pub(crate) fn select_preset_model(&mut self, preset: &'static str, window: &mut Window, cx: &mut Context<Self>) {
        self.llm_inputs.model.update(cx, |st, cx| st.set_value(preset, window, cx));
        self.state.llm_settings.model = preset.to_string();
        cx.notify();
    }

    /// Persists the typed LLM settings (an empty model falls back to the provider default).
    pub(crate) fn save_settings(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let (key, model, endpoint) = self.llm_inputs.values(cx);
        let settings = &mut self.state.llm_settings;
        settings.api_key = key;
        settings.model = if model.trim().is_empty() {
            settings.provider.default_model().to_string()
        } else {
            model
        };
        settings.custom_endpoint = endpoint;
        self.state.update_llm_settings(self.state.llm_settings.clone());
        self.chrome.is_settings_open = false;
        cx.notify();
    }

    pub(crate) fn close_settings(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.chrome.is_settings_open = false;
        cx.notify();
    }
}
