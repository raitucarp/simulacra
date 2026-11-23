//! N13: the "Still true?" view of a list of posts: the claims due for a check, the one due
//! longest first, each with when it fell due. Locked posts are never due, so never listed.

use super::rapid_log::one_line;
use crate::domain::post::still_true::due_at;
use crate::domain::{Post, PostId};
use jiff::Timestamp;

/// One claim due for a check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DueClaim {
    pub post_id: PostId,
    pub text: String,
    pub due: Timestamp,
}

/// The claims among `posts` due at `now`, longest due first.
pub fn due_claims<'a>(posts: impl IntoIterator<Item = &'a Post>, now: Timestamp) -> Vec<DueClaim> {
    let mut due: Vec<DueClaim> = posts.into_iter().filter_map(|post| Some(DueClaim { post_id: post.id.clone(), text: one_line(post), due: due_at(post).filter(|due| *due <= now)? })).collect();
    due.sort_by_key(|claim| claim.due);
    due
}
