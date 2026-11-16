//! Theme token, LLM provider and autonomous-scheduler settings.

use super::AppState;
use crate::agent::LlmSettings;
use crate::error::{AppError, Result};
use crate::theme::ThemeTokens;

/// Settings key persisting the global autonomous-scheduler switch.
const SCHEDULER_ENABLED_KEY: &str = "scheduler_enabled";
/// The scheduler stays off until the user turns it on, so the app never surprise-posts.
const SCHEDULER_ENABLED_DEFAULT: bool = false;

/// Logs a write the vault worker could not even accept (its thread is gone).
fn report_unqueued(result: Result<()>) {
    if let Err(err) = result {
        eprintln!("simulacra: vault write not queued: {err}");
    }
}

impl AppState {
    /// Updates dynamic theme design tokens and queues them for the vault (a failed write
    /// is reported through the vault's failure channel).
    pub fn update_theme_tokens(&mut self, tokens: ThemeTokens) {
        if let Some(ref db) = self.db {
            report_unqueued(db.save_theme_tokens(&tokens));
        }
        self.theme_tokens = tokens;
    }

    /// Updates LLM Provider settings and queues them for the vault.
    pub fn update_llm_settings(&mut self, settings: LlmSettings) {
        if let Some(ref db) = self.db {
            let saved = settings.clone();
            report_unqueued(db.write(move |db| saved.save_to_db(db).map_err(AppError::DatabaseError)));
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
