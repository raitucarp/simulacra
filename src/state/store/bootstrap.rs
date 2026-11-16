//! Entity bundle shared by the load and seed paths, and the `AppState` assembler.

use super::{AppState, NavigationTab};
use crate::agent::LlmSettings;
use crate::domain::{GroundingDocument, Network, NetworkId, Post, ProfileId, Profile, TrendingTopic};
use crate::error::Result;
use crate::state::seed::{seed_documents, seed_networks, seed_posts, seed_profiles, seed_trending};
use crate::storage::Database;
use crate::theme::ThemeTokens;
use std::collections::HashMap;
use std::rc::Rc;

/// Persistent entity collections that make up a store snapshot.
pub(super) struct StoreData {
    pub networks: Vec<Network>,
    pub user_profiles: Vec<Profile>,
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
            posts: db.load_posts()?,
            documents: db.load_all_documents().unwrap_or_default(),
            trending: db.load_trending()?,
        })
    }

    /// Builds the first-run seed collections.
    pub fn seeded() -> Self {
        let networks = seed_networks();
        let user_profiles = seed_profiles();
        let posts = seed_posts(&user_profiles);
        Self {
            networks,
            user_profiles,
            posts,
            documents: seed_documents(),
            trending: seed_trending(),
        }
    }

    /// Writes every collection to SQLite, ignoring individual write failures.
    pub fn persist(&self, database: &Database) {
        for net in &self.networks {
            let _ = database.save_network(net);
        }
        for prof in &self.user_profiles {
            let _ = database.save_profile(prof);
        }
        for post in &self.posts {
            let _ = database.save_post(post);
        }
        for doc in &self.documents {
            let _ = database.save_document(doc);
        }
        for trend in &self.trending {
            let _ = database.save_trending(trend);
        }
    }
}

impl AppState {
    /// Assembles a store with empty indices from loaded or seeded collections.
    pub(super) fn assemble(
        data: StoreData,
        active_network_id: NetworkId,
        active_profile_id: ProfileId,
        llm_settings: LlmSettings,
        theme_tokens: ThemeTokens,
        db: Option<Rc<Database>>,
    ) -> Self {
        Self {
            networks: data.networks,
            active_network_id,
            user_profiles: data.user_profiles,
            active_profile_id,
            active_tab: NavigationTab::Timeline,
            posts: data.posts,
            post_index_by_id: HashMap::new(),
            network_post_indices: HashMap::new(),
            replies_index: HashMap::new(),
            wikilink_cache: HashMap::new(),
            documents: data.documents,
            trending: data.trending,
            draft_text: String::new(),
            feed_day_filter: None,
            llm_settings,
            theme_tokens,
            db,
        }
    }
}
