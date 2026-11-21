//! Entity bundle shared by the load and seed paths, and the `AppState` assembler.

use super::networks::recount_networks;
use super::{AppState, NavigationTab};
use super::snapshot::StoredSettings;
use crate::domain::document::Library;
use crate::domain::reaction::ReactionBook;
use crate::domain::{GroundingDocument, Network, NetworkId, Post, ProfileId, Profile, TrendingTopic};
use crate::error::Result;
use crate::state::seed::{seed_documents, seed_library, seed_networks, seed_posts, seed_profiles, seed_trending};
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
    /// Subject vocabulary and collections of the vault catalogue.
    pub library: Library,
    pub trending: Vec<TrendingTopic>,
    /// Emoji reactions and claps (epic R).
    pub reactions: ReactionBook,
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
            library: db.load_library().unwrap_or_default(),
            trending: db.load_trending()?,
            reactions: db.load_reactions().map(ReactionBook::from_rows).unwrap_or_default(),
        })
    }

    /// Builds the first-run seed collections, with network counts derived from them.
    pub fn seeded() -> Self {
        let (mut networks, user_profiles, documents) = (seed_networks(), seed_profiles(), seed_documents());
        recount_networks(&mut networks, &user_profiles, &documents);
        Self {
            networks,
            user_profiles,
            archived_authors: HashMap::new(),
            posts: seed_posts(),
            documents,
            library: seed_library(),
            trending: seed_trending(),
            reactions: ReactionBook::default(),
        }
    }

    /// Writes every collection to SQLite, attempting every record and returning the first
    /// failure (the vault worker reports it).
    pub fn persist(&self, database: &Database) -> Result<()> {
        let results = self.networks.iter().map(|n| database.save_network(n))
            .chain(self.user_profiles.iter().map(|p| database.save_profile(p)))
            .chain(self.posts.iter().map(|p| database.save_post(p)))
            .chain(self.library.subjects.iter().map(|s| database.save_subject(s)))
            .chain(self.library.collections.iter().map(|c| database.save_collection(c)))
            .chain(self.documents.iter().map(|d| database.save_document(d)))
            .chain(self.trending.iter().map(|t| database.save_trending(t)))
            .chain(self.reactions.rows().map(|r| database.save_reaction(r)));
        // Collecting first runs every save, even after a failure; the first error is returned.
        let outcomes: Vec<Result<()>> = results.collect();
        outcomes.into_iter().collect()
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
        let (scheduled, posts) = data.posts.into_iter().partition(|post| post.lifecycle.is_scheduled());
        Self {
            networks: data.networks,
            active_network_id,
            user_profiles: data.user_profiles,
            archived_authors: data.archived_authors,
            active_profile_id,
            active_tab: NavigationTab::Timeline,
            posts,
            scheduled: super::scheduled::soonest_first(scheduled),
            index: Default::default(),
            documents: data.documents,
            library: data.library,
            trending: data.trending,
            draft_text: String::new(),
            feed_day_filter: None,
            llm_settings: settings.llm,
            credentials: settings.credentials,
            theme_tokens: settings.theme.tokens(),
            theme_settings: settings.theme,
            preferences: settings.preferences,
            db,
            load_error: None,
            media: Default::default(),
            engagement: 0,
            reactions: data.reactions,
        }
    }
}
