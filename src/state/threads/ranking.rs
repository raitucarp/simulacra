//! The recommended order of the "For you" stream: conversations by their best-scored post
//! (scores come from `crate::state::recommend`), the usual recency order breaking ties.

use super::grouping::group_conversations;
use super::FeedTab;
use crate::domain::{Post, PostId};
use crate::state::AppState;
use std::collections::HashMap;
use std::rc::Rc;

/// Post scores that reorder the stream, and the generation that tells caches they changed.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FeedRanking {
    pub generation: u64,
    pub scores: Rc<HashMap<PostId, f32>>,
}

impl FeedRanking {
    /// The best score in `conversation` (lowest possible when none of its posts is scored).
    fn best(&self, conversation: &[&Post]) -> f32 {
        conversation.iter().filter_map(|post| self.scores.get(&post.id)).copied().fold(f32::MIN, f32::max)
    }
}

/// The active network's stream for `tab`, ordered by `ranking` when given (recency otherwise).
pub fn ranked_feed_entries<'a>(state: &'a AppState, tab: FeedTab, ranking: Option<&FeedRanking>) -> Vec<&'a Post> {
    let posts: Vec<&Post> = state.day_scoped(state.feed_posts()).collect();
    let mut conversations: Vec<Vec<&Post>> = group_conversations(&posts).into_iter().filter(|c| tab.includes(c)).collect();
    if let Some(ranking) = ranking {
        conversations.sort_by(|a, b| ranking.best(b).total_cmp(&ranking.best(a)));
    }
    conversations.into_iter().flatten().collect()
}

/// Display position of `post_id` in the `tab` stream ordered by `ranking`.
pub fn ranked_feed_position(state: &AppState, tab: FeedTab, ranking: Option<&FeedRanking>, post_id: &PostId) -> Option<usize> {
    ranked_feed_entries(state, tab, ranking).iter().position(|post| &post.id == post_id)
}
