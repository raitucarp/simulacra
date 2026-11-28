//! Credential persistence: one JSON array in the `settings` table.

use super::{Credential, CredentialKind, CredentialStore};
use crate::agent::llm::LlmSettings;
use crate::storage::{Database, SettingKey};

/// Id of the credential mirroring the global Settings › AI key.
pub const DEFAULT_CREDENTIAL_ID: &str = "cred_default";
/// Label of the credential mirroring the global key.
const DEFAULT_CREDENTIAL_LABEL: &str = "Global default";

impl Credential {
    /// The name shown for the credential: the default one, while it keeps its stored name,
    /// reads as the main AI key in the interface language.
    pub fn shown_label(&self) -> String {
        if self.id == DEFAULT_CREDENTIAL_ID && self.label == DEFAULT_CREDENTIAL_LABEL {
            rust_i18n::t!("settings.credentials_default_label").to_string()
        } else {
            self.label.clone()
        }
    }
}

impl CredentialStore {
    /// Loads the store. The first load of an older vault (no credentials row) migrates the
    /// legacy global API key into [`DEFAULT_CREDENTIAL_ID`] and saves the result. A corrupt
    /// row loads as empty but is left untouched.
    pub fn load_from_db(db: &Database, global: &LlmSettings) -> Self {
        match db.get_setting(SettingKey::Credentials).ok().flatten() {
            Some(json) => Self::from_json(&json),
            None => {
                let mut store = Self::default();
                store.sync_default(global);
                if let Err(e) = store.save_to_db(db) {
                    eprintln!("simulacra: credential migration not saved: {e}");
                }
                store
            }
        }
    }

    /// Parses a stored array; malformed JSON yields an empty store.
    pub fn from_json(json: &str) -> Self {
        let items = serde_json::from_str::<Vec<Credential>>(json).unwrap_or_else(|e| {
            eprintln!("simulacra: credentials row is malformed, ignoring it: {e}");
            Vec::new()
        });
        Self { items }
    }

    /// The store as a JSON array.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.items).unwrap_or_default()
    }

    /// Persists the store.
    pub fn save_to_db(&self, db: &Database) -> Result<(), String> {
        db.set_setting(SettingKey::Credentials, &self.to_json()).map_err(|e| e.to_string())
    }

    /// Mirrors the global settings into the default credential (created only when the global
    /// settings hold a key; an existing default is always updated).
    pub fn sync_default(&mut self, global: &LlmSettings) {
        if !global.has_api_key() && self.get(DEFAULT_CREDENTIAL_ID).is_none() {
            return;
        }
        self.upsert(Credential {
            id: DEFAULT_CREDENTIAL_ID.to_string(),
            kind: CredentialKind::from_provider(global.provider),
            label: DEFAULT_CREDENTIAL_LABEL.to_string(),
            secret: global.api_key.trim().to_string(),
            endpoint: global.custom_endpoint.trim().to_string(),
        });
    }
}
