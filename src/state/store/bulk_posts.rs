//! Publishing many posts at once (imports, captures): one batch write to the vault, one
//! splice into the timeline and one index rebuild instead of a write and index update each.

use super::AppState;
use crate::domain::{Post, PostId};
use crate::error::Result;

impl AppState {
    /// A fresh unique post id (for posts built outside the store, such as imports).
    pub fn fresh_post_id() -> PostId {
        Self::new_post_id()
    }

    /// Publishes `posts`, given in reading order (oldest first, parents before replies), as
    /// the newest posts of the timeline. Reply counters inside the batch must already be set.
    pub fn publish_batch(&mut self, posts: Vec<Post>) -> Result<usize> {
        let count = posts.len();
        if count == 0 {
            return Ok(0);
        }
        if let Some(ref db) = self.db {
            db.save_posts(posts.clone())?;
        }
        let mut timeline: Vec<Post> = posts.into_iter().rev().collect();
        timeline.append(&mut self.posts);
        self.posts = timeline;
        self.rebuild_indices();
        self.sync_network_counts();
        Ok(count)
    }
}
