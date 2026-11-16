//! Scale check: ~5k posts must be indexed and listed incrementally, not rebuilt per change
//! or per frame. Time bounds are generous (debug-ish test profile, shared CI machines).

use crate::domain::Post;
use crate::state::{AppState, FeedTab, StreamCache};
use std::time::{Duration, Instant};

const POST_COUNT: usize = 5_000;
/// Every tenth post replies to the post published just before it.
const REPLY_EVERY: usize = 10;
const FRAMES: usize = 1_000;
const INSERT_BUDGET: Duration = Duration::from_secs(5);
const FRAMES_BUDGET: Duration = Duration::from_millis(500);

/// A store without a vault, filled with `POST_COUNT` posts through the incremental path.
fn large_state() -> (AppState, Duration) {
    let mut state = AppState::new_in_memory();
    state.db = None;
    let (network, author) = (state.active_network_id.clone(), state.active_profile_id.clone());
    let started = Instant::now();
    for i in 0..POST_COUNT {
        let id = format!("scale_{i}");
        let body = format!("thought {i} about #Scale and [[scale_{}]]", i.saturating_sub(1));
        let post = match (i % REPLY_EVERY == 0, state.posts.first()) {
            (true, Some(parent)) => {
                let parent = parent.id.clone();
                Post::create_reply(id, network.clone(), author.clone(), &body, parent, None, None).unwrap()
            }
            _ => Post::create(id, network.clone(), author.clone(), &body).unwrap(),
        };
        state.insert_post_at(0, post);
    }
    (state, started.elapsed())
}

#[test]
fn five_thousand_inserts_update_the_index_incrementally() {
    let (state, elapsed) = large_state();
    assert_eq!(state.index.full_rebuilds(), 1, "only the initial seed load rebuilds");
    assert_eq!(state.index.len(), state.posts.len());
    assert!(elapsed < INSERT_BUDGET, "{POST_COUNT} inserts took {elapsed:?}");
    let newest = &state.posts[0];
    assert_eq!(state.post_position(&newest.id), Some(0));
    assert!(state.index.meta(newest.id.as_str()).is_some_and(|m| m.tokens.len() > 1));
}

#[test]
fn stream_listing_is_built_once_per_change_not_per_frame() {
    let (mut state, _) = large_state();
    let mut cache = StreamCache::default();
    let started = Instant::now();
    for _ in 0..FRAMES {
        let (positions, _) = cache.positions(&state, FeedTab::ForYou, "");
        assert!(positions.len() > POST_COUNT);
    }
    let elapsed = started.elapsed();
    assert_eq!(cache.builds(), 1, "unchanged frames reuse the listing");
    assert!(elapsed < FRAMES_BUDGET, "{FRAMES} frames took {elapsed:?}");

    state.submit_post("one more").unwrap();
    let (positions, rebuilt) = cache.positions(&state, FeedTab::ForYou, "");
    assert!(rebuilt);
    assert_eq!(state.posts[positions[0]].content, "one more");
    let (_, rebuilt) = cache.positions(&state, FeedTab::ForYou, "#scale");
    assert!(rebuilt, "a new query rebuilds");
    assert_eq!(cache.builds(), 3);
}
