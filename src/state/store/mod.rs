//! Central in-memory state store and actions for the UI layer.
//!
//! `AppState` is a single type whose `impl` blocks are spread across the sibling
//! modules by concern (loading, networks, profiles, posts, indices, ...).

mod bootstrap;
mod day_filter;
mod documents;
mod indices;
mod interactions;
mod load;
mod networks;
mod posts;
mod profile_create;
mod profiles;
mod reposts;
mod search;
mod settings;
mod threads;
mod upgrade;
mod wikilinks;

#[cfg(test)]
mod tests;

pub use threads::{ThreadAnchor, ThreadDraft};

use crate::domain::{GroundingDocument, Network, NetworkId, Post, PostId, Profile, ProfileId, TrendingTopic};
use crate::storage::Database;
use std::collections::HashMap;
use std::rc::Rc;

/// Settings key persisting the active network selection.
const ACTIVE_NETWORK_KEY: &str = "active_network_id";
/// Settings key persisting the active authoring profile.
const ACTIVE_PROFILE_KEY: &str = "active_profile_id";
/// Placeholder id used when no network or profile exists.
const DEFAULT_ENTITY_ID: &str = "default";
/// The primary human persona, which can never be deleted.
const PRIMARY_PROFILE_ID: &str = "user_main";
/// Prefix of every generated post identifier.
const POST_ID_PREFIX: &str = "post_";

/// Primary navigation tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationTab {
    Timeline,
    Knowledge,
    AgentLab,
    PersonaLab,
    MindGraph,
    Appearance,
}

/// Application state container.
pub struct AppState {
    pub networks: Vec<Network>,
    pub active_network_id: NetworkId,
    pub user_profiles: Vec<Profile>,
    pub active_profile_id: ProfileId,
    pub active_tab: NavigationTab,
    pub posts: Vec<Post>,
    pub post_index_by_id: HashMap<PostId, usize>,
    pub network_post_indices: HashMap<NetworkId, Vec<usize>>,
    pub replies_index: HashMap<PostId, Vec<usize>>,
    pub wikilink_cache: HashMap<String, String>,
    pub documents: Vec<GroundingDocument>,
    pub trending: Vec<TrendingTopic>,
    pub draft_text: String,
    /// Local calendar day the timeline is narrowed to (set from the cadence heatmap).
    pub feed_day_filter: Option<chrono::NaiveDate>,
    pub llm_settings: crate::agent::LlmSettings,
    pub theme_tokens: crate::theme::ThemeTokens,
    pub db: Option<Rc<Database>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
