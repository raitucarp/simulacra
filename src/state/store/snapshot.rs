//! Everything the store reads from the vault at startup, gathered in one worker round trip.

use super::bootstrap::StoreData;
use super::{ACTIVE_NETWORK_KEY, ACTIVE_PROFILE_KEY};
use crate::agent::LlmSettings;
use crate::error::Result;
use crate::storage::Database;
use crate::theme::ThemeTokens;

/// Persisted settings that do not depend on the store contents.
pub(super) struct StoredSettings {
    pub llm: LlmSettings,
    pub theme: Option<ThemeTokens>,
}

impl StoredSettings {
    /// Reads the LLM settings and theme tokens (absent or unreadable tokens give `None`).
    pub fn read(db: &Database) -> Self {
        Self { llm: LlmSettings::load_from_db(db), theme: db.load_theme_tokens().unwrap_or(None) }
    }
}

/// A non-empty vault's collections, settings and persisted selections.
pub(super) struct VaultSnapshot {
    pub data: StoreData,
    pub settings: StoredSettings,
    pub active_network: Option<String>,
    pub active_profile: Option<String>,
}

impl VaultSnapshot {
    /// Reads the snapshot on the worker thread; `None` for an empty (never seeded) vault.
    pub fn read(db: &Database) -> Result<Option<Self>> {
        if db.is_empty()? {
            return Ok(None);
        }
        Ok(Some(Self {
            data: StoreData::load(db)?,
            settings: StoredSettings::read(db),
            active_network: db.get_setting(ACTIVE_NETWORK_KEY).unwrap_or(None),
            active_profile: db.get_setting(ACTIVE_PROFILE_KEY).unwrap_or(None),
        }))
    }
}
