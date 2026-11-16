//! Store construction: opening the vault, loading persisted state, or seeding a first run.
//!
//! Startup reads happen synchronously (one round trip to the vault worker); every later
//! write is queued on the worker thread.

use super::bootstrap::StoreData;
use super::snapshot::{StoredSettings, VaultSnapshot};
use super::{AppState, DEFAULT_ENTITY_ID};
use crate::agent::LlmSettings;
use crate::domain::{Network, Profile};
use crate::error::Result;
use crate::storage::{Database, Vault};
use crate::theme::ThemeSettings;

impl AppState {
    /// Initializes state, loading from the local SQLite database if available,
    /// or seeding initial mock entities if first run.
    pub fn new() -> Self {
        Self::with_db_path(Database::prepare_default_path())
    }

    /// Initializes state with a specific database path (or in-memory if invalid/failed).
    pub fn with_db_path(path: impl AsRef<std::path::Path>) -> Self {
        match Database::open(path).and_then(Vault::spawn) {
            Ok(vault) => Self::load_from_db(&vault).unwrap_or_else(|_| Self::fallback_seed(Some(vault))),
            Err(err) => {
                eprintln!("simulacra: vault unavailable, running without persistence: {err}");
                Self::fallback_seed(None)
            }
        }
    }

    /// Creates an in-memory test state.
    pub fn new_in_memory() -> Self {
        Self::fallback_seed(Database::in_memory().and_then(Vault::spawn).ok())
    }

    /// Loads persisted data from the vault (seeding it first when it is empty).
    pub(crate) fn load_from_db(vault: &Vault) -> Result<Self> {
        let Some(snapshot) = vault.read(VaultSnapshot::read)? else {
            return Ok(Self::fallback_seed(Some(vault.clone())));
        };
        let VaultSnapshot { data, settings, active_network, active_profile } = snapshot;
        let active_network_id = persisted_or_first(active_network, &data.networks, |n| &n.id);
        let active_profile_id = persisted_or_first(active_profile, &data.user_profiles, |p| &p.id);
        let mut state = Self::assemble(data, active_network_id, active_profile_id, settings, Some(vault.clone()));
        state.upgrade_legacy_profiles_if_needed();
        state.sync_network_counts();
        state.rebuild_indices();
        Ok(state)
    }

    /// Populates initial seed data into state and queues it for the vault.
    pub(super) fn fallback_seed(vault: Option<Vault>) -> Self {
        let data = StoreData::seeded();
        let settings = match vault.as_ref() {
            Some(vault) => {
                let seed = data.clone();
                let _ = vault.write(move |db| seed.persist(db));
                vault.read(|db| Ok(StoredSettings::read(db))).unwrap_or_else(|_| default_settings())
            }
            None => default_settings(),
        };
        let active_network_id = persisted_or_first(None, &data.networks, |n: &Network| &n.id);
        let active_profile_id = persisted_or_first(None, &data.user_profiles, |p: &Profile| &p.id);
        let mut state = Self::assemble(data, active_network_id, active_profile_id, settings, vault);
        state.rebuild_indices();
        state
    }
}

fn default_settings() -> StoredSettings {
    StoredSettings { llm: LlmSettings::default(), theme: ThemeSettings::default() }
}

/// The persisted id when it still names an entity, else the first entity's id, else the default.
fn persisted_or_first<T, Id>(persisted: Option<String>, entities: &[T], id_of: impl Fn(&T) -> &Id) -> Id
where
    Id: Clone + From<String> + From<&'static str> + PartialEq,
{
    persisted
        .map(Id::from)
        .filter(|id| entities.iter().any(|e| id_of(e) == id))
        .or_else(|| entities.first().map(|e| id_of(e).clone()))
        .unwrap_or_else(|| Id::from(DEFAULT_ENTITY_ID))
}
