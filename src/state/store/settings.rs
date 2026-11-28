//! Theme, LLM provider and autonomous-scheduler settings.

use super::AppState;
use crate::agent::LlmSettings;
use crate::error::{AppError, Result};
use crate::theme::ThemeSettings;

/// Logs a write the vault worker could not even accept (its thread is gone).
fn report_unqueued(result: Result<()>) {
    if let Err(err) = result {
        eprintln!("simulacra: vault write not queued: {err}");
    }
}

impl AppState {
    /// Replaces the theme settings, re-derives the tokens and queues the settings for the
    /// vault (a failed write is reported through the vault's failure channel).
    pub fn update_theme_settings(&mut self, settings: ThemeSettings) {
        if let Some(ref db) = self.db {
            report_unqueued(db.save_theme_settings(&settings));
        }
        self.theme_tokens = settings.tokens();
        self.theme_settings = settings;
    }

    /// Updates LLM Provider settings and queues them for the vault.
    pub fn update_llm_settings(&mut self, settings: LlmSettings) {
        if let Some(ref db) = self.db {
            let saved = settings.clone();
            report_unqueued(db.write(move |db| saved.save_to_db(db).map_err(AppError::DatabaseError)));
        }
        self.llm_settings = settings;
    }
}

impl AppState {
    /// Replaces the credential store and queues it for the vault.
    pub fn update_credentials(&mut self, credentials: crate::agent::credentials::CredentialStore) {
        if let Some(ref db) = self.db {
            let saved = credentials.clone();
            report_unqueued(db.write(move |db| saved.save_to_db(db).map_err(AppError::DatabaseError)));
        }
        self.credentials = credentials;
    }

    /// Folder of the downloaded icon sets (next to the vault, S12).
    pub fn symbols_dir(&self) -> std::path::PathBuf {
        crate::icons::symbols_dir_for(&self.db.as_ref().map(|db| db.path()).unwrap_or_default())
    }

    /// Folder of the installed illustration packs (next to the vault, WS5).
    pub fn packs_dir(&self) -> std::path::PathBuf {
        crate::packs::packs_dir_for(&self.db.as_ref().map(|db| db.path()).unwrap_or_default())
    }

    /// Folder where generated agent images are saved (next to the vault).
    pub fn media_dir(&self) -> std::path::PathBuf {
        crate::storage::media::media_dir_for(&self.db.as_ref().map(|db| db.path()).unwrap_or_default())
    }
}
