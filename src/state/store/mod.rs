//! Central in-memory state store and actions for the UI layer.
//!
//! `AppState` is a single type whose `impl` blocks are spread across the sibling
//! modules by concern (loading, networks, profiles, posts, indices, ...).

mod action_target;
mod archive; // Y5
mod authors;
mod catalogue;
mod avatar;
mod bootstrap;
mod bulk_posts;
mod captures;
mod day_filter;
mod documents;
mod guards;
mod indices;
mod interactions;
mod load;
mod network_create;
mod onboarding;
mod networks;
mod post_index;
mod post_meta;
mod post_delete;
mod post_heartbeat;
mod posts;
mod posts_by;
mod preferences;
mod forge_autosave;
mod identities;
mod profile_create;
mod profiles;
mod reposts;
mod revisions; // sb F2
mod scheduled;
mod scheduled_queue; // Y4
mod search;
mod settings;
mod seed_catalogue;
mod snapshot;
mod threads;
mod upgrade;
mod wikilinks;

#[cfg(test)]
mod tests;

pub(crate) use profile_create::{AGENT_ID_PREFIX, PERSONA_ID_PREFIX};
pub use guards::require_persona;
pub use post_heartbeat::LifecycleReport;
pub use post_index::PostIndex;
pub use posts_by::Page;
pub use scheduled_queue::ScheduledEdit;
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
    /// Spaced-repetition review, quizzes and statistics (sb C1).
    Review,
    Appearance,
    /// Charts and graph analytics of the network (sb D2).
    Insights,
    /// The private journal, mood log, focus sessions and time capsules (sb E3).
    Journal,
    /// Debates and thought experiments between agents (sb F1).
    Arena,
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
    /// Every published or archived post, newest first. Mutate through the store methods so
    /// `index` stays in sync.
    pub posts: Vec<Post>,
    /// Scheduled posts, soonest due first: out of `posts` (so out of the feed, search and
    /// agents' context) until they are published.
    scheduled: Vec<Post>,
    /// Incrementally maintained lookups and derived body data over `posts`.
    pub index: PostIndex,
    pub documents: Vec<GroundingDocument>,
    /// The vault catalogue's subject vocabulary and collections.
    pub library: crate::domain::document::Library,
    pub trending: Vec<TrendingTopic>,
    pub draft_text: String,
    /// Local calendar day the timeline is narrowed to (set from the cadence heatmap).
    pub feed_day_filter: Option<jiff::civil::Date>,
    pub llm_settings: crate::agent::LlmSettings,
    /// Provider and service keys referenced by agent engines (Settings › AI).
    pub credentials: crate::agent::credentials::CredentialStore,
    /// The user's theme choice; `theme_tokens` is always derived from it.
    pub theme_settings: crate::theme::ThemeSettings,
    pub theme_tokens: crate::theme::ThemeTokens,
    /// Typed global preferences; read through `preferences()`, replaced through
    /// `update_preferences` (which validates and persists them).
    pub(crate) preferences: crate::domain::preferences::Preferences,
    /// Handle to the vault worker thread (`None` when running without persistence).
    pub db: Option<Vault>,
    /// Why the vault could not be opened or read at startup (the session then runs on
    /// unsaved seed data); taken once by the shell to show a notice.
    pub load_error: Option<crate::error::AppError>,
    /// Cached link previews and image scans (sb C3), filled in the background by the shell.
    pub media: crate::domain::media::MediaCatalog,
    /// Bumped whenever the user likes or unlikes a post (recommendations depend on likes, the
    /// post index revision does not change for them).
    pub engagement: u64,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
