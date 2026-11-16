//! Store construction: opening the vault, loading persisted state, or seeding a first run.

use super::bootstrap::StoreData;
use super::{AppState, ACTIVE_NETWORK_KEY, ACTIVE_PROFILE_KEY, DEFAULT_ENTITY_ID};
use crate::agent::LlmSettings;
use crate::domain::{Network, NetworkId, Profile, ProfileId};
use crate::error::Result;
use crate::storage::Database;
use crate::theme::ThemeTokens;
use std::rc::Rc;

impl AppState {
    /// Initializes state, loading from the local SQLite database if available,
    /// or seeding initial mock entities if first run.
    pub fn new() -> Self {
        Self::with_db_path(Database::default_path())
    }

    /// Initializes state with a specific database path (or in-memory if invalid/failed).
    pub fn with_db_path(path: impl AsRef<std::path::Path>) -> Self {
        match Database::open(path) {
            Ok(db) => {
                let rc_db = Rc::new(db);
                match Self::load_from_db(&rc_db) {
                    Ok(state) => state,
                    Err(_) => Self::fallback_seed(Some(rc_db)),
                }
            }
            Err(_) => Self::fallback_seed(None),
        }
    }

    /// Creates an in-memory test state.
    pub fn new_in_memory() -> Self {
        match Database::in_memory() {
            Ok(db) => {
                let rc_db = Rc::new(db);
                Self::fallback_seed(Some(rc_db))
            }
            Err(_) => Self::fallback_seed(None),
        }
    }

    /// Loads persisted data from SQLite database.
    pub(super) fn load_from_db(db: &Rc<Database>) -> Result<Self> {
        if db.is_empty()? {
            return Ok(Self::fallback_seed(Some(db.clone())));
        }

        let data = StoreData::load(db)?;
        let active_network_id = persisted_network_id(db, &data.networks);
        let active_profile_id = persisted_profile_id(db, &data.user_profiles);
        let llm_settings = LlmSettings::load_from_db(db);
        let theme_tokens = stored_theme_tokens(db);

        let mut state = Self::assemble(
            data,
            active_network_id,
            active_profile_id,
            llm_settings,
            theme_tokens,
            Some(db.clone()),
        );
        state.upgrade_legacy_profiles_if_needed();
        state.sync_network_counts();
        state.rebuild_indices();
        Ok(state)
    }

    /// Populates initial seed data into state and optionally syncs it to SQLite.
    pub(super) fn fallback_seed(db: Option<Rc<Database>>) -> Self {
        let data = StoreData::seeded();
        if let Some(ref database) = db {
            data.persist(database);
        }

        let active_network_id = first_network_id(&data.networks);
        let active_profile_id = first_profile_id(&data.user_profiles);
        let llm_settings = match db.as_ref() {
            Some(d) => LlmSettings::load_from_db(d),
            None => LlmSettings::default(),
        };
        let theme_tokens = match db.as_ref() {
            Some(d) => stored_theme_tokens(d),
            None => crate::theme::minimalist_dark(),
        };

        let mut state = Self::assemble(
            data,
            active_network_id,
            active_profile_id,
            llm_settings,
            theme_tokens,
            db,
        );
        state.rebuild_indices();
        state
    }
}

/// Persisted theme tokens, or the default dark theme when absent or unreadable.
fn stored_theme_tokens(db: &Database) -> ThemeTokens {
    db.load_theme_tokens()
        .unwrap_or(None)
        .unwrap_or_else(crate::theme::minimalist_dark)
}

/// Restores the persisted active network when it still exists, else the first network.
fn persisted_network_id(db: &Database, networks: &[Network]) -> NetworkId {
    db.get_setting(ACTIVE_NETWORK_KEY)
        .unwrap_or(None)
        .map(NetworkId::new)
        .filter(|id| networks.iter().any(|n| &n.id == id))
        .unwrap_or_else(|| first_network_id(networks))
}

/// Restores the persisted active profile when it still exists, else the first profile.
fn persisted_profile_id(db: &Database, profiles: &[Profile]) -> ProfileId {
    db.get_setting(ACTIVE_PROFILE_KEY)
        .unwrap_or(None)
        .map(ProfileId::new)
        .filter(|id| profiles.iter().any(|p| &p.id == id))
        .unwrap_or_else(|| first_profile_id(profiles))
}

fn first_network_id(networks: &[Network]) -> NetworkId {
    networks
        .first()
        .map(|n| n.id.clone())
        .unwrap_or_else(|| NetworkId::new(DEFAULT_ENTITY_ID))
}

fn first_profile_id(profiles: &[Profile]) -> ProfileId {
    profiles
        .first()
        .map(|p| p.id.clone())
        .unwrap_or_else(|| ProfileId::new(DEFAULT_ENTITY_ID))
}
