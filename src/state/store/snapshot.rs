//! Everything the store reads from the vault at startup, gathered in one worker round trip.

use super::bootstrap::StoreData;
use crate::agent::credentials::CredentialStore;
use crate::agent::LlmSettings;
use crate::domain::ProfileId;
use crate::error::Result;
use crate::storage::{Database, SettingKey};
use crate::theme::ThemeSettings;
use std::collections::BTreeSet;

/// Persisted settings that do not depend on the store contents.
pub(super) struct StoredSettings {
    pub llm: LlmSettings,
    pub theme: ThemeSettings,
    pub credentials: CredentialStore,
    pub preferences: crate::domain::preferences::Preferences,
}

impl StoredSettings {
    /// Reads the LLM, credential and theme settings (migrating a legacy theme and the legacy
    /// global key on the way).
    pub fn read(db: &Database) -> Self {
        let llm = LlmSettings::load_from_db(db);
        let credentials = CredentialStore::load_from_db(db, &llm);
        Self { llm, theme: db.load_theme_settings(), credentials, preferences: db.load_preferences() }
    }
}

/// A non-empty vault's collections, settings and persisted selections.
pub(super) struct VaultSnapshot {
    pub data: StoreData,
    pub settings: StoredSettings,
    pub active_network: Option<String>,
    pub active_profile: Option<String>,
    /// Seed profiles the user deleted; the seed upgrade must not re-add them.
    pub dismissed_seeds: BTreeSet<ProfileId>,
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
            active_network: db.get_setting(SettingKey::ActiveNetwork).unwrap_or(None),
            active_profile: db.get_setting(SettingKey::ActiveProfile).unwrap_or(None),
            dismissed_seeds: db.dismissed_seeds()?,
        }))
    }
}
