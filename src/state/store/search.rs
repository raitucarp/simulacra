//! Timeline search filtering.

use super::AppState;
use crate::domain::Post;

impl AppState {
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
            let author = self.author_of(p);
            p.content.to_lowercase().contains(&q)
                || author.name.to_lowercase().contains(&q)
                || author.handle.to_lowercase().contains(&q)
        })
    }
}
