//! Typed vault operations mirroring [`Database`]: writes are queued, reads round-trip.

use super::Vault;
use crate::domain::{GroundingDocument, Network, Post, PostId, Profile, ProfileId, TrendingTopic};
use crate::error::Result;
use crate::storage::{Database, SettingKey};
use crate::theme::ThemeSettings;

impl Vault {
    /// Queues an upsert of `post`.
    pub fn save_post(&self, post: &Post) -> Result<()> {
        let post = post.clone();
        self.write(move |db| db.save_post(&post))
    }

    /// Queues upserting `posts` in one transaction (all or nothing).
    pub fn save_posts(&self, posts: Vec<Post>) -> Result<()> {
        self.write(move |db| db.save_posts(&posts))
    }

    /// Queues deleting `post_id` and its instant reposts.
    pub fn delete_post_with_reposts(&self, post_id: &PostId) -> Result<()> {
        let id = post_id.clone();
        self.write(move |db| db.delete_post_with_reposts(&id))
    }

    /// Queues an upsert of `profile`.
    pub fn save_profile(&self, profile: &Profile) -> Result<()> {
        let profile = profile.clone();
        self.write(move |db| db.save_profile(&profile))
    }

    /// Queues archiving `profile` on its posts, then deleting the profile.
    pub fn archive_and_delete_profile(&self, profile: &Profile) -> Result<()> {
        let profile = profile.clone();
        self.write(move |db| {
            db.archive_post_author(&profile)?;
            db.delete_profile(&profile.id)
        })
    }

    /// Queues remembering that the seed profile `id` was deleted.
    pub fn dismiss_seed(&self, id: &ProfileId) -> Result<()> {
        let id = id.clone();
        self.write(move |db| db.dismiss_seed(&id))
    }

    /// Queues an upsert of `network`.
    pub fn save_network(&self, network: &Network) -> Result<()> {
        let network = network.clone();
        self.write(move |db| db.save_network(&network))
    }

    /// Queues an upsert of `doc`.
    pub fn save_document(&self, doc: &GroundingDocument) -> Result<()> {
        let doc = doc.clone();
        self.write(move |db| db.save_document(&doc))
    }

    /// Queues deleting document `doc_id`.
    pub fn delete_document(&self, doc_id: &str) -> Result<()> {
        let doc_id = doc_id.to_string();
        self.write(move |db| db.delete_document(&doc_id))
    }

    /// Queues an upsert of `topic`.
    pub fn save_trending(&self, topic: &TrendingTopic) -> Result<()> {
        let topic = topic.clone();
        self.write(move |db| db.save_trending(&topic))
    }

    /// Queues the like counter and flag of `post_id`.
    pub fn update_post_like(&self, post_id: &PostId, likes: usize, is_liked: bool) -> Result<()> {
        let id = post_id.clone();
        self.write(move |db| db.update_post_like(&id, likes, is_liked))
    }

    /// Queues the repost counter and flag of `post_id`.
    pub fn update_post_repost(&self, post_id: &PostId, count: usize, is_reposted: bool) -> Result<()> {
        let id = post_id.clone();
        self.write(move |db| db.update_post_repost(&id, count, is_reposted))
    }

    /// Queues the reply counter of `post_id`.
    pub fn update_post_replies_count(&self, post_id: &PostId, count: usize) -> Result<()> {
        let id = post_id.clone();
        self.write(move |db| db.update_post_replies_count(&id, count))
    }

    /// Queues deleting `author_id`'s repost of `target`.
    pub fn delete_repost_by_target_and_author(&self, target: &PostId, author_id: &ProfileId) -> Result<()> {
        let (target, author) = (target.clone(), author_id.clone());
        self.write(move |db| db.delete_repost_by_target_and_author(&target, &author))
    }

    /// Queues a settings upsert.
    pub fn set_setting(&self, key: SettingKey, value: &str) -> Result<()> {
        let value = value.to_string();
        self.write(move |db| db.set_setting(key, &value))
    }

    /// Queues persisting the theme settings.
    pub fn save_theme_settings(&self, settings: &ThemeSettings) -> Result<()> {
        let settings = settings.clone();
        self.write(move |db| db.save_theme_settings(&settings))
    }

    /// Queues persisting every preference category.
    pub fn save_preferences(&self, prefs: &crate::domain::preferences::Preferences) -> Result<()> {
        let prefs = prefs.clone();
        self.write(move |db| db.save_preferences(&prefs))
    }

    /// Reads a setting (after every queued write).
    pub fn get_setting(&self, key: SettingKey) -> Result<Option<String>> {
        self.read(move |db| db.get_setting(key))
    }

    /// Reads every post (after every queued write).
    pub fn load_posts(&self) -> Result<Vec<Post>> {
        self.read(Database::load_posts)
    }

    /// Reads every network (after every queued write).
    pub fn load_networks(&self) -> Result<Vec<Network>> {
        self.read(Database::load_networks)
    }

    /// Reads every profile (after every queued write).
    pub fn load_profiles(&self) -> Result<Vec<Profile>> {
        self.read(Database::load_profiles)
    }
}
