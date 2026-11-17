//! Store construction: opening the vault, loading persisted state, or seeding a first run.
//!
//! Startup reads happen synchronously (one round trip to the vault worker); every later
//! write is queued on the worker thread.

use super::bootstrap::StoreData;
use super::snapshot::{StoredSettings, VaultSnapshot};
use super::{AppState, DEFAULT_ENTITY_ID};
use crate::agent::LlmSettings;
use crate::domain::{Network, Profile};
use crate::error::{AppError, Result};
use crate::storage::{Database, Vault};
use crate::theme::ThemeSettings;

impl AppState {
    /// Initializes state, loading from the local SQLite database if available,
    /// or seeding initial mock entities if first run.
    pub fn new() -> Self {
        Self::with_db_path(Database::prepare_default_path())
    }

    /// Initializes state from the database at `path`; a missing file is created and seeded.
    ///
    /// A vault that cannot be opened or read is never written to (no re-seeding over the
    /// user's data): the session runs on unsaved seed data and records `load_error`.
    pub fn with_db_path(path: impl AsRef<std::path::Path>) -> Self {
        Self::from_vault(Database::open(path).and_then(Vault::spawn))
    }

    /// Opens (creating and migrating if needed) the database at the default path: the first
    /// step of a staged startup, which reads the theme from it before the rest loads.
    pub fn open_default_database() -> Result<Database> {
        Database::open(Database::prepare_default_path())
    }

    /// Loads the state from a started vault worker; a vault that failed to open or read
    /// leaves the session on unsaved seed data with `load_error` set.
    pub fn from_vault(vault: Result<Vault>) -> Self {
        vault.and_then(|vault| Self::load_from_db(&vault)).unwrap_or_else(Self::unsaved_after_failure)
    }

    /// Seed data without persistence, carrying the reason the vault was unavailable.
    fn unsaved_after_failure(err: AppError) -> Self {
        eprintln!("simulacra: vault unavailable, running without persistence: {err}");
        let mut state = Self::fallback_seed(None);
        state.load_error = Some(err);
        state
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
        let VaultSnapshot { data, settings, active_network, active_profile, dismissed_seeds } = snapshot;
        let active_network_id = persisted_or_first(active_network, &data.networks, |n| &n.id);
        let active_profile_id = persisted_or_first(active_profile, &data.user_profiles, |p| &p.id);
        let mut state = Self::assemble(data, active_network_id, active_profile_id, settings, Some(vault.clone()));
        state.upgrade_legacy_profiles_if_needed(&dismissed_seeds);
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
