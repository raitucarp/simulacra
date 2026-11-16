//! Central in-memory state store and actions for the UI layer.
//!
//! `AppState` is a single type whose `impl` blocks are spread across the sibling
//! modules by concern (loading, networks, profiles, posts, indices, ...).

mod authors;
mod bootstrap;
mod day_filter;
mod documents;
mod indices;
mod interactions;
mod load;
mod network_create;
mod networks;
mod post_index;
mod post_meta;
mod post_delete;
mod posts;
mod forge_autosave;
mod profile_create;
mod profiles;
mod reposts;
mod search;
mod settings;
mod snapshot;
mod threads;
mod upgrade;
mod wikilinks;

#[cfg(test)]
mod tests;

pub(crate) use profile_create::{AGENT_ID_PREFIX, PERSONA_ID_PREFIX};
pub use post_index::PostIndex;
pub use post_meta::PostMeta;
pub use network_create::{network_slug, validate_network_name, NetworkNameError, MAX_NETWORK_NAME_CHARS};
pub use threads::{ThreadAnchor, ThreadDraft};
pub use wikilinks::WikilinkTarget;

use crate::domain::{GroundingDocument, Network, NetworkId, Post, Profile, ProfileId, TrendingTopic};
use crate::storage::Vault;
use std::collections::HashMap;

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
    /// Last known profile of deleted authors whose posts remain, by author id.
    pub archived_authors: HashMap<ProfileId, Profile>,
    pub active_profile_id: ProfileId,
    pub active_tab: NavigationTab,
    /// Every post, newest first. Mutate through the store methods so `index` stays in sync.
    pub posts: Vec<Post>,
    /// Incrementally maintained lookups and derived body data over `posts`.
    pub index: PostIndex,
    pub documents: Vec<GroundingDocument>,
    pub trending: Vec<TrendingTopic>,
    pub draft_text: String,
    /// Local calendar day the timeline is narrowed to (set from the cadence heatmap).
    pub feed_day_filter: Option<chrono::NaiveDate>,
    pub llm_settings: crate::agent::LlmSettings,
    /// The user's theme choice; `theme_tokens` is always derived from it.
    pub theme_settings: crate::theme::ThemeSettings,
    pub theme_tokens: crate::theme::ThemeTokens,
    /// Handle to the vault worker thread (`None` when running without persistence).
    pub db: Option<Vault>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
