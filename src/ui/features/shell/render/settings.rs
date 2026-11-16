//! Settings modal wiring.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::settings::SettingsModal;

impl AppView {
    /// Settings modal, while open.
    pub(super) fn settings_modal(&self, bind: &Binder) -> Option<SettingsModal> {
        self.chrome.is_settings_open.then(|| {
            SettingsModal::new(
                self.theme.clone(),
                self.state.llm_settings.clone(),
                self.chrome.active_settings_tab,
                self.llm_inputs.api_key.clone(),
                self.llm_inputs.model.clone(),
                self.llm_inputs.endpoint.clone(),
                bind.with(Self::select_settings_tab),
                bind.with(Self::select_theme),
                bind.with(Self::select_provider),
                bind.with(Self::select_preset_model),
                bind.act(Self::save_settings),
                bind.act(Self::close_settings),
            )
        })
    }
}
