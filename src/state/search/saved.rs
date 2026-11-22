//! Saved searches in the vault (Q5): one JSON row of the settings table.

use crate::domain::search_query::SavedSearches;
use crate::state::AppState;
use crate::storage::SettingKey;

impl AppState {
    /// The saved searches (none without a vault, or when the row is unreadable).
    pub fn saved_searches(&self) -> SavedSearches {
        let raw = self.db.as_ref().and_then(|db| db.get_setting(SettingKey::SavedSearches).ok().flatten());
        raw.map(|raw| SavedSearches::from_json(&raw)).unwrap_or_default()
    }

    /// Stores `saved` (queued; a failure reaches the vault's failure channel).
    pub fn store_saved_searches(&self, saved: &SavedSearches) {
        if let Some(db) = self.db.as_ref() {
            let _ = db.set_setting(SettingKey::SavedSearches, &saved.to_json());
        }
    }
}
