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

    /// Persists a validated post (with the user's default auto-archive moment, Y6), bumps its
    /// parent's reply counter and prepends it.
    pub(super) fn commit_post(&mut self, post: Post) -> Result<()> {
        let post = self.with_pinned_embeds(self.with_replied_version(self.with_default_archive(post))); // V12, N1
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

    /// Prepends the post to the timeline, indexing it incrementally.
    pub(super) fn insert_post_at_head(&mut self, post: Post) {
        self.insert_post_at(0, post);
    }

    /// Persists an agent reply, bumps its parent's reply counter and inserts it right after
    /// the parent (or at the head when the parent is unknown).
    pub fn insert_reply_after_parent(&mut self, reply: Post) -> Result<()> {
        let reply = self.with_replied_version(self.without_agent_symbols(reply)); // S2: agents never carry symbols; V12
        if let Some(ref db) = self.db {
            db.save_post(&reply)?;
        }
        let parent_position = reply.reply_to_id.as_ref().and_then(|id| self.post_position(id));
        if let Some(parent_id) = reply.reply_to_id.clone().filter(|_| parent_position.is_some()) {
            self.increment_replies_count(&parent_id);
        }
        self.insert_post_at(parent_position.map_or(0, |idx| idx + 1), reply);
        Ok(())
    }

    /// Increments a parent post's reply counter (index lookup first, linear scan if unindexed).
    fn increment_replies_count(&mut self, parent_id: &PostId) {
        let parent = self.post_position(parent_id).and_then(|idx| self.posts.get_mut(idx));
        if let Some(parent) = parent {
            parent.replies_count += 1;
            if let Some(ref db) = self.db {
                let _ = db.update_post_replies_count(parent_id, parent.replies_count);
            }
        }
    }
}
