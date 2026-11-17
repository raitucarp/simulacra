//! LLM configuration and its persistence in the SQLite `settings` table.

use super::LlmProvider;
use crate::storage::{Database, SettingKey};
use serde::{Deserialize, Serialize};

/// Default sampling temperature.
pub(crate) const DEFAULT_TEMPERATURE: f32 = 0.7;
/// Default max output token budget.
pub(crate) const DEFAULT_MAX_TOKENS: u32 = 300;

/// LLM Configuration Settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmSettings {
    pub provider: LlmProvider,
    pub api_key: String,
    pub model: String,
    pub custom_endpoint: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for LlmSettings {
    fn default() -> Self {
        let provider = LlmProvider::default();
        Self {
            provider,
            api_key: String::new(),
            model: provider.default_model().to_string(),
            custom_endpoint: String::new(),
            temperature: DEFAULT_TEMPERATURE,
            max_tokens: DEFAULT_MAX_TOKENS,
        }
    }
}

impl LlmSettings {
    /// Whether an API key is configured (blank keys do not count).
    pub fn has_api_key(&self) -> bool {
        !self.api_key.trim().is_empty()
    }

    /// The model id, or the provider default when blank.
    pub fn model_or_default(&self) -> &str {
        if self.model.trim().is_empty() {
            self.provider.default_model()
        } else {
            self.model.trim()
        }
    }

    /// Loads saved LLM settings from the SQLite database.
    pub fn load_from_db(db: &Database) -> Self {
        let mut settings = Self::default();
        let get = |key: SettingKey| db.get_setting(key).ok().flatten();

        if let Some(provider_str) = get(SettingKey::LlmProvider) {
            settings.provider = LlmProvider::from_id(&provider_str);
        }
        if let Some(key) = get(SettingKey::LlmApiKey) {
            settings.api_key = key;
        }
        if let Some(model) = get(SettingKey::LlmModel) {
            settings.model = if model.trim().is_empty() {
                settings.provider.default_model().to_string()
            } else {
                model
            };
        }
        if let Some(endpoint) = get(SettingKey::LlmEndpoint) {
            settings.custom_endpoint = endpoint;
        }
        if let Some(val) = get(SettingKey::LlmTemperature).and_then(|s| s.parse::<f32>().ok()) {
            settings.temperature = val;
        }
        if let Some(val) = get(SettingKey::LlmMaxTokens).and_then(|s| s.parse::<u32>().ok()) {
            settings.max_tokens = val;
        }

        settings
    }

    /// Persists LLM settings to the SQLite database.
    pub fn save_to_db(&self, db: &Database) -> Result<(), String> {
        let entries = [
            (SettingKey::LlmProvider, self.provider.id_str().to_string()),
            (SettingKey::LlmApiKey, self.api_key.clone()),
            (SettingKey::LlmModel, self.model.clone()),
            (SettingKey::LlmEndpoint, self.custom_endpoint.clone()),
            (SettingKey::LlmTemperature, self.temperature.to_string()),
            (SettingKey::LlmMaxTokens, self.max_tokens.to_string()),
        ];
        for (key, value) in &entries {
            db.set_setting(*key, value).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
