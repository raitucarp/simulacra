//! Unsaved AI-provider choices of the open settings modal: they only reach the store on Save.

use crate::agent::credentials::{CredentialKind, CredentialStore};
use crate::agent::{LlmProvider, LlmSettings};

/// `(api key, model, endpoint)` as typed into the settings inputs.
pub type TypedLlmFields = (String, String, String);

/// Provider and preset-model picks made since the settings modal opened.
#[derive(Debug, Clone, Default)]
pub struct SettingsDraft {
    draft: Option<LlmSettings>,
}

impl SettingsDraft {
    /// The settings the modal shows: the draft once edited, otherwise the persisted ones.
    pub fn view(&self, saved: &LlmSettings) -> LlmSettings {
        self.draft.clone().unwrap_or_else(|| saved.clone())
    }

    /// Whether anything was picked since the modal opened.
    pub fn is_dirty(&self) -> bool {
        self.draft.is_some()
    }

    fn edit(&mut self, saved: &LlmSettings) -> &mut LlmSettings {
        self.draft.get_or_insert_with(|| saved.clone())
    }

    /// Picks `provider` with its default model; returns that model for the model input.
    pub fn select_provider(&mut self, saved: &LlmSettings, provider: LlmProvider) -> &'static str {
        let model = provider.default_model();
        let draft = self.edit(saved);
        draft.provider = provider;
        draft.model = model.to_string();
        model
    }

    /// Picks a preset model identifier.
    pub fn select_model(&mut self, saved: &LlmSettings, model: &str) {
        self.edit(saved).model = model.to_string();
    }

    /// The settings to persist: the draft (or saved settings) with the typed fields merged in
    /// (an empty model falls back to the provider default). Clears the draft.
    pub fn apply(&mut self, saved: &LlmSettings, typed: TypedLlmFields) -> LlmSettings {
        let (api_key, model, endpoint) = typed;
        let mut settings = self.draft.take().unwrap_or_else(|| saved.clone());
        settings.api_key = api_key;
        settings.model = if model.trim().is_empty() {
            settings.provider.default_model().to_string()
        } else {
            model
        };
        settings.custom_endpoint = endpoint;
        settings
    }

    /// Whether saving now would change `saved` (a pick or a typed value differs).
    pub fn is_unsaved(&self, saved: &LlmSettings, typed: TypedLlmFields) -> bool {
        self.clone().apply(saved, typed) != *saved
    }

    /// Forgets every unsaved pick.
    pub fn discard(&mut self) {
        self.draft = None;
    }
}

/// The key the API key input shows once `provider` is picked: the saved key for the saved
/// provider, otherwise the first stored credential of that service (blank without one), so
/// one service's key never sits under another's name.
pub fn provider_key(saved: &LlmSettings, credentials: &CredentialStore, provider: LlmProvider) -> String {
    if provider == saved.provider {
        return saved.api_key.clone();
    }
    credentials.first_of(CredentialKind::from_provider(provider)).map(|stored| stored.secret.clone()).unwrap_or_default()
}
