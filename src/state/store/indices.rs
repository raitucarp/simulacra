//! Precomputed post indices for O(1) lookups by id, network and thread parent.

use super::AppState;
use crate::domain::{Post, PostId};

impl AppState {
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

    /// Returns the direct replies (`reply_to_id == root_id`) in store order via the precomputed index.
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
        self.posts
            .iter()
            .filter(|reply| reply.id != *root_id && reply.reply_to_id.as_ref() == Some(root_id))
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

    /// Synchronizes `post_index_by_id`, `network_post_indices`, and `replies_index` (direct
    /// `reply_to_id` children only) without re-evaluating `wikilink_cache`.
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
            // Only explicit `reply_to_id` relations are replies; quotes and wikilinks are not.
            if let Some(parent_id) = post.reply_to_id.as_ref().filter(|id| *id != &post.id) {
                self.replies_index.entry(parent_id.clone()).or_default().push(idx);
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
}
