//! Turning a plan into posts (pure): each draft becomes a root post, later parts of its
//! thread reply to the part before, every post cites the source and carries the import
//! origin, and creation times rise in reading order.

use super::ImportPlan;
use crate::domain::post::lock::{read_lock_block, LOCKED_PLACEHOLDER};
use crate::domain::post::{PostExtras, PostOrigin};
use crate::domain::{NetworkId, Post, PostId, ProfileId};
use crate::error::Result;
use jiff::Timestamp;

/// Milliseconds between consecutive imported posts, so reading order survives sorting by time.
pub const IMPORT_POST_SPACING_MS: i64 = 1;

/// The posts of `plan` in reading order (parents before their replies), published by
/// `author` into `network`, the first created at `start`. `next_id` mints post ids.
pub fn build_posts(
    plan: &ImportPlan,
    author: &ProfileId,
    network: &NetworkId,
    start: Timestamp,
    origin: PostOrigin,
    mut next_id: impl FnMut() -> PostId,
) -> Result<Vec<Post>> {
    let mut posts: Vec<Post> = Vec::with_capacity(plan.drafts.iter().map(|draft| draft.thread.len()).sum());
    for draft in &plan.drafts {
        let mut parent: Option<PostId> = None;
        for text in &draft.thread {
            let id = next_id();
            let lock = read_lock_block(text); // X6: an exported locked post comes back locked, its seal unchanged
            let body = if lock.is_some() { LOCKED_PLACEHOLDER } else { text.as_str() };
            let mut post = Post::create_with_attachments(id.clone(), network.clone(), author.clone(), body, None, None, parent.clone(), None, None)?;
            post.created_at = start + jiff::SignedDuration::from_millis(posts.len() as i64 * IMPORT_POST_SPACING_MS);
            post.extras = PostExtras { sources: vec![draft.source.clone()], origin: Some(origin), lock, ..PostExtras::default() };
            if parent.is_some() {
                if let Some(previous) = posts.last_mut() {
                    previous.replies_count = 1;
                }
            }
            parent = Some(id);
            posts.push(post);
        }
    }
    Ok(posts)
}
