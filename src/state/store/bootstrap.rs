//! Entity bundle shared by the load and seed paths, and the `AppState` assembler.

use super::{AppState, NavigationTab};
use super::snapshot::StoredSettings;
use crate::domain::{GroundingDocument, Network, NetworkId, Post, ProfileId, Profile, TrendingTopic};
use crate::error::Result;
use crate::state::seed::{seed_documents, seed_networks, seed_posts, seed_profiles, seed_trending};
use crate::storage::{Database, Vault};
use std::collections::HashMap;

/// Persistent entity collections that make up a store snapshot.
#[derive(Clone)]
pub(super) struct StoreData {
    pub networks: Vec<Network>,
    pub user_profiles: Vec<Profile>,
    pub archived_authors: HashMap<ProfileId, Profile>,
    pub posts: Vec<Post>,
    pub documents: Vec<GroundingDocument>,
    pub trending: Vec<TrendingTopic>,
}

impl StoreData {
    /// Loads every collection from SQLite (documents tolerate a read failure).
    pub fn load(db: &Database) -> Result<Self> {
        Ok(Self {
            networks: db.load_networks()?,
            user_profiles: db.load_profiles()?,
            archived_authors: db.load_archived_authors().unwrap_or_default(),
            posts: db.load_posts()?,
            documents: db.load_all_documents().unwrap_or_default(),
            trending: db.load_trending()?,
        })
    }

    /// Builds the first-run seed collections.
    pub fn seeded() -> Self {
        let networks = seed_networks();
        let user_profiles = seed_profiles();
        let posts = seed_posts();
        Self {
            networks,
            user_profiles,
            archived_authors: HashMap::new(),
            posts,
            documents: seed_documents(),
            trending: seed_trending(),
        }
    }

    /// Writes every collection to SQLite, attempting every record and returning the first
    /// failure (the vault worker reports it).
    pub fn persist(&self, database: &Database) -> Result<()> {
        let results = self.networks.iter().map(|n| database.save_network(n))
            .chain(self.user_profiles.iter().map(|p| database.save_profile(p)))
            .chain(self.posts.iter().map(|p| database.save_post(p)))
            .chain(self.documents.iter().map(|d| database.save_document(d)))
            .chain(self.trending.iter().map(|t| database.save_trending(t)));
        results.fold(Ok(()), |first, next| first.and(next))
    }
}

impl AppState {
    /// Assembles a store with empty indices from loaded or seeded collections.
    pub(super) fn assemble(
        data: StoreData,
        active_network_id: NetworkId,
        active_profile_id: ProfileId,
        settings: StoredSettings,
        db: Option<Vault>,
    ) -> Self {
        Self {
            networks: data.networks,
            active_network_id,
            user_profiles: data.user_profiles,
            archived_authors: data.archived_authors,
            active_profile_id,
            active_tab: NavigationTab::Timeline,
            posts: data.posts,
            index: Default::default(),
            documents: data.documents,
            trending: data.trending,
            draft_text: String::new(),
            feed_day_filter: None,
            llm_settings: settings.llm,
            theme_tokens: settings.theme.tokens(),
            theme_settings: settings.theme,
            db,
        }
    }
}
