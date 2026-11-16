//! Seed profiles the user deleted, remembered as a JSON id list in the `settings` table so
//! the startup seed upgrade never brings them back (new seeds of later versions still land).

use super::{db_err, Database, SettingKey};
use crate::domain::ProfileId;
use crate::error::Result;
use std::collections::BTreeSet;

impl Database {
    /// Ids of the seed profiles the user deleted (an unreadable list counts as empty).
    pub fn dismissed_seeds(&self) -> Result<BTreeSet<ProfileId>> {
        let stored = self.get_setting(SettingKey::DismissedSeeds)?;
        Ok(stored.and_then(|json| serde_json::from_str(&json).ok()).unwrap_or_default())
    }

    /// Remembers that the seed profile `id` was deleted.
    pub fn dismiss_seed(&self, id: &ProfileId) -> Result<()> {
        let mut dismissed = self.dismissed_seeds()?;
        if !dismissed.insert(id.clone()) {
            return Ok(());
        }
        let json = serde_json::to_string(&dismissed).map_err(db_err)?;
        self.set_setting(SettingKey::DismissedSeeds, &json)
    }
}
