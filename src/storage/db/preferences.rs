//! Persistence of [`Preferences`]: one JSON row per category (an unreadable row resets only
//! its category), and the one-time migration of the legacy `scheduler_enabled` row.

use super::{Database, SettingKey};
use crate::domain::preferences::{migrate_legacy, LegacyValues, Preferences};
use crate::error::{AppError, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Copies the open database into a new file.
const BACKUP_INTO: &str = "VACUUM INTO ?1";

/// Every category row, in [`Preferences`] field order.
pub const PREFERENCE_KEYS: [SettingKey; 13] = [
    SettingKey::PrefsGeneral,
    SettingKey::PrefsAppearance,
    SettingKey::PrefsWriting,
    SettingKey::PrefsAi,
    SettingKey::PrefsEmbeddings,
    SettingKey::PrefsImages,
    SettingKey::PrefsAgents,
    SettingKey::PrefsLearning,
    SettingKey::PrefsImport,
    SettingKey::PrefsPrivacy,
    SettingKey::PrefsNotifications,
    SettingKey::PrefsCode,
    SettingKey::PrefsAdvanced,
];

fn json<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(|e| AppError::DatabaseError(format!("preferences: {e}")))
}

/// The rows that store `prefs` (pure).
pub fn preference_rows(prefs: &Preferences) -> Result<Vec<(SettingKey, String)>> {
    let values = [
        json(&prefs.general)?, json(&prefs.appearance)?, json(&prefs.writing)?, json(&prefs.ai)?,
        json(&prefs.embeddings)?, json(&prefs.images)?, json(&prefs.agents)?, json(&prefs.learning)?,
        json(&prefs.import)?, json(&prefs.privacy)?, json(&prefs.notifications)?, json(&prefs.code)?, json(&prefs.advanced)?,
    ];
    Ok(PREFERENCE_KEYS.into_iter().zip(values).collect())
}

impl Database {
    /// The category stored under `key`; absent or unreadable rows give the defaults.
    fn preference_row<T: DeserializeOwned + Default>(&self, key: SettingKey) -> T {
        let Some(raw) = self.get_setting(key).ok().flatten() else { return T::default() };
        serde_json::from_str(&raw)
            .map_err(|err| eprintln!("simulacra: ignoring unreadable `{}` preferences: {err}", key.as_str()))
            .unwrap_or_default()
    }

    /// Loads (validated) preferences; a vault without preference rows is migrated first.
    pub fn load_preferences(&self) -> Preferences {
        let stored = PREFERENCE_KEYS.iter().any(|key| self.get_setting(*key).ok().flatten().is_some());
        if !stored {
            return self.migrate_preferences();
        }
        Preferences {
            general: self.preference_row(SettingKey::PrefsGeneral),
            appearance: self.preference_row(SettingKey::PrefsAppearance),
            writing: self.preference_row(SettingKey::PrefsWriting),
            ai: self.preference_row(SettingKey::PrefsAi),
            embeddings: self.preference_row(SettingKey::PrefsEmbeddings),
            images: self.preference_row(SettingKey::PrefsImages),
            agents: self.preference_row(SettingKey::PrefsAgents),
            learning: self.preference_row(SettingKey::PrefsLearning),
            import: self.preference_row(SettingKey::PrefsImport),
            privacy: self.preference_row(SettingKey::PrefsPrivacy),
            notifications: self.preference_row(SettingKey::PrefsNotifications),
            code: self.preference_row(SettingKey::PrefsCode),
            advanced: self.preference_row(SettingKey::PrefsAdvanced),
        }
        .validated()
    }

    /// Folds the legacy rows into default preferences, saves them and drops the legacy rows.
    fn migrate_preferences(&self) -> Preferences {
        let legacy = LegacyValues { scheduler_enabled: self.get_setting(SettingKey::SchedulerEnabled).ok().flatten() };
        let prefs = migrate_legacy(Preferences::default(), &legacy);
        let saved = self.save_preferences(&prefs).and_then(|()| self.delete_setting(SettingKey::SchedulerEnabled));
        if let Err(err) = saved {
            eprintln!("simulacra: preferences migration not saved: {err}");
        }
        prefs
    }

    /// Persists every preference category.
    pub fn save_preferences(&self, prefs: &Preferences) -> Result<()> {
        preference_rows(prefs)?.iter().try_for_each(|(key, value)| self.set_setting(*key, value))
    }

    /// Writes a consistent copy of the whole database to `path` (which must not exist).
    pub fn backup_to(&self, path: &std::path::Path) -> Result<()> {
        let target = path.to_string_lossy().to_string();
        self.conn.execute(BACKUP_INTO, [target]).map(|_| ()).map_err(super::db_err)
    }
}
