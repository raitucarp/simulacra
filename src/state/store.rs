//! Central in-memory state store and actions for the UI layer.

use super::mock::{seed_documents, seed_networks, seed_posts, seed_profiles, seed_trending};
use crate::domain::{GroundingDocument, Network, NetworkId, Post, PostId, Profile, ProfileId, TrendingTopic};
use crate::error::{AppError, Result};
use crate::storage::Database;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

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
    pub llm_settings: crate::agent::LlmSettings,
    pub theme_tokens: crate::theme::ThemeTokens,
    pub db: Option<Rc<Database>>,
}

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
    fn load_from_db(db: &Rc<Database>) -> Result<Self> {
        if db.is_empty()? {
            return Ok(Self::fallback_seed(Some(db.clone())));
        }

        let networks = db.load_networks()?;
        let user_profiles = db.load_profiles()?;
        let posts = db.load_posts()?;
        let documents = db.load_all_documents().unwrap_or_default();
        let trending = db.load_trending()?;

        let active_net_setting = db.get_setting("active_network_id").unwrap_or(None);
        let active_network_id = active_net_setting
            .map(NetworkId::new)
            .filter(|id| networks.iter().any(|n| &n.id == id))
            .or_else(|| networks.first().map(|n| n.id.clone()))
            .unwrap_or_else(|| NetworkId::new("default"));

        let active_prof_setting = db.get_setting("active_profile_id").unwrap_or(None);
        let active_profile_id = active_prof_setting
            .map(ProfileId::new)
            .filter(|id| user_profiles.iter().any(|p| &p.id == id))
            .or_else(|| user_profiles.first().map(|p| p.id.clone()))
            .unwrap_or_else(|| ProfileId::new("default"));

        let llm_settings = crate::agent::LlmSettings::load_from_db(db);
        let theme_tokens = db
            .load_theme_tokens()
            .unwrap_or(None)
            .unwrap_or_else(crate::theme::minimalist_dark);

        let mut state = Self {
            networks,
            active_network_id,
            user_profiles,
            active_profile_id,
            active_tab: NavigationTab::Timeline,
            posts,
            post_index_by_id: HashMap::new(),
            network_post_indices: HashMap::new(),
            replies_index: HashMap::new(),
            wikilink_cache: HashMap::new(),
            documents,
            trending,
            draft_text: String::new(),
            llm_settings,
            theme_tokens,
            db: Some(db.clone()),
        };
        state.upgrade_legacy_profiles_if_needed();
        state.sync_network_counts();
        state.rebuild_indices();
        Ok(state)
    }

    /// Automatically upgrades legacy bot-named profiles (`Socrates-GPT`, `Spinoza-Node`, etc.)
    /// or missing per-network human agents in an existing SQLite database so all agents are
    /// modeled as authentic human thinkers scoped to their respective networks.
    fn upgrade_legacy_profiles_if_needed(&mut self) {
        let has_legacy_bot = self.user_profiles.iter().any(|p| {
            p.name.contains("GPT")
                || p.name.contains("-Node")
                || p.handle == "@socrates_ai"
                || p.handle == "@spinoza_bot"
                || p.name == "You (Architect)"
                || p.name == "Ghost Writer"
                || (p.is_agent() && p.network_id.is_none())
        });

        let fresh_seeds = seed_profiles();
        let mut mutated = false;

        if has_legacy_bot {
            for seeded in &fresh_seeds {
                if let Some(existing) = self.user_profiles.iter_mut().find(|p| p.id == seeded.id) {
                    *existing = seeded.clone();
                    mutated = true;
                }
            }
        }

        // Ensure all seeded per-network human agents and personas exist
        for seeded in fresh_seeds {
            if !self.user_profiles.iter().any(|p| p.id == seeded.id) {
                self.user_profiles.push(seeded);
                mutated = true;
            }
        }

        // Ensure active_profile_id points to a human-controlled persona rather than an autonomous agent
        if self
            .user_profiles
            .iter()
            .find(|p| p.id == self.active_profile_id)
            .map_or(true, |p| p.is_agent())
        {
            if let Some(first_human) = self.user_profiles.iter().find(|p| p.is_human_persona()) {
                self.active_profile_id = first_human.id.clone();
                if let Some(ref db) = self.db {
                    let _ = db.set_setting("active_profile_id", self.active_profile_id.as_str());
                }
            }
        }

        if mutated {
            // Update author snapshots in existing posts and persist to SQLite
            for prof in &self.user_profiles {
                for post in &mut self.posts {
                    if post.author.id == prof.id {
                        post.author = prof.clone();
                    }
                }
                if let Some(ref db) = self.db {
                    let _ = db.save_profile(prof);
                    let _ = db.update_post_author_snapshots(prof);
                }
            }
        }
    }

    /// Synchronizes each network's `agent_count` and `doc_count` with actual state.
    pub fn sync_network_counts(&mut self) {
        for net in &mut self.networks {
            let agent_cnt = self
                .user_profiles
                .iter()
                .filter(|p| p.is_agent() && p.belongs_to_network(&net.id))
                .count();
            let doc_cnt = self
                .documents
                .iter()
                .filter(|d| d.network_id == net.id)
                .count();
            if net.agent_count != agent_cnt || net.doc_count != doc_cnt {
                net.agent_count = agent_cnt;
                net.doc_count = doc_cnt;
                if let Some(ref db) = self.db {
                    let _ = db.save_network(net);
                }
            }
        }
    }

    /// Populates initial seed data into state and optionally syncs it to SQLite.
    fn fallback_seed(db: Option<Rc<Database>>) -> Self {
        let networks = seed_networks();
        let user_profiles = seed_profiles();
        let posts = seed_posts(&user_profiles);
        let documents = seed_documents();
        let trending = seed_trending();

        if let Some(ref database) = db {
            for net in &networks {
                let _ = database.save_network(net);
            }
            for prof in &user_profiles {
                let _ = database.save_profile(prof);
            }
            for post in &posts {
                let _ = database.save_post(post);
            }
            for doc in &documents {
                let _ = database.save_document(doc);
            }
            for trend in &trending {
                let _ = database.save_trending(trend);
            }
        }

        let active_network_id = networks
            .first()
            .map(|n| n.id.clone())
            .unwrap_or_else(|| NetworkId::new("default"));
        let active_profile_id = user_profiles
            .first()
            .map(|p| p.id.clone())
            .unwrap_or_else(|| ProfileId::new("default"));

        let llm_settings = match db.as_ref() {
            Some(d) => crate::agent::LlmSettings::load_from_db(d),
            None => crate::agent::LlmSettings::default(),
        };

        let theme_tokens = match db.as_ref() {
            Some(d) => d
                .load_theme_tokens()
                .unwrap_or(None)
                .unwrap_or_else(crate::theme::minimalist_dark),
            None => crate::theme::minimalist_dark(),
        };

        let mut state = Self {
            networks,
            active_network_id,
            user_profiles,
            active_profile_id,
            active_tab: NavigationTab::Timeline,
            posts,
            post_index_by_id: HashMap::new(),
            network_post_indices: HashMap::new(),
            replies_index: HashMap::new(),
            wikilink_cache: HashMap::new(),
            documents,
            trending,
            draft_text: String::new(),
            llm_settings,
            theme_tokens,
            db,
        };
        state.rebuild_indices();
        state
    }

    /// Updates and persists dynamic theme design tokens to SQLite.
    pub fn update_theme_tokens(&mut self, tokens: crate::theme::ThemeTokens) {
        if let Some(ref db) = self.db {
            let _ = db.save_theme_tokens(&tokens);
        }
        self.theme_tokens = tokens;
    }

    /// Updates and persists LLM Provider settings.
    pub fn update_llm_settings(&mut self, settings: crate::agent::LlmSettings) {
        if let Some(ref database) = self.db {
            let _ = settings.save_to_db(database);
        }
        self.llm_settings = settings;
    }

    /// Returns a reference to the active network domain.
    #[inline]
    pub fn current_network(&self) -> Option<&Network> {
        self.networks.iter().find(|n| n.id == self.active_network_id)
    }

    /// Returns a reference to the currently active author profile.
    #[inline]
    pub fn current_profile(&self) -> Option<&Profile> {
        self.user_profiles.iter().find(|p| p.id == self.active_profile_id)
    }

    /// Switches the active network domain and persists selection.
    pub fn switch_network(&mut self, network_id: &NetworkId) -> Result<()> {
        if self.networks.iter().any(|n| &n.id == network_id) {
            self.active_network_id = network_id.clone();
            if let Some(ref db) = self.db {
                let _ = db.set_setting("active_network_id", network_id.as_str());
            }
            Ok(())
        } else {
            Err(AppError::NetworkNotFound(network_id.to_string()))
        }
    }

    /// Switches the active authoring profile and persists selection.
    pub fn switch_profile(&mut self, profile_id: &ProfileId) -> Result<()> {
        if self.user_profiles.iter().any(|p| &p.id == profile_id) {
            self.active_profile_id = profile_id.clone();
            if let Some(ref db) = self.db {
                let _ = db.set_setting("active_profile_id", profile_id.as_str());
            }
            Ok(())
        } else {
            Err(AppError::ProfileNotFound(profile_id.to_string()))
        }
    }

    /// Returns all autonomous human-like member agents belonging to the currently active network.
    pub fn current_network_agents(&self) -> Vec<&Profile> {
        self.agents_for_network(&self.active_network_id)
    }

    /// Returns all autonomous human-like member agents belonging to the specified network.
    pub fn agents_for_network(&self, network_id: &NetworkId) -> Vec<&Profile> {
        self.user_profiles
            .iter()
            .filter(|p| p.is_agent() && p.belongs_to_network(network_id))
            .collect()
    }

    /// Returns all controllable human personas available for profile switching in the active network.
    pub fn current_network_personas(&self) -> Vec<&Profile> {
        self.user_profiles
            .iter()
            .filter(|p| p.is_human_persona() && p.belongs_to_network(&self.active_network_id))
            .collect()
    }

    /// Cycles the active human persona to the next controllable persona in the current network.
    pub fn cycle_active_persona(&mut self) -> Option<ProfileId> {
        let persona_ids: Vec<ProfileId> = self
            .current_network_personas()
            .into_iter()
            .map(|p| p.id.clone())
            .collect();
        if persona_ids.is_empty() {
            return None;
        }
        let current_idx = persona_ids
            .iter()
            .position(|id| id == &self.active_profile_id)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % persona_ids.len();
        let next_id = persona_ids[next_idx].clone();
        let _ = self.switch_profile(&next_id);
        Some(next_id)
    }

    /// Creates a new custom human-like autonomous agent scoped to the active network and persists to SQLite.
    pub fn create_network_agent(
        &mut self,
        name: &str,
        handle: &str,
        description: &str,
    ) -> Result<ProfileId> {
        let clean_name = if name.trim().is_empty() {
            "New Network Thinker"
        } else {
            name.trim()
        };
        let raw_handle = if handle.trim().is_empty() {
            clean_name
                .split_whitespace()
                .next()
                .unwrap_or("member")
                .to_lowercase()
        } else {
            handle.trim().trim_start_matches('@').to_lowercase()
        };
        let clean_handle = format!("@{}", raw_handle.replace(' ', "_"));
        let clean_desc = if description.trim().is_empty() {
            "Independent Thinker"
        } else {
            description.trim()
        };

        let id_str = format!("agent_{}", Uuid::new_v4().simple());
        let mut cfg = crate::domain::AgentConfig::default();
        cfg.description = clean_desc.to_string();
        cfg.occupation = clean_desc.to_string();
        cfg.system_prompt = format!(
            "You are {} ({}), a human member of this network ({}). Share authentic, insightful reflections grounded in your lived biography and psychological profile.",
            clean_name, clean_handle, clean_desc
        );

        let profile = Profile::new_agent(id_str.as_str(), clean_name, clean_handle.as_str(), clean_desc)
            .with_network(self.active_network_id.as_str())
            .with_config(cfg);

        if let Some(ref db) = self.db {
            db.save_profile(&profile)?;
        }

        let new_id = profile.id.clone();
        self.user_profiles.push(profile);
        self.sync_network_counts();
        Ok(new_id)
    }

    /// Creates a new controllable human persona in the active network for Persona Lab switching.
    pub fn create_human_persona(
        &mut self,
        name: &str,
        handle: &str,
        description: &str,
    ) -> Result<ProfileId> {
        let clean_name = if name.trim().is_empty() {
            "New Human Persona"
        } else {
            name.trim()
        };
        let raw_handle = if handle.trim().is_empty() {
            clean_name
                .split_whitespace()
                .next()
                .unwrap_or("persona")
                .to_lowercase()
        } else {
            handle.trim().trim_start_matches('@').to_lowercase()
        };
        let clean_handle = format!("@{}", raw_handle.replace(' ', "_"));
        let clean_desc = if description.trim().is_empty() {
            "Alter-Ego Persona"
        } else {
            description.trim()
        };

        let id_str = format!("user_{}", Uuid::new_v4().simple());
        let mut cfg = crate::domain::AgentConfig::default();
        cfg.description = clean_desc.to_string();
        cfg.occupation = clean_desc.to_string();

        let profile = Profile::new_alt(id_str.as_str(), clean_name, clean_handle.as_str())
            .with_network(self.active_network_id.as_str())
            .with_config(cfg);

        if let Some(ref db) = self.db {
            db.save_profile(&profile)?;
        }

        let new_id = profile.id.clone();
        self.user_profiles.push(profile);
        Ok(new_id)
    }

    /// Updates an existing agent or human persona profile, synchronizes derived fields and post author snapshots,
    /// and persists to SQLite.
    pub fn update_profile(&mut self, mut updated: Profile) -> Result<()> {
        updated.sync_derived_fields();
        let target_id = updated.id.clone();

        if let Some(existing) = self.user_profiles.iter_mut().find(|p| p.id == target_id) {
            *existing = updated.clone();
        } else {
            self.user_profiles.push(updated.clone());
        }

        for post in &mut self.posts {
            if post.author.id == target_id {
                post.author = updated.clone();
            }
        }

        if let Some(ref db) = self.db {
            db.save_profile(&updated)?;
            let _ = db.update_post_author_snapshots(&updated);
        }

        self.sync_network_counts();
        Ok(())
    }

    /// Deletes an agent or custom persona profile by ID.
    /// Protects the primary human persona (`user_main`) from accidental deletion.
    pub fn delete_profile(&mut self, profile_id: &ProfileId) -> Result<()> {
        if profile_id.as_str() == "user_main" {
            return Ok(());
        }

        self.user_profiles.retain(|p| &p.id != profile_id);
        if let Some(ref db) = self.db {
            db.delete_profile(profile_id)?;
        }

        if &self.active_profile_id == profile_id {
            if let Some(first_human) = self.user_profiles.iter().find(|p| p.is_human_persona()) {
                self.active_profile_id = first_human.id.clone();
                if let Some(ref db) = self.db {
                    let _ = db.set_setting("active_profile_id", self.active_profile_id.as_str());
                }
            }
        }

        self.sync_network_counts();
        Ok(())
    }

    /// Creates and publishes a new thought to the current network with optional attachments and relation IDs.
    pub fn submit_post_with_attachments(
        &mut self,
        content: &str,
        link_url: Option<String>,
        image_path: Option<String>,
        reply_to_id: Option<PostId>,
        quote_id: Option<PostId>,
    ) -> Result<PostId> {
        let author = self
            .current_profile()
            .cloned()
            .ok_or_else(|| AppError::ProfileNotFound(self.active_profile_id.to_string()))?;

        let post_id = PostId::new(format!("post_{}", Uuid::new_v4().simple()));
        let post = Post::create_with_attachments(
            post_id.clone(),
            self.active_network_id.clone(),
            author,
            content,
            link_url,
            image_path,
            reply_to_id.clone(),
            quote_id.clone(),
            None,
        )?;

        // If this is a reply to an existing post, increment parent's replies_count and persist
        if let Some(ref parent_id) = reply_to_id {
            let parent_idx = self.post_index_by_id.get(parent_id).copied();
            if let Some(idx) = parent_idx {
                if let Some(parent) = self.posts.get_mut(idx) {
                    parent.replies_count += 1;
                    if let Some(ref db) = self.db {
                        let _ = db.update_post_replies_count(parent_id, parent.replies_count);
                    }
                }
            } else if let Some(parent) = self.posts.iter_mut().find(|p| &p.id == parent_id) {
                parent.replies_count += 1;
                if let Some(ref db) = self.db {
                    let _ = db.update_post_replies_count(parent_id, parent.replies_count);
                }
            }
        }

        if let Some(ref db) = self.db {
            db.save_post(&post)?;
        }

        let post_excerpt = Self::compute_wikilink_excerpt(&post);
        self.wikilink_cache.insert(post.id.as_str().to_string(), post_excerpt);

        self.posts.insert(0, post);

        self.sync_post_index();
        self.draft_text.clear();
        Ok(post_id)
    }

    /// Creates and publishes a standard thought to the current network timeline.
    #[inline]
    pub fn submit_post(&mut self, content: &str) -> Result<PostId> {
        self.submit_post_with_attachments(content, None, None, None, None)
    }

    /// Immediately publishes an instant repost of the target thought to the timeline.
    pub fn repost(&mut self, target_post_id: &PostId) -> Result<PostId> {
        let author = self
            .current_profile()
            .cloned()
            .ok_or_else(|| AppError::ProfileNotFound(self.active_profile_id.to_string()))?;

        // Check if author has already reposted this target post
        let already_reposted = self
            .posts
            .iter()
            .any(|p| p.repost_id.as_ref() == Some(target_post_id) && p.author.id == author.id);

        let is_now_reposted = !already_reposted;

        // Locate target post and update its reposts count and is_reposted state
        let (target_network_id, new_repost_count) = {
            let target_idx = self
                .post_index_by_id
                .get(target_post_id)
                .copied()
                .or_else(|| self.posts.iter().position(|p| &p.id == target_post_id))
                .ok_or_else(|| AppError::DatabaseError(format!("Target post '{target_post_id}' not found")))?;

            let target = self
                .posts
                .get_mut(target_idx)
                .ok_or_else(|| AppError::DatabaseError(format!("Target post '{target_post_id}' not found")))?;

            target.is_reposted = is_now_reposted;
            if is_now_reposted {
                target.reposts_count += 1;
            } else if target.reposts_count > 0 {
                target.reposts_count -= 1;
            }
            (target.network_id.clone(), target.reposts_count)
        };

        if let Some(ref db) = self.db {
            db.update_post_repost(target_post_id, new_repost_count, is_now_reposted)?;
        }

        if !is_now_reposted {
            self.posts
                .retain(|p| p.repost_id.as_ref() != Some(target_post_id) || p.author.id != author.id);
            if let Some(ref db) = self.db {
                let _ = db.delete_repost_by_target_and_author(target_post_id, &author.id);
            }
            self.sync_post_index();
            return Ok(target_post_id.clone());
        }

        let post_id = PostId::new(format!("post_{}", Uuid::new_v4().simple()));
        let repost_post = Post::create_with_attachments(
            post_id.clone(),
            target_network_id,
            author,
            "",
            None,
            None,
            None,
            None,
            Some(target_post_id.clone()),
        )?;

        if let Some(ref db) = self.db {
            db.save_post(&repost_post)?;
        }

        let repost_excerpt = Self::compute_wikilink_excerpt(&repost_post);
        self.wikilink_cache.insert(repost_post.id.as_str().to_string(), repost_excerpt);

        self.posts.insert(0, repost_post);
        self.sync_post_index();
        Ok(post_id)
    }

    /// Submits a quoted thought referencing another thought with commentary and optional attachments.
    pub fn submit_quote(
        &mut self,
        target_post_id: &PostId,
        content: &str,
        link: Option<String>,
        img: Option<String>,
    ) -> Result<PostId> {
        if !self.post_index_by_id.contains_key(target_post_id) && !self.posts.iter().any(|p| &p.id == target_post_id) {
            return Err(AppError::DatabaseError(format!("Target post '{target_post_id}' not found")));
        }

        self.submit_post_with_attachments(
            content,
            link,
            img,
            None,
            Some(target_post_id.clone()),
        )
    }

    /// Submits a dialectic reply to a parent thought connecting them via threadlines.
    pub fn submit_reply(
        &mut self,
        parent_post_id: &PostId,
        content: &str,
        link: Option<String>,
        img: Option<String>,
    ) -> Result<PostId> {
        if !self.post_index_by_id.contains_key(parent_post_id) && !self.posts.iter().any(|p| &p.id == parent_post_id) {
            return Err(AppError::DatabaseError(format!("Parent post '{parent_post_id}' not found")));
        }

        self.submit_post_with_attachments(
            content,
            link,
            img,
            Some(parent_post_id.clone()),
            None,
        )
    }

    /// Returns posts matching the active network.
    pub fn current_network_posts(&self) -> impl Iterator<Item = &Post> {
        self.posts.iter().filter(|p| p.network_id == self.active_network_id)
    }

    /// Toggles like on a post by id and persists the change to SQLite.
    pub fn toggle_like(&mut self, post_id: &PostId) {
        let post_opt = self
            .post_index_by_id
            .get(post_id)
            .copied()
            .and_then(|idx| self.posts.get_mut(idx));

        if let Some(post) = post_opt {
            post.toggle_like();
            if let Some(ref db) = self.db {
                let _ = db.update_post_like(post_id, post.likes, post.is_liked);
            }
        } else if let Some(post) = self.posts.iter_mut().find(|p| &p.id == post_id) {
            post.toggle_like();
            if let Some(ref db) = self.db {
                let _ = db.update_post_like(post_id, post.likes, post.is_liked);
            }
        }
    }

    /// Returns posts matching the active network and optional search query filter.
    pub fn filtered_posts<'a>(&'a self, query: &'a str) -> impl Iterator<Item = &'a Post> {
        let q = query.trim().to_lowercase();
        self.posts.iter().filter(move |p| {
            if p.network_id != self.active_network_id {
                return false;
            }
            if q.is_empty() {
                return true;
            }
            p.content.to_lowercase().contains(&q)
                || p.author.name.to_lowercase().contains(&q)
                || p.author.handle.to_lowercase().contains(&q)
        })
    }
    /// Returns all grounding documents for the currently active network.
    pub fn current_documents(&self) -> impl Iterator<Item = &GroundingDocument> {
        self.documents.iter().filter(|d| d.network_id == self.active_network_id)
    }

    /// Adds a new grounding document to the current network and persists to SQLite.
    pub fn add_document(&mut self, title: &str, content: &str) -> Result<String> {
        let trimmed_title = title.trim();
        let trimmed_content = content.trim();

        if trimmed_title.is_empty() || trimmed_content.is_empty() {
            return Err(AppError::EmptyContent);
        }

        let doc_id = format!("doc_{}", Uuid::new_v4().simple());
        let doc = GroundingDocument::new(doc_id.clone(), self.active_network_id.clone(), trimmed_title, trimmed_content);

        if let Some(ref db) = self.db {
            db.save_document(&doc)?;
        }

        self.documents.insert(0, doc);
        Ok(doc_id)
    }

    /// Deletes a grounding document by id.
    pub fn delete_document(&mut self, doc_id: &str) -> Result<()> {
        if let Some(ref db) = self.db {
            db.delete_document(doc_id)?;
        }
        self.documents.retain(|d| d.id != doc_id);
        Ok(())
    }

    /// Returns a zero-copy borrowed slice of all posts currently stored in memory.
    #[inline]
    pub fn posts(&self) -> &[Post] {
        &self.posts
    }

    /// Returns a zero-copy borrowed slice of all grounding documents currently stored in memory.
    #[inline]
    pub fn documents(&self) -> &[GroundingDocument] {
        &self.documents
    }

    /// Computes the standard wikilink excerpt for a thought post.
    pub fn compute_wikilink_excerpt(post: &Post) -> String {
        let words: Vec<&str> = post
            .content
            .split_whitespace()
            .filter(|w| !w.starts_with("[[") && !w.starts_with('#'))
            .take(3)
            .collect();
        if !words.is_empty() {
            format!("{}...", words.join(" "))
        } else {
            let clean = post.id.as_str();
            let slug = clean.strip_prefix("post_").unwrap_or(clean);
            if slug.chars().count() > 6 {
                format!("#{}", slug.chars().take(4).collect::<String>())
            } else {
                clean.to_string()
            }
        }
    }

    /// Returns the total count of posts in the active network in O(1) time via precomputed index.
    #[inline]
    pub fn current_network_post_count(&self) -> usize {
        if self.post_index_by_id.len() == self.posts.len() {
            self.network_post_indices
                .get(&self.active_network_id)
                .map_or(0, Vec::len)
        } else {
            self.current_network_posts().count()
        }
    }

    /// Returns direct and linked replies to `root_id` in O(1) lookup time via precomputed index.
    #[inline]
    pub fn replies_for_post(&self, root_id: &PostId) -> Vec<&Post> {
        if self.post_index_by_id.len() == self.posts.len() {
            if let Some(indices) = self.replies_index.get(root_id) {
                return indices
                    .iter()
                    .filter_map(|&idx| self.posts.get(idx))
                    .collect();
            }
            return Vec::new();
        }
        let root_id_str = root_id.as_str();
        self.posts
            .iter()
            .filter(|reply| {
                reply.id != *root_id
                    && (reply.reply_to_id.as_deref() == Some(root_id_str)
                        || reply.linked_posts.iter().any(|id| id.as_str() == root_id_str))
            })
            .collect()
    }

    /// Rebuilds `post_index_by_id`, `network_post_indices`, `replies_index`, and `wikilink_cache` from `self.posts`.
    pub fn rebuild_indices(&mut self) {
        self.wikilink_cache.clear();
        for post in &self.posts {
            let excerpt = Self::compute_wikilink_excerpt(post);
            self.wikilink_cache.insert(post.id.as_str().to_string(), excerpt);
        }
        self.sync_post_index();
    }

    /// Synchronizes `post_index_by_id`, `network_post_indices`, and `replies_index` without re-evaluating `wikilink_cache`.
    #[inline]
    pub fn sync_post_index(&mut self) {
        self.post_index_by_id.clear();
        self.network_post_indices.clear();
        self.replies_index.clear();
        for (idx, post) in self.posts.iter().enumerate() {
            self.post_index_by_id.insert(post.id.clone(), idx);
            self.network_post_indices
                .entry(post.network_id.clone())
                .or_default()
                .push(idx);
            if let Some(ref parent_id) = post.reply_to_id {
                if parent_id != &post.id {
                    self.replies_index
                        .entry(parent_id.clone())
                        .or_default()
                        .push(idx);
                }
            }
            for linked_id in &post.linked_posts {
                if linked_id != &post.id && post.reply_to_id.as_ref() != Some(linked_id) {
                    self.replies_index
                        .entry(linked_id.clone())
                        .or_default()
                        .push(idx);
                }
            }
        }
    }

    /// Returns a borrowed reference to a Post by its strongly-typed PostId in O(1) time.
    #[inline]
    pub fn get_post(&self, id: &PostId) -> Option<&Post> {
        self.post_index_by_id
            .get(id)
            .and_then(|&idx| self.posts.get(idx))
    }

    /// Returns a borrowed reference to a Post by its string identifier in O(1) time.
    #[inline]
    pub fn get_post_by_id(&self, id: &str) -> Option<&Post> {
        let post_id = PostId::new(id);
        self.post_index_by_id
            .get(&post_id)
            .and_then(|&idx| self.posts.get(idx))
    }

    /// Resolves an Obsidian wikilink excerpt in O(1) time using the precomputed cache.
    ///
    /// If the referenced target post exists in the vault, returns the cached 3-word preview.
    /// Otherwise, generates a fallback 4-character slug without linear searching.
    pub fn resolve_wikilink_fast(&self, target_id: &str) -> String {
        let clean = target_id.trim();
        if let Some(cached) = self.wikilink_cache.get(clean) {
            return cached.clone();
        }
        if clean.starts_with("post_") {
            let slug = clean.strip_prefix("post_").unwrap_or(clean);
            if slug.chars().count() > 6 {
                let short_slug: String = slug.chars().take(4).collect();
                return format!("#{}", short_slug);
            }
        }
        clean.to_string()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_initialization() {
        let state = AppState::new_in_memory();
        assert!(!state.networks.is_empty());
        assert_eq!(state.active_network_id.as_str(), "net_philosophy");
        assert!(state.current_network().is_some());
    }

    #[test]
    fn test_store_submit_post() {
        let mut state = AppState::new_in_memory();
        let post_id = state
            .submit_post("New cognitive connection [[post_1]] #Ontology")
            .expect("Submission should succeed");

        let found = state.posts.iter().find(|p| p.id == post_id).expect("Post was inserted");
        assert_eq!(found.id, post_id);
        assert_eq!(found.linked_posts, vec![PostId::new("post_1")]);
    }

    #[test]
    fn test_store_switch_network() {
        let mut state = AppState::new_in_memory();
        let target = NetworkId::new("net_systems");
        state.switch_network(&target).expect("Network exists");
        assert_eq!(state.active_network_id, target);
    }

    #[test]
    fn test_store_persistence() {
        let db = Rc::new(Database::in_memory().expect("in-memory db"));
        let mut state = AppState::fallback_seed(Some(db.clone()));
        let post_id = state
            .submit_post("Persistent post test")
            .expect("Submission should succeed");

        let loaded = AppState::load_from_db(&db).expect("Load from db");
        assert!(loaded.posts.iter().any(|p| p.id == post_id));
    }

    #[test]
    fn test_store_theme_tokens_persistence() {
        let db = Rc::new(Database::in_memory().expect("in-memory db"));
        let mut state = AppState::fallback_seed(Some(db.clone()));
        let mut custom_tokens = crate::theme::obsidian_zinc();
        custom_tokens.accent_brand = gpui_kit::rgb(0xff00aa);
        custom_tokens.titlebar_height = gpui_kit::px(40.0);

        state.update_theme_tokens(custom_tokens.clone());
        assert_eq!(state.theme_tokens.accent_brand, gpui_kit::rgb(0xff00aa));

        let loaded = AppState::load_from_db(&db).expect("Load from db");
        assert_eq!(loaded.theme_tokens.accent_brand, gpui_kit::rgb(0xff00aa));
        assert_eq!(loaded.theme_tokens.titlebar_height, gpui_kit::px(40.0));
    }

    #[test]
    fn test_store_submit_with_attachments_and_reply_quote() {
        let mut state = AppState::new_in_memory();
        let root_id = state.submit_post("Root thought").expect("Submit root");

        // Submit reply
        let reply_id = state
            .submit_reply(&root_id, "Reply to root thought", Some("https://example.com".into()), None)
            .expect("Submit reply");

        let reply_post = state.posts.iter().find(|p| p.id == reply_id).unwrap();
        assert_eq!(reply_post.reply_to_id, Some(root_id.clone()));
        assert_eq!(reply_post.link_url, Some("https://example.com".into()));

        // Root post replies_count should have incremented
        let root_post = state.posts.iter().find(|p| p.id == root_id).unwrap();
        assert!(root_post.replies_count >= 1);

        // Submit quote
        let quote_id = state
            .submit_quote(&root_id, "Quoting thought with commentary", None, Some("/path/image.png".into()))
            .expect("Submit quote");

        let quote_post = state.posts.iter().find(|p| p.id == quote_id).unwrap();
        assert_eq!(quote_post.quote_id, Some(root_id.clone()));
        assert_eq!(quote_post.image_path, Some("/path/image.png".into()));
    }

    #[test]
    fn test_store_repost_flow() {
        let mut state = AppState::new_in_memory();
        let root_id = state.submit_post("Thought to repost").expect("Submit root");

        let repost_id = state.repost(&root_id).expect("Repost action");
        let repost_post = state.posts.iter().find(|p| p.id == repost_id).unwrap();
        assert_eq!(repost_post.repost_id, Some(root_id.clone()));
        assert_eq!(repost_post.content, "");

        let target_post = state.posts.iter().find(|p| p.id == root_id).unwrap();
        assert_eq!(target_post.reposts_count, 1);
        assert!(target_post.is_reposted);

        // Toggle un-repost
        let un_repost_id = state.repost(&root_id).expect("Un-repost action");
        assert_eq!(un_repost_id, root_id);
        let target_post_after = state.posts.iter().find(|p| p.id == root_id).unwrap();
        assert_eq!(target_post_after.reposts_count, 0);
        assert!(!target_post_after.is_reposted);
        assert!(!state.posts.iter().any(|p| p.repost_id.as_ref() == Some(&root_id)));
    }

    #[test]
    fn test_store_post_indexing_and_fast_lookup() {
        let state = AppState::new_in_memory();
        assert!(!state.posts.is_empty());
        assert_eq!(state.post_index_by_id.len(), state.posts.len());

        for post in &state.posts {
            let found_by_ref = state.get_post(&post.id);
            assert!(found_by_ref.is_some());
            assert_eq!(found_by_ref.unwrap().id, post.id);

            let found_by_str = state.get_post_by_id(post.id.as_str());
            assert!(found_by_str.is_some());
            assert_eq!(found_by_str.unwrap().id, post.id);
        }

        assert!(state.get_post_by_id("nonexistent_post_xyz").is_none());
    }

    #[test]
    fn test_store_wikilink_cache_and_resolution() {
        let state = AppState::new_in_memory();
        assert!(!state.wikilink_cache.is_empty());

        let _post_1 = state.get_post_by_id("post_1").expect("post_1 exists");
        let excerpt = state.resolve_wikilink_fast("post_1");
        assert!(excerpt.ends_with("..."));

        // Fallback for non-existent post
        let fallback_short = state.resolve_wikilink_fast("post_12345678");
        assert_eq!(fallback_short, "#1234");

        // Fallback for non-post plain string
        let plain = state.resolve_wikilink_fast("Ontology");
        assert_eq!(plain, "Ontology");
    }

    #[test]
    fn test_store_indices_sync_on_mutations() {
        let mut state = AppState::new_in_memory();
        let initial_count = state.posts.len();

        // 1. Submit post (human top-level thought is placed directly at index 0 without auto-spawning agent post above it)
        let post_id = state.submit_post("Brand new indexed thought").unwrap();
        assert_eq!(state.posts.len(), initial_count + 1);
        assert_eq!(state.post_index_by_id.get(&post_id), Some(&0));
        assert!(state.wikilink_cache.contains_key(post_id.as_str()));
        assert_eq!(state.post_index_by_id.len(), state.posts.len());

        // 2. Submit reply (1 post added)
        let reply_id = state.submit_reply(&post_id, "Reply to indexed thought", None, None).unwrap();
        assert_eq!(state.post_index_by_id.get(&reply_id), Some(&0));
        assert_eq!(state.post_index_by_id.get(&post_id), Some(&1));
        assert_eq!(state.post_index_by_id.len(), state.posts.len());

        // 3. Submit quote (1 post added at index 0)
        let quote_id = state.submit_quote(&post_id, "Quote indexed thought", None, None).unwrap();
        assert_eq!(state.post_index_by_id.get(&quote_id), Some(&0));
        assert_eq!(state.post_index_by_id.len(), state.posts.len());

        // 4. Repost & un-repost
        let repost_id = state.repost(&post_id).unwrap();
        assert_eq!(state.post_index_by_id.get(&repost_id), Some(&0));
        assert_eq!(state.post_index_by_id.len(), state.posts.len());

        let _ = state.repost(&post_id).unwrap(); // un-repost
        assert_eq!(state.post_index_by_id.len(), state.posts.len());
        assert!(!state.post_index_by_id.contains_key(&repost_id));
    }

    #[test]
    fn test_store_zero_copy_accessors() {
        let state = AppState::new_in_memory();
        let posts_slice = state.posts();
        let docs_slice = state.documents();

        assert_eq!(posts_slice.len(), state.posts.len());
        assert_eq!(docs_slice.len(), state.documents.len());
        assert!(std::ptr::eq(posts_slice.as_ptr(), state.posts.as_ptr()));
        assert!(std::ptr::eq(docs_slice.as_ptr(), state.documents.as_ptr()));
    }

    #[test]
    fn test_submit_post_does_not_auto_spawn_agent_above_user_post() {
        let mut state = AppState::new_in_memory();
        let initial_len = state.posts.len();

        let post_id = state.submit_post("My independent epistemic reflection").unwrap();

        assert_eq!(state.posts.len(), initial_len + 1);
        assert_eq!(state.posts[0].id, post_id);
        assert_eq!(state.posts[0].content, "My independent epistemic reflection");
        assert_eq!(state.posts[0].author.id, state.active_profile_id);
        assert_eq!(state.posts[0].replies_count, 0);
    }

    #[test]
    fn test_agents_are_human_named_and_scoped_per_network() {
        let mut state = AppState::new_in_memory();

        // 1. Verify NO profile uses bot/GPT/Node naming
        for prof in &state.user_profiles {
            assert!(!prof.name.contains("GPT"), "Agent must not contain GPT: {}", prof.name);
            assert!(!prof.name.contains("Bot"), "Agent must not contain Bot: {}", prof.name);
            assert!(!prof.name.contains("-Node"), "Agent must not contain -Node: {}", prof.name);
            assert!(!prof.handle.contains("_ai"), "Handle must not contain _ai: {}", prof.handle);
            assert!(!prof.handle.contains("_bot"), "Handle must not contain _bot: {}", prof.handle);
        }

        // 2. Network 1: `net_philosophy` has Sokrates, Baruch de Spinoza, Simone Weil
        assert_eq!(state.active_network_id.as_str(), "net_philosophy");
        let phil_agents: Vec<String> = state
            .current_network_agents()
            .into_iter()
            .map(|p| p.handle.clone())
            .collect();
        assert_eq!(phil_agents, vec!["@sokrates", "@baruch_spinoza", "@simone_weil"]);

        // 3. Switch to Network 2: `net_systems` -> Elena Rostova, Marcus Vance, Kenji Takahashi
        state.switch_network(&NetworkId::new("net_systems")).unwrap();
        let sys_agents: Vec<String> = state
            .current_network_agents()
            .into_iter()
            .map(|p| p.handle.clone())
            .collect();
        assert_eq!(sys_agents, vec!["@elena_rostova", "@marcus_vance", "@kenji_t"]);

        // 4. Switch to Network 3: `net_creative` -> Ursula K. Vance, Julian Sterling
        state.switch_network(&NetworkId::new("net_creative")).unwrap();
        let creative_agents: Vec<String> = state
            .current_network_agents()
            .into_iter()
            .map(|p| p.handle.clone())
            .collect();
        assert_eq!(creative_agents, vec!["@ursula_v", "@julian_sterling"]);
    }

    #[test]
    fn test_create_custom_agent_and_persona_lab_separation() {
        let db = Rc::new(Database::in_memory().expect("in-memory db"));
        let mut state = AppState::fallback_seed(Some(db.clone()));

        // Create a custom human-like agent in `net_philosophy`
        let new_agent_id = state
            .create_network_agent("Ki Hajar Dewantara", "kihajar", "Philosopher of Education")
            .expect("create custom agent");

        // Verify it is in `net_philosophy` agents but NOT in `net_systems` agents
        assert!(state.current_network_agents().iter().any(|p| p.id == new_agent_id));
        assert!(!state.current_network_personas().iter().any(|p| p.id == new_agent_id));

        state.switch_network(&NetworkId::new("net_systems")).unwrap();
        assert!(!state.current_network_agents().iter().any(|p| p.id == new_agent_id));

        // Switch back to `net_philosophy` and customize the agent's full psychological & linguistic profile
        state.switch_network(&NetworkId::new("net_philosophy")).unwrap();
        let mut agent_prof = state
            .user_profiles
            .iter()
            .find(|p| p.id == new_agent_id)
            .cloned()
            .unwrap();
        agent_prof.avatar_path = Some("C:/avatars/kihajar.png".to_string());
        agent_prof.config.mbti = crate::domain::MbtiType::Enfj;
        agent_prof.config.enneagram = crate::domain::EnneagramType::Type1w2;
        agent_prof.config.primary_language = "Indonesian".to_string();
        agent_prof.config.hobbies = "Gamelan orchestration, educational philosophy, journalism".to_string();
        state.update_profile(agent_prof).unwrap();

        // Create a custom controllable human persona in Persona Lab and switch to it
        let new_persona_id = state
            .create_human_persona("Tan Malaka", "@tan_malaka", "Madilog Dialectician")
            .expect("create human persona");
        assert!(state.current_network_personas().iter().any(|p| p.id == new_persona_id));
        assert!(!state.current_network_agents().iter().any(|p| p.id == new_persona_id));

        state.switch_profile(&new_persona_id).unwrap();
        assert_eq!(state.active_profile_id, new_persona_id);

        // Cycle active persona and verify it never selects an autonomous agent
        for _ in 0..5 {
            let cycled = state.cycle_active_persona().expect("cycled persona");
            let prof = state.user_profiles.iter().find(|p| p.id == cycled).unwrap();
            assert!(prof.is_human_persona(), "Cycle must only select controllable human personas");
        }

        // Reload from SQLite and verify full persistence of custom agent & persona
        let reloaded = AppState::load_from_db(&db).expect("reload from db");
        let loaded_agent = reloaded
            .user_profiles
            .iter()
            .find(|p| p.id == new_agent_id)
            .expect("custom agent persisted");
        assert_eq!(loaded_agent.name, "Ki Hajar Dewantara");
        assert_eq!(loaded_agent.handle, "@kihajar");
        assert_eq!(loaded_agent.avatar_path.as_deref(), Some("C:/avatars/kihajar.png"));
        assert_eq!(loaded_agent.config.mbti, crate::domain::MbtiType::Enfj);
        assert_eq!(loaded_agent.config.enneagram, crate::domain::EnneagramType::Type1w2);
        assert_eq!(loaded_agent.config.primary_language, "Indonesian");
        assert!(loaded_agent.config.hobbies.contains("Gamelan orchestration"));
    }
}

