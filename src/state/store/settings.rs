//! Theme token, LLM provider and autonomous-scheduler settings.

use super::AppState;
use crate::agent::LlmSettings;
use crate::theme::ThemeTokens;

/// Settings key persisting the global autonomous-scheduler switch.
const SCHEDULER_ENABLED_KEY: &str = "scheduler_enabled";
/// The scheduler stays off until the user turns it on, so the app never surprise-posts.
const SCHEDULER_ENABLED_DEFAULT: bool = false;

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

    /// Whether the autonomous scheduler is switched on (persisted; off by default).
    pub fn scheduler_enabled(&self) -> bool {
        self.db
            .as_ref()
            .and_then(|db| db.get_setting(SCHEDULER_ENABLED_KEY).ok().flatten())
            .and_then(|raw| raw.parse::<bool>().ok())
            .unwrap_or(SCHEDULER_ENABLED_DEFAULT)
    }

    /// Persists the autonomous-scheduler switch.
    pub fn set_scheduler_enabled(&self, enabled: bool) {
        if let Some(ref db) = self.db {
            if let Err(e) = db.set_setting(SCHEDULER_ENABLED_KEY, &enabled.to_string()) {
                eprintln!("simulacra: failed to persist scheduler switch: {e}");
            }
        }
    }
}
