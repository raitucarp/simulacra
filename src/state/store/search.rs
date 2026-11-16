//! Timeline search filtering over the pre-lowercased body text cached in the post index.

use super::AppState;
use crate::domain::{Post, ProfileId};
use std::collections::HashSet;

impl AppState {
    /// Authors (live or archived) whose name or handle contains the lower-cased `query`.
    fn authors_matching(&self, query: &str) -> HashSet<&ProfileId> {
        let matches = |name: &str, handle: &str| {
            name.to_lowercase().contains(query) || handle.to_lowercase().contains(query)
        };
        self.user_profiles
            .iter()
            .chain(self.archived_authors.values())
            .filter(|p| matches(&p.name, &p.handle))
            .map(|p| &p.id)
            .collect()
    }

    /// Whether `post`'s body contains the lower-cased `query` (cached text, else computed).
    fn body_matches(&self, post: &Post, query: &str) -> bool {
        match self.index.meta(post.id.as_str()) {
            Some(meta) => meta.search_text.contains(query),
            None => post.content.to_lowercase().contains(query),
        }
    }

    /// Returns posts matching the active network and optional search query filter.
    pub fn filtered_posts<'a>(&'a self, query: &'a str) -> impl Iterator<Item = &'a Post> {
        let q = query.trim().to_lowercase();
        let authors = if q.is_empty() { HashSet::new() } else { self.authors_matching(&q) };
        self.posts.iter().filter(move |p| {
            p.network_id == self.active_network_id
                && (q.is_empty() || authors.contains(&p.author_id) || self.body_matches(p, &q))
        })
    }
}
