//! Incrementally maintained post indices.
//!
//! Positions are stored as *ordinals* counted from the end of the newest-first post vector
//! (`ordinal = len - 1 - position`), so the common case — publishing at the head — leaves
//! every existing entry untouched. Inserting or removing elsewhere only shifts the ordinals
//! of the posts newer than the change. [`PostIndex::rebuild`] remains for loading.

use super::post_meta::PostMeta;
use crate::domain::{NetworkId, Post, PostId};
use std::collections::HashMap;

/// Lookup tables over `AppState::posts`, kept in sync by the store's mutation methods.
#[derive(Debug, Default)]
pub struct PostIndex {
    ordinals: HashMap<PostId, usize>,
    /// Direct `reply_to_id` children per parent, as ordinals in store order (descending).
    replies: HashMap<PostId, Vec<usize>>,
    network_counts: HashMap<NetworkId, usize>,
    pub(super) meta: HashMap<PostId, PostMeta>,
    pub(super) backrefs: super::backrefs::Backrefs, // N1
    pub(super) versions: super::embedded::VersionTexts, // N1
    len: usize,
    pub(super) revision: u64,
    full_rebuilds: usize,
}

/// The parent a post is indexed under as a reply (self-replies are ignored).
fn reply_parent(post: &Post) -> Option<&PostId> {
    post.reply_to_id.as_ref().filter(|id| *id != &post.id)
}

impl PostIndex {
    /// Re-indexes every post from scratch (startup / bulk loads).
    pub fn rebuild(&mut self, posts: &[Post]) {
        *self = Self { revision: self.revision + 1, full_rebuilds: self.full_rebuilds + 1, ..Self::default() };
        for (position, post) in posts.iter().enumerate().rev() {
            self.len += 1;
            self.add(post, posts.len() - 1 - position);
        }
    }

    /// Indexes `posts[position]`, which was just inserted there.
    pub fn inserted(&mut self, posts: &[Post], position: usize) {
        let ordinal = posts.len() - 1 - position;
        self.shift_from(ordinal, true);
        self.len += 1;
        self.add(&posts[position], ordinal);
        self.revision += 1;
    }

    /// Drops `post`, which was just removed from `position`.
    pub fn removed(&mut self, post: &Post, position: usize) {
        let ordinal = self.len - 1 - position;
        self.ordinals.remove(&post.id);
        self.versions.forget(&post.id);
        if let Some(meta) = self.meta.remove(&post.id) {
            self.backrefs.remove(&post.id, &meta.refs);
        }
        if let Some(children) = reply_parent(post).and_then(|parent| self.replies.get_mut(parent)) {
            children.retain(|&child| child != ordinal);
        }
        if let Some(count) = self.network_counts.get_mut(&post.network_id) {
            *count = count.saturating_sub(1);
        }
        self.len -= 1;
        self.shift_from(ordinal, false);
        self.revision += 1;
    }

    fn add(&mut self, post: &Post, ordinal: usize) {
        self.ordinals.insert(post.id.clone(), ordinal);
        let meta = PostMeta::of(post);
        self.backrefs.add(&post.id, &meta.refs);
        self.meta.insert(post.id.clone(), meta);
        *self.network_counts.entry(post.network_id.clone()).or_default() += 1;
        if let Some(parent) = reply_parent(post) {
            let children = self.replies.entry(parent.clone()).or_default();
            let at = children.partition_point(|&child| child > ordinal);
            children.insert(at, ordinal);
        }
    }

    /// Moves every ordinal at/above `from` one step up (insert) or above `from` one step down
    /// (remove). Nothing moves for changes at the head of the timeline.
    fn shift_from(&mut self, from: usize, up: bool) {
        let shift = |ordinal: &mut usize| match up {
            true if *ordinal >= from => *ordinal += 1,
            false if *ordinal > from => *ordinal -= 1,
            _ => {}
        };
        if from >= self.len {
            return;
        }
        self.ordinals.values_mut().for_each(shift);
        self.replies.values_mut().flatten().for_each(shift);
    }

    fn to_position(&self, ordinal: usize) -> usize {
        self.len - 1 - ordinal
    }

    /// Store position of `id`.
    pub fn position(&self, id: &str) -> Option<usize> {
        self.ordinals.get(id).map(|&ordinal| self.to_position(ordinal))
    }

    /// Store positions of the direct replies of `id`, in store order.
    pub fn reply_positions(&self, id: &PostId) -> impl Iterator<Item = usize> + '_ {
        self.replies.get(id).into_iter().flatten().map(|&ordinal| self.to_position(ordinal))
    }

    /// Number of indexed posts in `network`.
    pub fn network_count(&self, network: &NetworkId) -> usize {
        self.network_counts.get(network).copied().unwrap_or_default()
    }

    /// Derived body data of post `id`.
    pub fn meta(&self, id: &str) -> Option<&PostMeta> {
        self.meta.get(id)
    }

    /// Number of indexed posts.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether no post is indexed.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Marks author data (names, handles, archived authors) or a post's place in the feed
    /// (archived, Y5) as changed: listings over the posts, such as the cached stream, must be
    /// recomputed.
    pub fn listing_changed(&mut self) {
        self.revision += 1;
    }

    /// Bumped on every insert, removal, rebuild and author change; caches over the post
    /// listing key on it.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// How many times the index was rebuilt from scratch.
    pub fn full_rebuilds(&self) -> usize {
        self.full_rebuilds
    }
}
