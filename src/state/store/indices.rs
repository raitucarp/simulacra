//! Post lookups by id, network and thread parent, backed by the incremental [`PostIndex`].

use super::AppState;
use crate::domain::{Post, PostId};

impl AppState {
    /// Whether the index covers `posts` (false only if `posts` was mutated directly).
    #[inline]
    fn index_in_sync(&self) -> bool {
        self.index.len() == self.posts.len()
    }

    /// Returns the total count of posts in the active network in O(1) time via precomputed index.
    #[inline]
    pub fn current_network_post_count(&self) -> usize {
        if self.index_in_sync() {
            self.index.network_count(&self.active_network_id)
        } else {
            self.current_network_posts().count()
        }
    }

    /// Returns the direct replies (`reply_to_id == root_id`) in store order via the precomputed index.
    #[inline]
    pub fn replies_for_post(&self, root_id: &PostId) -> Vec<&Post> {
        if self.index_in_sync() {
            return self.index.reply_positions(root_id).filter_map(|idx| self.posts.get(idx)).collect();
        }
        self.posts
            .iter()
            .filter(|reply| reply.id != *root_id && reply.reply_to_id.as_ref() == Some(root_id))
            .collect()
    }

    /// Re-indexes every post from scratch (load / seed). Single mutations update incrementally.
    pub fn rebuild_indices(&mut self) {
        self.index.rebuild(&self.posts);
    }

    /// Store position of `id` (index lookup, then a linear scan if the index is stale).
    #[inline]
    pub fn post_position(&self, id: &PostId) -> Option<usize> {
        self.index
            .position(id)
            .filter(|&idx| self.posts.get(idx).is_some_and(|p| &p.id == id))
            .or_else(|| self.posts.iter().position(|p| &p.id == id))
    }

    /// Returns a borrowed reference to a Post by its strongly-typed PostId in O(1) time.
    #[inline]
    pub fn get_post(&self, id: &PostId) -> Option<&Post> {
        self.get_post_by_id(id.as_str())
    }

    /// Returns a borrowed reference to a Post by its string identifier in O(1) time.
    #[inline]
    pub fn get_post_by_id(&self, id: &str) -> Option<&Post> {
        self.index.position(id).and_then(|idx| self.posts.get(idx)).filter(|p| p.id.as_str() == id)
    }

    /// Inserts `post` at store `position` and indexes it incrementally.
    pub(crate) fn insert_post_at(&mut self, position: usize, post: Post) {
        let position = position.min(self.posts.len());
        self.posts.insert(position, post);
        self.index.inserted(&self.posts, position);
    }

    /// Removes the post at store `position` and unindexes it incrementally.
    pub(crate) fn remove_post_at(&mut self, position: usize) -> Post {
        let post = self.posts.remove(position);
        self.index.removed(&post, position);
        post
    }
}
