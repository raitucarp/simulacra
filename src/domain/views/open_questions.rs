//! N18: the open questions of a list of posts, the one asked longest ago first, each with when
//! it was asked; a locked one (which cannot be made a question, but may have been stored as
//! one) is listed only as its placeholder.

use super::rapid_log::one_line;
use crate::domain::post::open_question::is_open;
use crate::domain::{Post, PostId};
use jiff::Timestamp;

/// One open question.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenRow {
    pub post_id: PostId,
    /// Its text on one line (`None`: locked).
    pub text: Option<String>,
    pub asked: Timestamp,
}

/// The open questions among `posts`, oldest first.
pub fn open_rows<'a>(posts: impl IntoIterator<Item = &'a Post>) -> Vec<OpenRow> {
    let mut rows: Vec<OpenRow> = posts.into_iter().filter(|post| is_open(post)).map(|post| OpenRow { post_id: post.id.clone(), text: (!post.is_locked()).then(|| one_line(post)), asked: post.created_at }).collect();
    rows.sort_by_key(|row| row.asked);
    rows
}
