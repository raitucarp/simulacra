//! Persisting the Agent Forge's in-progress draft in the `settings` table.

use super::AppState;
use crate::state::forge::AgentDraft;
use crate::storage::SettingKey;

/// Mnemo greets every new forger until they dismiss him.
const GUIDE_HIDDEN_DEFAULT: bool = false;

impl AppState {
    /// The autosaved forge draft, if one exists and can still be read.
    pub fn load_forge_autosave(&self) -> Option<AgentDraft> {
        let db = self.db.as_ref()?;
        let raw = db.get_setting(SettingKey::ForgeAutosave).ok().flatten()?;
        AgentDraft::from_autosave(&raw).ok()
    }

    /// Stores `draft` so an interrupted forge session can resume.
    pub fn save_forge_autosave(&self, draft: &AgentDraft) {
        if let Some(ref db) = self.db {
            if let Err(e) = db.set_setting(SettingKey::ForgeAutosave, &draft.to_autosave()) {
                eprintln!("simulacra: failed to autosave the forge draft: {e}");
            }
        }
    }

    /// Forgets the autosaved draft (after it was committed or discarded).
    pub fn clear_forge_autosave(&self) {
        if let Some(ref db) = self.db {
            if let Err(e) = db.write(|db| db.delete_setting(SettingKey::ForgeAutosave)) {
                eprintln!("simulacra: failed to clear the forge autosave: {e}");
            }
        }
    }
}

impl AppState {
    /// Whether the user dismissed Mnemo the guide owl (persisted; shown by default).
    pub fn forge_guide_hidden(&self) -> bool {
        self.db
            .as_ref()
            .and_then(|db| db.get_setting(SettingKey::ForgeHideGuide).ok().flatten())
            .and_then(|raw| raw.parse::<bool>().ok())
            .unwrap_or(GUIDE_HIDDEN_DEFAULT)
    }

    /// Persists whether Mnemo the guide owl is hidden.
    pub fn set_forge_guide_hidden(&self, hidden: bool) {
        if let Some(ref db) = self.db {
            if let Err(e) = db.set_setting(SettingKey::ForgeHideGuide, &hidden.to_string()) {
                eprintln!("simulacra: failed to persist the forge guide switch: {e}");
            }
        }
    }
}
