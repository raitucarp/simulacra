//! Reactions to every post, keyed by post, with the same changes as [`PostReactions`].

use super::{PostReactions, Reaction};
use crate::domain::{PostId, ProfileId};
use jiff::Timestamp;
use std::collections::HashMap;

/// Every post's reactions (posts without any have no entry).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReactionBook {
    posts: HashMap<PostId, PostReactions>,
}

impl ReactionBook {
    /// Groups stored rows by post, keeping their order within each post.
    pub fn from_rows(rows: impl IntoIterator<Item = Reaction>) -> Self {
        let mut grouped: HashMap<PostId, Vec<Reaction>> = HashMap::new();
        for row in rows {
            grouped.entry(row.post_id.clone()).or_default().push(row);
        }
        let posts = grouped.into_iter().map(|(post, rows)| (post, PostReactions::from_rows(rows))).filter(|(_, set)| !set.is_empty()).collect();
        Self { posts }
    }

    /// The reactions to `post` (`None` when it has none).
    pub fn of(&self, post: &PostId) -> Option<&PostReactions> {
        self.posts.get(post)
    }

    /// Every row of every post (posts in no particular order).
    pub fn rows(&self) -> impl Iterator<Item = &Reaction> {
        self.posts.values().flat_map(|set| set.rows().iter())
    }

    /// Every row `profile` gave, across posts.
    pub fn given_by<'a>(&'a self, profile: &'a ProfileId) -> impl Iterator<Item = &'a Reaction> + 'a {
        self.rows().filter(move |row| row.profile_id == *profile)
    }

    /// Forgets every reaction to `post` (it was deleted).
    pub fn forget(&mut self, post: &PostId) -> bool {
        self.posts.remove(post).is_some()
    }

    /// Applies `change` to `post`'s reactions, dropping the post's entry once it is empty.
    fn change(&mut self, post: &PostId, change: impl FnOnce(&mut PostReactions) -> Option<Reaction>) -> Option<Reaction> {
        let set = self.posts.entry(post.clone()).or_default();
        let changed = change(set);
        if set.is_empty() {
            self.posts.remove(post);
        }
        changed
    }

    /// See [`PostReactions::add`].
    pub fn add(&mut self, post: &PostId, profile: &ProfileId, emoji: &str, at: Timestamp) -> Option<Reaction> {
        self.change(post, |set| set.add(post, profile, emoji, at))
    }

    /// See [`PostReactions::remove`].
    pub fn remove(&mut self, post: &PostId, profile: &ProfileId, emoji: &str, at: Timestamp) -> Option<Reaction> {
        self.change(post, |set| set.remove(profile, emoji, at))
    }

    /// See [`PostReactions::toggle`].
    pub fn toggle(&mut self, post: &PostId, profile: &ProfileId, emoji: &str, at: Timestamp) -> Option<Reaction> {
        self.change(post, |set| set.toggle(post, profile, emoji, at))
    }

    /// See [`PostReactions::clap`].
    pub fn clap(&mut self, post: &PostId, profile: &ProfileId, times: u32, at: Timestamp) -> Option<Reaction> {
        self.change(post, |set| set.clap(post, profile, times, at))
    }

    /// See [`PostReactions::unclap`].
    pub fn unclap(&mut self, post: &PostId, profile: &ProfileId, times: u32, at: Timestamp) -> Option<Reaction> {
        self.change(post, |set| set.unclap(profile, times, at))
    }

    /// Replaces (or, with a zero count, removes) one stored row, as an import does.
    pub fn put(&mut self, row: Reaction) {
        let post = row.post_id.clone();
        self.change(&post, |set| set.put(row));
    }
}
