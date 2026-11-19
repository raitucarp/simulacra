//! Conversation grouping for the main stream: replies sit under their parent (oldest first)
//! instead of floating to the top of the timeline on their own.

use super::FeedTab;
use crate::domain::{Post, PostId};
use crate::state::AppState;
use jiff::Timestamp;
use std::cmp::Reverse;
use std::collections::HashMap;

/// Recency of `posts[idx]`: creation time, ties broken by store order (lower index = newer).
type Recency = (Timestamp, Reverse<usize>);

fn recency(posts: &[&Post], idx: usize) -> Recency {
    (posts[idx].created_at, Reverse(idx))
}

/// Groups `posts` (store order, newest first) into conversations.
///
/// A conversation starts at a post whose parent is not among `posts` and lists the replies
/// depth-first, siblings oldest first, so every reply follows its parent. Conversations are
/// ordered by their most recent post, newest first. Reply cycles cannot drop posts.
pub fn group_conversations<'a>(posts: &[&'a Post]) -> Vec<Vec<&'a Post>> {
    let (roots, children) = link_parents(posts);
    let mut visited = vec![false; posts.len()];
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for start in roots.into_iter().chain(0..posts.len()) {
        if !visited[start] {
            groups.push(walk(start, &children, &mut visited));
        }
    }
    groups.sort_by_key(|group| Reverse(group.iter().map(|&idx| recency(posts, idx)).max()));
    groups
        .into_iter()
        .map(|group| group.into_iter().map(|idx| posts[idx]).collect())
        .collect()
}

/// Root positions and, per parent position, its reply positions oldest first.
fn link_parents(posts: &[&Post]) -> (Vec<usize>, HashMap<usize, Vec<usize>>) {
    let position: HashMap<&PostId, usize> =
        posts.iter().enumerate().map(|(idx, post)| (&post.id, idx)).collect();
    let mut roots = Vec::new();
    let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
    for (idx, post) in posts.iter().enumerate() {
        match post.reply_to_id.as_ref().and_then(|id| position.get(id)) {
            Some(&parent) if parent != idx => children.entry(parent).or_default().push(idx),
            _ => roots.push(idx),
        }
    }
    for replies in children.values_mut() {
        replies.sort_by_key(|&idx| recency(posts, idx));
    }
    (roots, children)
}

/// Depth-first (pre-order) walk from `start`, marking visited positions.
fn walk(start: usize, children: &HashMap<usize, Vec<usize>>, visited: &mut [bool]) -> Vec<usize> {
    let mut order = Vec::new();
    let mut stack = vec![start];
    while let Some(idx) = stack.pop() {
        if std::mem::replace(&mut visited[idx], true) {
            continue;
        }
        order.push(idx);
        if let Some(replies) = children.get(&idx) {
            stack.extend(replies.iter().rev());
        }
    }
    order
}

/// The active network's stream for `tab`: grouped conversations flattened in display order.
pub fn feed_entries(state: &AppState, tab: FeedTab) -> Vec<&Post> {
    super::ranking::ranked_feed_entries(state, tab, None)
}

/// Display position of `post_id` in the `tab` stream, if it is shown there.
pub fn feed_position(state: &AppState, tab: FeedTab, post_id: &PostId) -> Option<usize> {
    feed_entries(state, tab).iter().position(|post| &post.id == post_id)
}
