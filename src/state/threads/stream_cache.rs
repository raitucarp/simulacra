//! Memoized stream listing: grouping conversations is O(n log n), so it runs when the posts,
//! network, tab, day filter or search query change — not on every frame.

use super::{feed_entries, FeedTab};
use crate::domain::{NetworkId, Post};
use crate::state::AppState;
use chrono::NaiveDate;
use std::rc::Rc;

/// Everything a stream listing depends on.
#[derive(Debug, Clone, PartialEq, Eq)]
struct StreamKey {
    revision: u64,
    len: usize,
    network: NetworkId,
    tab: FeedTab,
    day: Option<NaiveDate>,
    query: String,
}

impl StreamKey {
    fn of(state: &AppState, tab: FeedTab, query: &str) -> Self {
        Self {
            revision: state.index.revision(),
            len: state.posts.len(),
            network: state.active_network_id.clone(),
            tab,
            day: state.feed_day_filter,
            query: query.to_string(),
        }
    }
}

/// The stream's posts in display order: grouped conversations, or flat search matches.
pub fn stream_posts<'a>(state: &'a AppState, tab: FeedTab, query: &'a str) -> Vec<&'a Post> {
    if query.is_empty() {
        feed_entries(state, tab)
    } else {
        state.day_scoped(state.filtered_posts(query)).collect()
    }
}

/// Caches the store positions of the stream's posts.
#[derive(Debug, Default)]
pub struct StreamCache {
    key: Option<StreamKey>,
    positions: Rc<[usize]>,
    builds: usize,
}

impl StreamCache {
    /// Store positions of the stream in display order, and whether they were just rebuilt.
    pub fn positions(&mut self, state: &AppState, tab: FeedTab, query: &str) -> (Rc<[usize]>, bool) {
        let query = query.trim();
        let key = StreamKey::of(state, tab, query);
        let rebuilt = self.key.as_ref() != Some(&key);
        if rebuilt {
            let posts = stream_posts(state, tab, query);
            self.positions = posts.iter().filter_map(|post| state.post_position(&post.id)).collect();
            self.key = Some(key);
            self.builds += 1;
        }
        (Rc::clone(&self.positions), rebuilt)
    }

    /// How many times the listing was computed.
    pub fn builds(&self) -> usize {
        self.builds
    }
}
