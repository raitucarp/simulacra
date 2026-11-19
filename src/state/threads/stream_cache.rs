//! Memoized stream listing: grouping conversations is O(n log n), so it runs when the posts,
//! network, tab, day filter or search query change — not on every frame.

use super::{ranked_feed_entries, FeedRanking, FeedTab};
use crate::domain::{NetworkId, Post};
use crate::state::AppState;
use jiff::civil::Date;
use std::rc::Rc;

/// Everything a stream listing depends on.
#[derive(Debug, Clone, PartialEq, Eq)]
struct StreamKey {
    revision: u64,
    len: usize,
    network: NetworkId,
    tab: FeedTab,
    day: Option<Date>,
    query: String,
    /// Generation of the ranking the listing was ordered by (`None`: newest first).
    ranking: Option<u64>,
}

impl StreamKey {
    fn of(state: &AppState, tab: FeedTab, query: &str, ranking: Option<&FeedRanking>) -> Self {
        Self {
            revision: state.index.revision(),
            len: state.posts.len(),
            network: state.active_network_id.clone(),
            tab,
            day: state.feed_day_filter,
            query: query.to_string(),
            ranking: ranking.map(|r| r.generation),
        }
    }
}

/// The stream's posts in display order: grouped conversations, or flat search matches.
pub fn stream_posts<'a>(state: &'a AppState, tab: FeedTab, query: &'a str) -> Vec<&'a Post> {
    ranked_stream_posts(state, tab, query, None)
}

/// [`stream_posts`] with the grouped conversations ordered by `ranking` when given.
pub fn ranked_stream_posts<'a>(state: &'a AppState, tab: FeedTab, query: &'a str, ranking: Option<&FeedRanking>) -> Vec<&'a Post> {
    if query.is_empty() {
        ranked_feed_entries(state, tab, ranking)
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
        self.ranked_positions(state, tab, query, None)
    }

    /// [`Self::positions`] with the stream ordered by `ranking` when given.
    pub fn ranked_positions(&mut self, state: &AppState, tab: FeedTab, query: &str, ranking: Option<&FeedRanking>) -> (Rc<[usize]>, bool) {
        let query = query.trim();
        let key = StreamKey::of(state, tab, query, ranking);
        let rebuilt = self.key.as_ref() != Some(&key);
        if rebuilt {
            let posts = ranked_stream_posts(state, tab, query, ranking);
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
