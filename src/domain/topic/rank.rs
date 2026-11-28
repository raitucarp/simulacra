//! Trending tags counted from the posts themselves: how many thoughts used each `#tag`.

use super::TrendingTopic;
use crate::domain::Post;
use jiff::Timestamp;
use std::collections::{HashMap, HashSet};

/// "Trending" looks at this many recent days, or at every post when they hold no tag.
pub const TRENDING_WINDOW_DAYS: i64 = 30;

/// Per tag (case-folded): the spelling first seen, how many posts used it and the latest use.
type Tally = HashMap<String, (String, usize, Timestamp)>;

/// The `limit` most used tags of `posts` written since `since`, or of all `posts` when that
/// window has none. Each post counts once per tag; ties go to the most recently used tag.
pub fn rank_tags(posts: &[&Post], since: Timestamp, limit: usize) -> Vec<TrendingTopic> {
    let recent: Vec<&Post> = posts.iter().copied().filter(|post| post.created_at >= since).collect();
    let tally = match tally(&recent) {
        tally if tally.is_empty() => self::tally(posts),
        tally => tally,
    };
    let mut ranked: Vec<(String, usize, Timestamp)> = tally.into_values().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)).then_with(|| a.0.cmp(&b.0)));
    ranked.into_iter().take(limit).map(|(tag, count, _)| TrendingTopic::new(tag, count, String::new())).collect()
}

fn tally(posts: &[&Post]) -> Tally {
    let mut tally = Tally::new();
    for post in posts {
        let mut seen = HashSet::new();
        for tag in Post::parse_hashtags(&post.content) {
            let key = tag.to_lowercase();
            if !seen.insert(key.clone()) {
                continue;
            }
            let entry = tally.entry(key).or_insert((tag, 0, post.created_at));
            entry.1 += 1;
            entry.2 = entry.2.max(post.created_at);
        }
    }
    tally
}
