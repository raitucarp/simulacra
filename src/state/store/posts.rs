//! Publishing new thoughts and reading the post collection.

use super::{AppState, POST_ID_PREFIX};
use crate::domain::{Post, PostId};
use crate::error::Result;
use uuid::Uuid;

impl AppState {
    /// Creates and publishes a new thought to the current network with optional attachments and relation IDs.
    pub fn submit_post_with_attachments(
        &mut self,
        content: &str,
        link_url: Option<String>,
        image_path: Option<String>,
        reply_to_id: Option<PostId>,
        quote_id: Option<PostId>,
    ) -> Result<PostId> {
        let author = self.current_profile_or_err()?;

        let post_id = Self::new_post_id();
        let post = Post::create_with_attachments(
            post_id.clone(),
            self.active_network_id.clone(),
            author,
            content,
            link_url,
            image_path,
            reply_to_id,
            quote_id,
            None,
        )?;

        self.commit_post(post)?;
        self.draft_text.clear();
        Ok(post_id)
    }

    /// Persists a validated post, bumps its parent's reply counter and prepends it.
    pub(super) fn commit_post(&mut self, post: Post) -> Result<()> {
        if let Some(ref db) = self.db {
            db.save_post(&post)?;
        }
        if let Some(parent_id) = post.reply_to_id.clone() {
            self.increment_replies_count(&parent_id);
        }
        self.insert_post_at_head(post);
        Ok(())
    }

    /// Creates and publishes a standard thought to the current network timeline.
    #[inline]
    pub fn submit_post(&mut self, content: &str) -> Result<PostId> {
        self.submit_post_with_attachments(content, None, None, None, None)
    }

    /// Returns posts matching the active network.
    pub fn current_network_posts(&self) -> impl Iterator<Item = &Post> {
        self.posts.iter().filter(|p| p.network_id == self.active_network_id)
    }

    /// Returns a zero-copy borrowed slice of all posts currently stored in memory.
    #[inline]
    pub fn posts(&self) -> &[Post] {
        &self.posts
    }

    /// Generates a fresh unique post identifier.
    pub(super) fn new_post_id() -> PostId {
        PostId::new(format!("{POST_ID_PREFIX}{}", Uuid::new_v4().simple()))
    }

    /// Caches the post's wikilink excerpt, prepends it to the timeline and resyncs indices.
    pub(super) fn insert_post_at_head(&mut self, post: Post) {
        let excerpt = Self::compute_wikilink_excerpt(&post);
        self.wikilink_cache.insert(post.id.as_str().to_string(), excerpt);
        self.posts.insert(0, post);
        self.sync_post_index();
    }

    /// Increments a parent post's reply counter (index lookup first, linear scan if unindexed).
    fn increment_replies_count(&mut self, parent_id: &PostId) {
        let parent = match self.post_index_by_id.get(parent_id).copied() {
            Some(idx) => self.posts.get_mut(idx),
            None => self.posts.iter_mut().find(|p| &p.id == parent_id),
        };
        if let Some(parent) = parent {
            parent.replies_count += 1;
            if let Some(ref db) = self.db {
                let _ = db.update_post_replies_count(parent_id, parent.replies_count);
            }
        }
    }
}
