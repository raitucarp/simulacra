//! Theme token and LLM provider settings.

use super::AppState;
use crate::agent::LlmSettings;
use crate::theme::ThemeTokens;

impl AppState {
    /// Updates and persists dynamic theme design tokens to SQLite.
    pub fn update_theme_tokens(&mut self, tokens: ThemeTokens) {
        if let Some(ref db) = self.db {
            let _ = db.save_theme_tokens(&tokens);
        }
        self.theme_tokens = tokens;
    }

    /// Updates and persists LLM Provider settings.
    pub fn update_llm_settings(&mut self, settings: LlmSettings) {
        if let Some(ref database) = self.db {
            let _ = settings.save_to_db(database);
        }
        self.llm_settings = settings;
    }
}
