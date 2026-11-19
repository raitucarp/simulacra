//! Threads of an export: each selected post filed under the root of its reply chain.

use crate::domain::{Post, PostId};
use std::collections::HashMap;

/// Longest reply chain followed (a cycle in corrupt data must not loop forever).
const MAX_CHAIN: usize = 512;

/// The root of `post`'s reply chain within `by_id` (the post itself when it starts one).
pub fn thread_root<'a>(post: &'a Post, by_id: &HashMap<&str, &'a Post>) -> &'a Post {
    let mut current = post;
    for _ in 0..MAX_CHAIN {
        match current.reply_to_id.as_ref().filter(|parent| **parent != current.id).and_then(|parent| by_id.get(parent.as_str())) {
            Some(parent) => current = parent,
            None => break,
        }
    }
    current
}

/// `selected` grouped by thread root, each thread in time order, threads by first post.
pub(super) fn group_threads(all: &[Post], selected: &[&Post]) -> Vec<(PostId, Vec<Post>)> {
    let by_id: HashMap<&str, &Post> = all.iter().map(|p| (p.id.as_str(), p)).collect();
    let mut groups: Vec<(PostId, Vec<Post>)> = Vec::new();
    let mut slots: HashMap<PostId, usize> = HashMap::new();
    for post in selected {
        let root = thread_root(post, &by_id).id.clone();
        match slots.get(&root) {
            Some(&slot) => groups[slot].1.push((*post).clone()),
            None => {
                slots.insert(root.clone(), groups.len());
                groups.push((root, vec![(*post).clone()]));
            }
        }
    }
    groups.iter_mut().for_each(|(_, posts)| posts.sort_by(|a, b| a.created_at.cmp(&b.created_at).then_with(|| a.id.cmp(&b.id))));
    groups.sort_by(|a, b| a.1[0].created_at.cmp(&b.1[0].created_at).then_with(|| a.0.cmp(&b.0)));
    groups
}
