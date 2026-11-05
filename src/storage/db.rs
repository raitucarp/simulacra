//! SQLite persistence layer for Simulacra.
//!
//! Stores network domains, profiles, thoughts/posts, likes, and settings locally.

use crate::domain::{GroundingDocument, Network, NetworkId, Post, PostId, Profile, ProfileId, ProfileRole, TrendingTopic};
use crate::error::{AppError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};

/// Primary SQLite persistence manager.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens or creates a database at the specified path and runs migrations.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path.as_ref())
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA temp_store = MEMORY;
             PRAGMA cache_size = -64000;
             PRAGMA mmap_size = 268435456;",
        )
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Opens an in-memory SQLite database (primarily for testing).
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Default database file path in the workspace or application folder.
    pub fn default_path() -> PathBuf {
        PathBuf::from("simulacra.db")
    }

    /// Initializes tables and indexes if they do not already exist,
    /// and performs safe, non-destructive column migrations.
    fn init_schema(&self) -> Result<()> {
        self.conn
            .execute_batch(
                r#"
                CREATE TABLE IF NOT EXISTS networks (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    slug TEXT NOT NULL,
                    description TEXT NOT NULL,
                    doc_count INTEGER NOT NULL DEFAULT 0,
                    agent_count INTEGER NOT NULL DEFAULT 0,
                    color_accent TEXT NOT NULL DEFAULT '#8b5cf6'
                );

                CREATE TABLE IF NOT EXISTS profiles (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    handle TEXT NOT NULL,
                    avatar_initial TEXT NOT NULL,
                    role_json TEXT NOT NULL,
                    network_id TEXT,
                    avatar_path TEXT,
                    config_json TEXT NOT NULL DEFAULT '{}'
                );

                CREATE TABLE IF NOT EXISTS posts (
                    id TEXT PRIMARY KEY,
                    network_id TEXT NOT NULL,
                    author_id TEXT NOT NULL,
                    author_json TEXT NOT NULL,
                    content TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    likes INTEGER NOT NULL DEFAULT 0,
                    is_liked INTEGER NOT NULL DEFAULT 0,
                    replies_count INTEGER NOT NULL DEFAULT 0,
                    mentions_json TEXT NOT NULL DEFAULT '[]',
                    linked_posts_json TEXT NOT NULL DEFAULT '[]',
                    reply_to_id TEXT,
                    quote_id TEXT,
                    repost_id TEXT,
                    link_url TEXT,
                    image_path TEXT,
                    reposts_count INTEGER NOT NULL DEFAULT 0,
                    is_reposted INTEGER NOT NULL DEFAULT 0
                );

                CREATE TABLE IF NOT EXISTS trending (
                    tag TEXT PRIMARY KEY,
                    post_count INTEGER NOT NULL DEFAULT 0,
                    category TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS documents (
                    id TEXT PRIMARY KEY,
                    network_id TEXT NOT NULL,
                    title TEXT NOT NULL,
                    content TEXT NOT NULL,
                    chunk_count INTEGER NOT NULL DEFAULT 1,
                    created_at TEXT NOT NULL
                );

                CREATE INDEX IF NOT EXISTS idx_posts_network ON posts(network_id);
                CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at DESC);
                CREATE INDEX IF NOT EXISTS idx_documents_network ON documents(network_id);
                "#,
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.migrate_posts_schema()?;

        self.conn
            .execute_batch(
                r#"
                CREATE INDEX IF NOT EXISTS idx_posts_reply_to ON posts(reply_to_id);
                CREATE INDEX IF NOT EXISTS idx_posts_repost_id ON posts(repost_id);
                "#,
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Safely applies non-destructive schema additions for existing user vaults.
    /// Silently ignores duplicate column errors if columns were already provisioned.
    fn migrate_posts_schema(&self) -> Result<()> {
        let migration_statements = [
            "ALTER TABLE posts ADD COLUMN reply_to_id TEXT;",
            "ALTER TABLE posts ADD COLUMN quote_id TEXT;",
            "ALTER TABLE posts ADD COLUMN repost_id TEXT;",
            "ALTER TABLE posts ADD COLUMN link_url TEXT;",
            "ALTER TABLE posts ADD COLUMN image_path TEXT;",
            "ALTER TABLE posts ADD COLUMN reposts_count INTEGER NOT NULL DEFAULT 0;",
            "ALTER TABLE posts ADD COLUMN is_reposted INTEGER NOT NULL DEFAULT 0;",
            "ALTER TABLE profiles ADD COLUMN network_id TEXT;",
            "ALTER TABLE profiles ADD COLUMN avatar_path TEXT;",
            "ALTER TABLE profiles ADD COLUMN config_json TEXT NOT NULL DEFAULT '{}';",
        ];

        for stmt in migration_statements {
            match self.conn.execute(stmt, []) {
                Ok(_) => {}
                Err(rusqlite::Error::SqliteFailure(_, Some(ref msg)))
                    if msg.contains("duplicate column") =>
                {
                    // Existing column in user vault; continue without error
                }
                Err(e) => {
                    let msg = e.to_string();
                    if msg.contains("duplicate column") {
                        // Safe to ignore duplicate column error
                    } else {
                        return Err(AppError::DatabaseError(format!(
                            "Database migration failed for statement '{}': {}",
                            stmt, e
                        )));
                    }
                }
            }
        }
        Ok(())
    }

    /// Checks if the database has any seeded networks.
    pub fn is_empty(&self) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM networks")
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let count: i64 = stmt
            .query_row([], |row| row.get(0))
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(count == 0)
    }

    /// Saves a network domain record.
    pub fn save_network(&self, net: &Network) -> Result<()> {
        self.conn
            .execute(
                r#"
                INSERT INTO networks (id, name, slug, description, doc_count, agent_count, color_accent)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                ON CONFLICT(id) DO UPDATE SET
                    name = excluded.name,
                    slug = excluded.slug,
                    description = excluded.description,
                    doc_count = excluded.doc_count,
                    agent_count = excluded.agent_count,
                    color_accent = excluded.color_accent;
                "#,
                params![
                    net.id.as_str(),
                    net.name,
                    net.slug,
                    net.description,
                    net.doc_count as i64,
                    net.agent_count as i64,
                    net.color_accent,
                ],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Loads all network domains.
    pub fn load_networks(&self) -> Result<Vec<Network>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, slug, description, doc_count, agent_count, color_accent FROM networks")
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                let slug: String = row.get(2)?;
                let description: String = row.get(3)?;
                let doc_count: i64 = row.get(4)?;
                let agent_count: i64 = row.get(5)?;
                let color_accent: String = row.get(6)?;

                Ok(Network {
                    id: NetworkId::new(id),
                    name,
                    slug,
                    description,
                    doc_count: doc_count as usize,
                    agent_count: agent_count as usize,
                    color_accent,
                })
            })
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut networks = Vec::new();
        for row in rows {
            networks.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
        }
        Ok(networks)
    }

    /// Saves an author profile (human persona or autonomous network agent) with full configuration.
    pub fn save_profile(&self, profile: &Profile) -> Result<()> {
        let role_json = serde_json::to_string(&profile.role)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let config_json = serde_json::to_string(&profile.config)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.conn
            .execute(
                r#"
                INSERT INTO profiles (id, name, handle, avatar_initial, role_json, network_id, avatar_path, config_json)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                ON CONFLICT(id) DO UPDATE SET
                    name = excluded.name,
                    handle = excluded.handle,
                    avatar_initial = excluded.avatar_initial,
                    role_json = excluded.role_json,
                    network_id = excluded.network_id,
                    avatar_path = excluded.avatar_path,
                    config_json = excluded.config_json;
                "#,
                params![
                    profile.id.as_str(),
                    profile.name,
                    profile.handle,
                    profile.avatar_initial,
                    role_json,
                    profile.network_id.as_ref().map(|n| n.as_str()),
                    profile.avatar_path.as_deref(),
                    config_json,
                ],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Deletes a profile from the database by ID.
    pub fn delete_profile(&self, profile_id: &ProfileId) -> Result<()> {
        self.conn
            .execute(
                "DELETE FROM profiles WHERE id = ?1",
                params![profile_id.as_str()],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    /// Updates the serialized `author_json` snapshot on all posts authored by `profile`.
    pub fn update_post_author_snapshots(&self, profile: &Profile) -> Result<()> {
        let author_json = serde_json::to_string(profile)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        self.conn
            .execute(
                "UPDATE posts SET author_json = ?1 WHERE author_id = ?2",
                params![author_json, profile.id.as_str()],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    /// Loads all author profiles with their network bindings, avatar paths, and psychological configs.
    pub fn load_profiles(&self) -> Result<Vec<Profile>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, handle, avatar_initial, role_json, network_id, avatar_path, config_json FROM profiles",
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                let handle: String = row.get(2)?;
                let avatar_initial: String = row.get(3)?;
                let role_json: String = row.get(4)?;
                let network_id: Option<String> = row.get(5).unwrap_or(None);
                let avatar_path: Option<String> = row.get(6).unwrap_or(None);
                let config_json: String = row
                    .get::<_, Option<String>>(7)
                    .unwrap_or(None)
                    .unwrap_or_else(|| "{}".to_string());

                let role: ProfileRole =
                    serde_json::from_str(&role_json).unwrap_or(ProfileRole::UserPrimary);
                let mut config: crate::domain::AgentConfig =
                    serde_json::from_str(&config_json).unwrap_or_default();
                if let ProfileRole::Agent { ref persona_type } = role {
                    if config.description == "Independent Thinker & Network Member"
                        && !persona_type.trim().is_empty()
                    {
                        config.description = persona_type.clone();
                    }
                }

                Ok(Profile {
                    id: ProfileId::new(id),
                    name,
                    handle,
                    avatar_initial,
                    role,
                    network_id: network_id.map(NetworkId::new),
                    avatar_path,
                    config,
                })
            })
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut profiles = Vec::new();
        for row in rows {
            profiles.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
        }
        Ok(profiles)
    }

    /// Saves or updates a post in the database with full attachments and relation metadata.
    pub fn save_post(&self, post: &Post) -> Result<()> {
        let author_json = serde_json::to_string(&post.author)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let mentions_json = serde_json::to_string(&post.mentions)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let linked_posts_json = serde_json::to_string(&post.linked_posts)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let created_at_str = post.created_at.to_rfc3339();

        self.conn
            .execute(
                r#"
                INSERT INTO posts (
                    id, network_id, author_id, author_json, content, created_at,
                    likes, is_liked, replies_count, mentions_json, linked_posts_json,
                    reply_to_id, quote_id, repost_id, link_url, image_path,
                    reposts_count, is_reposted
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)
                ON CONFLICT(id) DO UPDATE SET
                    network_id = excluded.network_id,
                    author_id = excluded.author_id,
                    author_json = excluded.author_json,
                    content = excluded.content,
                    created_at = excluded.created_at,
                    likes = excluded.likes,
                    is_liked = excluded.is_liked,
                    replies_count = excluded.replies_count,
                    mentions_json = excluded.mentions_json,
                    linked_posts_json = excluded.linked_posts_json,
                    reply_to_id = excluded.reply_to_id,
                    quote_id = excluded.quote_id,
                    repost_id = excluded.repost_id,
                    link_url = excluded.link_url,
                    image_path = excluded.image_path,
                    reposts_count = excluded.reposts_count,
                    is_reposted = excluded.is_reposted;
                "#,
                params![
                    post.id.as_str(),
                    post.network_id.as_str(),
                    post.author.id.as_str(),
                    author_json,
                    post.content,
                    created_at_str,
                    post.likes as i64,
                    if post.is_liked { 1i64 } else { 0i64 },
                    post.replies_count as i64,
                    mentions_json,
                    linked_posts_json,
                    post.reply_to_id.as_ref().map(|id| id.as_str()),
                    post.quote_id.as_ref().map(|id| id.as_str()),
                    post.repost_id.as_ref().map(|id| id.as_str()),
                    post.link_url.as_deref(),
                    post.image_path.as_deref(),
                    post.reposts_count as i64,
                    if post.is_reposted { 1i64 } else { 0i64 },
                ],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Updates like count and flag for a post.
    pub fn update_post_like(&self, post_id: &PostId, likes: usize, is_liked: bool) -> Result<()> {
        self.conn
            .execute(
                "UPDATE posts SET likes = ?1, is_liked = ?2 WHERE id = ?3",
                params![likes as i64, if is_liked { 1i64 } else { 0i64 }, post_id.as_str()],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Updates repost count and flag for a post.
    pub fn update_post_repost(&self, post_id: &PostId, reposts_count: usize, is_reposted: bool) -> Result<()> {
        self.conn
            .execute(
                "UPDATE posts SET reposts_count = ?1, is_reposted = ?2 WHERE id = ?3",
                params![reposts_count as i64, if is_reposted { 1i64 } else { 0i64 }, post_id.as_str()],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Deletes a repost referencing target post ID authored by a specific profile.
    pub fn delete_repost_by_target_and_author(
        &self,
        target_post_id: &PostId,
        author_id: &ProfileId,
    ) -> Result<()> {
        self.conn
            .execute(
                "DELETE FROM posts WHERE repost_id = ?1 AND author_id = ?2",
                params![target_post_id.as_str(), author_id.as_str()],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }


    /// Updates reply count for a post.
    pub fn update_post_replies_count(&self, post_id: &PostId, replies_count: usize) -> Result<()> {
        self.conn
            .execute(
                "UPDATE posts SET replies_count = ?1 WHERE id = ?2",
                params![replies_count as i64, post_id.as_str()],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Loads all posts ordered by creation date descending.
    pub fn load_posts(&self) -> Result<Vec<Post>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT
                    id, network_id, author_json, content, created_at,
                    likes, is_liked, replies_count, mentions_json, linked_posts_json,
                    reply_to_id, quote_id, repost_id, link_url, image_path,
                    reposts_count, is_reposted
                FROM posts
                ORDER BY created_at DESC
                "#,
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let network_id: String = row.get(1)?;
                let author_json: String = row.get(2)?;
                let content: String = row.get(3)?;
                let created_at_str: String = row.get(4)?;
                let likes: i64 = row.get(5)?;
                let is_liked: i64 = row.get(6)?;
                let replies_count: i64 = row.get(7)?;
                let mentions_json: String = row.get(8)?;
                let linked_posts_json: String = row.get(9)?;
                let reply_to_id: Option<String> = row.get(10)?;
                let quote_id: Option<String> = row.get(11)?;
                let repost_id: Option<String> = row.get(12)?;
                let link_url: Option<String> = row.get(13)?;
                let image_path: Option<String> = row.get(14)?;
                let reposts_count: i64 = row.get(15)?;
                let is_reposted: i64 = row.get(16)?;

                let author: Profile = serde_json::from_str(&author_json)
                    .unwrap_or_else(|_| Profile::new_primary("unknown", "Author", "@unknown"));
                let mentions: Vec<String> = serde_json::from_str(&mentions_json).unwrap_or_default();
                let linked_posts: Vec<PostId> = serde_json::from_str(&linked_posts_json).unwrap_or_default();
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                Ok(Post {
                    id: PostId::new(id),
                    network_id: NetworkId::new(network_id),
                    author,
                    content,
                    created_at,
                    likes: likes as usize,
                    is_liked: is_liked != 0,
                    replies_count: replies_count as usize,
                    mentions,
                    linked_posts,
                    reply_to_id: reply_to_id.map(PostId::new),
                    quote_id: quote_id.map(PostId::new),
                    repost_id: repost_id.map(PostId::new),
                    link_url,
                    image_path,
                    reposts_count: reposts_count as usize,
                    is_reposted: is_reposted != 0,
                })
            })
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut posts = Vec::new();
        for row in rows {
            posts.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
        }
        Ok(posts)
    }

    /// Loads trending topics.
    pub fn load_trending(&self) -> Result<Vec<TrendingTopic>> {
        let mut stmt = self
            .conn
            .prepare("SELECT tag, post_count, category FROM trending ORDER BY post_count DESC")
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let tag: String = row.get(0)?;
                let post_count: i64 = row.get(1)?;
                let category: String = row.get(2)?;

                Ok(TrendingTopic {
                    tag,
                    post_count: post_count as usize,
                    category,
                })
            })
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut list = Vec::new();
        for row in rows {
            list.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
        }
        Ok(list)
    }

    /// Saves a trending topic.
    pub fn save_trending(&self, topic: &TrendingTopic) -> Result<()> {
        self.conn
            .execute(
                r#"
                INSERT INTO trending (tag, post_count, category)
                VALUES (?1, ?2, ?3)
                ON CONFLICT(tag) DO UPDATE SET
                    post_count = excluded.post_count,
                    category = excluded.category;
                "#,
                params![topic.tag, topic.post_count as i64, topic.category],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Gets a setting value by key.
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM settings WHERE key = ?1")
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut rows = stmt
            .query([key])
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(row) = rows.next().map_err(|e| AppError::DatabaseError(e.to_string()))? {
            let val: String = row.get(0).map_err(|e| AppError::DatabaseError(e.to_string()))?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    /// Sets a setting value by key.
    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn
            .execute(
                r#"
                INSERT INTO settings (key, value)
                VALUES (?1, ?2)
                ON CONFLICT(key) DO UPDATE SET value = excluded.value;
                "#,
                params![key, value],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Loads persisted design system theme tokens from SQLite.
    pub fn load_theme_tokens(&self) -> Result<Option<crate::theme::ThemeTokens>> {
        if let Some(json_str) = self.get_setting("theme_tokens")? {
            let tokens: crate::theme::ThemeTokens = serde_json::from_str(&json_str)
                .map_err(|e| AppError::DatabaseError(format!("Failed to parse theme tokens: {}", e)))?;
            Ok(Some(tokens))
        } else {
            Ok(None)
        }
    }

    /// Persists design system theme tokens to SQLite without hardcoding.
    pub fn save_theme_tokens(&self, tokens: &crate::theme::ThemeTokens) -> Result<()> {
        let json_str = serde_json::to_string(tokens)
            .map_err(|e| AppError::DatabaseError(format!("Failed to serialize theme tokens: {}", e)))?;
        self.set_setting("theme_tokens", &json_str)
    }

    /// Saves a grounding document into the knowledge vault.
    pub fn save_document(&self, doc: &GroundingDocument) -> Result<()> {
        let created_at_str = doc.created_at.to_rfc3339();
        self.conn
            .execute(
                r#"
                INSERT INTO documents (id, network_id, title, content, chunk_count, created_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ON CONFLICT(id) DO UPDATE SET
                    network_id = excluded.network_id,
                    title = excluded.title,
                    content = excluded.content,
                    chunk_count = excluded.chunk_count,
                    created_at = excluded.created_at;
                "#,
                params![
                    doc.id,
                    doc.network_id.as_str(),
                    doc.title,
                    doc.content,
                    doc.chunk_count as i64,
                    created_at_str,
                ],
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Loads all grounding documents for a specific network domain.
    pub fn load_documents_for_network(&self, network_id: &NetworkId) -> Result<Vec<GroundingDocument>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, network_id, title, content, chunk_count, created_at
                FROM documents
                WHERE network_id = ?1
                ORDER BY created_at DESC
                "#,
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let rows = stmt
            .query_map([network_id.as_str()], |row| {
                let id: String = row.get(0)?;
                let net_id: String = row.get(1)?;
                let title: String = row.get(2)?;
                let content: String = row.get(3)?;
                let chunk_count: i64 = row.get(4)?;
                let created_at_str: String = row.get(5)?;

                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                Ok(GroundingDocument {
                    id,
                    network_id: NetworkId::new(net_id),
                    title,
                    content,
                    chunk_count: chunk_count as usize,
                    created_at,
                })
            })
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut docs = Vec::new();
        for row in rows {
            docs.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
        }
        Ok(docs)
    }

    /// Loads all documents across all networks.
    pub fn load_all_documents(&self) -> Result<Vec<GroundingDocument>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, network_id, title, content, chunk_count, created_at
                FROM documents
                ORDER BY created_at DESC
                "#,
            )
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let net_id: String = row.get(1)?;
                let title: String = row.get(2)?;
                let content: String = row.get(3)?;
                let chunk_count: i64 = row.get(4)?;
                let created_at_str: String = row.get(5)?;

                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                Ok(GroundingDocument {
                    id,
                    network_id: NetworkId::new(net_id),
                    title,
                    content,
                    chunk_count: chunk_count as usize,
                    created_at,
                })
            })
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut docs = Vec::new();
        for row in rows {
            docs.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
        }
        Ok(docs)
    }

    /// Deletes a document from the vault.
    pub fn delete_document(&self, doc_id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM documents WHERE id = ?1", params![doc_id])
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_kv_store() {
        let db = Database::in_memory().expect("in-memory db");
        assert_eq!(db.get_setting("custom_key").unwrap(), None);

        db.set_setting("custom_key", "value_1").unwrap();
        assert_eq!(db.get_setting("custom_key").unwrap(), Some("value_1".to_string()));

        // Conflict update
        db.set_setting("custom_key", "value_2").unwrap();
        assert_eq!(db.get_setting("custom_key").unwrap(), Some("value_2".to_string()));
    }

    #[test]
    fn test_db_theme_tokens_persistence() {
        let db = Database::in_memory().expect("in-memory db");
        assert_eq!(db.load_theme_tokens().unwrap(), None);

        let mut custom_tokens = crate::theme::obsidian_zinc();
        custom_tokens.titlebar_height = gpui_kit::px(42.0);
        custom_tokens.accent_brand = gpui_kit::rgb(0xff00ff);

        db.save_theme_tokens(&custom_tokens).expect("save theme tokens");

        let loaded = db.load_theme_tokens().unwrap().expect("loaded tokens");
        assert_eq!(loaded.titlebar_height, gpui_kit::px(42.0));
        assert_eq!(loaded.accent_brand, gpui_kit::rgb(0xff00ff));
        assert_eq!(loaded, custom_tokens);
    }

    #[test]
    fn test_db_theme_tokens_corrupted_json() {
        let db = Database::in_memory().expect("in-memory db");
        db.set_setting("theme_tokens", "{ invalid json }").unwrap();
        let result = db.load_theme_tokens();
        assert!(result.is_err());
    }

    #[test]
    fn test_db_post_attachments_and_repost_persistence() {
        let db = Database::in_memory().expect("in-memory db");
        let author = Profile::new_primary("u1", "Architect", "@arch");
        let post = Post::create_with_attachments(
            "p_db_test",
            "net_philosophy",
            author.clone(),
            "Persisting attachments and quote",
            Some("https://simulacra.ai".to_string()),
            Some("C:/images/test.png".to_string()),
            Some(PostId::new("p_parent")),
            Some(PostId::new("p_quoted")),
            None,
        )
        .expect("create post");

        db.save_post(&post).expect("save post");

        let loaded = db.load_posts().expect("load posts");
        let retrieved = loaded.iter().find(|p| p.id.as_str() == "p_db_test").expect("found post");

        assert_eq!(retrieved.link_url, Some("https://simulacra.ai".to_string()));
        assert_eq!(retrieved.image_path, Some("C:/images/test.png".to_string()));
        assert_eq!(retrieved.reply_to_id, Some(PostId::new("p_parent")));
        assert_eq!(retrieved.quote_id, Some(PostId::new("p_quoted")));

        // Test update_post_repost
        db.update_post_repost(&retrieved.id, 5, true).expect("update repost");
        let reloaded = db.load_posts().expect("reload posts");
        let reloaded_post = reloaded.iter().find(|p| p.id.as_str() == "p_db_test").unwrap();
        assert_eq!(reloaded_post.reposts_count, 5);
        assert!(reloaded_post.is_reposted);

        // Test update_post_replies_count
        db.update_post_replies_count(&retrieved.id, 9).expect("update replies count");
        let reloaded2 = db.load_posts().expect("reload posts 2");
        let reloaded_post2 = reloaded2.iter().find(|p| p.id.as_str() == "p_db_test").unwrap();
        assert_eq!(reloaded_post2.replies_count, 9);
    }

    #[test]
    fn test_db_schema_migration_from_legacy_vault() {
        // Create an in-memory database with the old schema (pre-M2)
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE posts (
                id TEXT PRIMARY KEY,
                network_id TEXT NOT NULL,
                author_id TEXT NOT NULL,
                author_json TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                likes INTEGER NOT NULL DEFAULT 0,
                is_liked INTEGER NOT NULL DEFAULT 0,
                replies_count INTEGER NOT NULL DEFAULT 0,
                mentions_json TEXT NOT NULL DEFAULT '[]',
                linked_posts_json TEXT NOT NULL DEFAULT '[]'
            );
            "#,
        ).unwrap();

        // Insert legacy post
        conn.execute(
            "INSERT INTO posts (id, network_id, author_id, author_json, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params!["old_1", "net1", "u1", "{}", "Legacy vault thought", "2026-01-01T00:00:00Z"]
        ).unwrap();

        // Wrap into Database and run init_schema
        let db = Database { conn };
        db.init_schema().expect("Migration must succeed without error");

        // Running init_schema a second time must also succeed idempotently
        db.init_schema().expect("Idempotent migration must succeed");

        let loaded = db.load_posts().expect("Load migrated posts");
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id.as_str(), "old_1");
        assert_eq!(loaded[0].reposts_count, 0);
        assert_eq!(loaded[0].reply_to_id, None);
        assert_eq!(loaded[0].link_url, None);
    }

    #[test]
    fn test_db_delete_repost_by_target_and_author_scoping() {
        let db = Database::in_memory().unwrap();
        let net = NetworkId::new("net_phil");
        let target_id = PostId::new("post_root");
        let author_a = Profile::new_primary("author_a", "Alice", "@alice");
        let author_b = Profile::new_primary("author_b", "Bob", "@bob");

        let post_a = Post::create_with_attachments(
            PostId::new("repost_a"),
            net.clone(),
            author_a.clone(),
            "",
            None,
            None,
            None,
            None,
            Some(target_id.clone()),
        ).unwrap();

        let post_b = Post::create_with_attachments(
            PostId::new("repost_b"),
            net,
            author_b.clone(),
            "",
            None,
            None,
            None,
            None,
            Some(target_id.clone()),
        ).unwrap();

        db.save_post(&post_a).unwrap();
        db.save_post(&post_b).unwrap();

        // Delete Alice's repost only
        db.delete_repost_by_target_and_author(&target_id, &author_a.id).unwrap();

        let remaining = db.load_posts().unwrap();
        assert!(!remaining.iter().any(|p| p.id.as_str() == "repost_a"));
        assert!(remaining.iter().any(|p| p.id.as_str() == "repost_b"));
    }

    #[test]
    fn test_db_high_performance_pragmas() {
        let temp_dir = std::env::temp_dir().join(format!("simulacra_pragma_test_{}.db", uuid::Uuid::new_v4()));
        let db = Database::open(&temp_dir).expect("Database::open");

        let journal_mode: String = db
            .conn
            .query_row("PRAGMA journal_mode;", [], |row| row.get(0))
            .expect("query journal_mode");
        assert_eq!(journal_mode.to_lowercase(), "wal");

        let synchronous: i64 = db
            .conn
            .query_row("PRAGMA synchronous;", [], |row| row.get(0))
            .expect("query synchronous");
        assert_eq!(synchronous, 1); // 1 = NORMAL

        let temp_store: i64 = db
            .conn
            .query_row("PRAGMA temp_store;", [], |row| row.get(0))
            .expect("query temp_store");
        assert_eq!(temp_store, 2); // 2 = MEMORY

        let cache_size: i64 = db
            .conn
            .query_row("PRAGMA cache_size;", [], |row| row.get(0))
            .expect("query cache_size");
        assert_eq!(cache_size, -64000);

        let mmap_size: i64 = db
            .conn
            .query_row("PRAGMA mmap_size;", [], |row| row.get(0))
            .expect("query mmap_size");
        assert!(mmap_size >= 268435456);

        drop(db);
        let _ = std::fs::remove_file(&temp_dir);
        let _ = std::fs::remove_file(format!("{}-wal", temp_dir.display()));
        let _ = std::fs::remove_file(format!("{}-shm", temp_dir.display()));
    }
}


