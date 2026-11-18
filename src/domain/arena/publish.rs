//! Publishing to the feed, pure: a debate becomes one thread (the motion by the user's
//! persona, then one post per turn by its speaker, a long turn continuing in follow-up posts
//! by the same speaker), and an experiment becomes a summary post. Each post replies to the
//! one before, so the thread reads as one continuous line.

use crate::domain::{split_into_thread, NetworkId, Post, PostId, ProfileId};
use crate::error::Result;
use chrono::{DateTime, Duration, Utc};

/// Milliseconds between consecutive posts of the thread, so reading order survives sorting.
pub const THREAD_SPACING_MS: i64 = 1;

/// One authored text of the thread, before splitting to the post limit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadPart {
    pub author: ProfileId,
    pub text: String,
}

/// The parts split to `limit` characters per post, blank ones dropped.
pub fn split_parts(parts: &[ThreadPart], limit: usize) -> Vec<ThreadPart> {
    parts
        .iter()
        .flat_map(|part| split_into_thread(&part.text, limit).into_iter().map(|text| ThreadPart { author: part.author.clone(), text }))
        .collect()
}

/// The posts of `parts` (already split) in reading order: each replies to the one before,
/// reply counters set, the first created at `start`.
pub fn build_thread(parts: &[ThreadPart], network: &NetworkId, start: DateTime<Utc>, mut next_id: impl FnMut() -> PostId) -> Result<Vec<Post>> {
    let mut posts: Vec<Post> = Vec::with_capacity(parts.len());
    for part in parts {
        let parent = posts.last().map(|post| post.id.clone());
        let mut post = Post::create_with_attachments(next_id(), network.clone(), part.author.clone(), &part.text, None, None, parent, None, None)?;
        post.created_at = start + Duration::milliseconds(posts.len() as i64 * THREAD_SPACING_MS);
        if let Some(previous) = posts.last_mut() {
            previous.replies_count = 1;
        }
        posts.push(post);
    }
    Ok(posts)
}

/// The parts of `debate`'s thread: `root` (the motion), every turn by its speaker, then
/// `closing` (the verdict, when an agent judged).
pub fn debate_parts(debate: &super::debate::Debate, root: ThreadPart, closing: Option<ThreadPart>) -> Vec<ThreadPart> {
    let turns = debate.turns.iter().map(|turn| ThreadPart { author: turn.slot.speaker.clone(), text: turn.text.clone() });
    std::iter::once(root).chain(turns).chain(closing).collect()
}
