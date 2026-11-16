//! LLM configuration and its persistence in the SQLite `settings` table.

use super::LlmProvider;
use crate::storage::Database;
use serde::{Deserialize, Serialize};

/// Settings-table key for the provider identifier.
pub(crate) const KEY_PROVIDER: &str = "llm_provider";
/// Settings-table key for the API key.
pub(crate) const KEY_API_KEY: &str = "llm_api_key";
/// Settings-table key for the model name.
pub(crate) const KEY_MODEL: &str = "llm_model";
/// Settings-table key for the custom endpoint URL.
pub(crate) const KEY_ENDPOINT: &str = "llm_endpoint";
/// Settings-table key for the sampling temperature.
pub(crate) const KEY_TEMPERATURE: &str = "llm_temperature";
/// Settings-table key for the max output token budget.
pub(crate) const KEY_MAX_TOKENS: &str = "llm_max_tokens";

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
    /// Loads saved LLM settings from the SQLite database.
    pub fn load_from_db(db: &Database) -> Self {
        let mut settings = Self::default();
        let get = |key: &str| db.get_setting(key).ok().flatten();

        if let Some(provider_str) = get(KEY_PROVIDER) {
            settings.provider = LlmProvider::from_id(&provider_str);
        }
        if let Some(key) = get(KEY_API_KEY) {
            settings.api_key = key;
        }
        if let Some(model) = get(KEY_MODEL) {
            settings.model = if model.trim().is_empty() {
                settings.provider.default_model().to_string()
            } else {
                model
            };
        }
        if let Some(endpoint) = get(KEY_ENDPOINT) {
            settings.custom_endpoint = endpoint;
        }
        if let Some(val) = get(KEY_TEMPERATURE).and_then(|s| s.parse::<f32>().ok()) {
            settings.temperature = val;
        }
        if let Some(val) = get(KEY_MAX_TOKENS).and_then(|s| s.parse::<u32>().ok()) {
            settings.max_tokens = val;
        }

        settings
    }

    /// Persists LLM settings to the SQLite database.
    pub fn save_to_db(&self, db: &Database) -> Result<(), String> {
        let entries = [
            (KEY_PROVIDER, self.provider.id_str().to_string()),
            (KEY_API_KEY, self.api_key.clone()),
            (KEY_MODEL, self.model.clone()),
            (KEY_ENDPOINT, self.custom_endpoint.clone()),
            (KEY_TEMPERATURE, self.temperature.to_string()),
            (KEY_MAX_TOKENS, self.max_tokens.to_string()),
        ];
        for (key, value) in &entries {
            db.set_setting(key, value).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
