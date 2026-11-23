//! The raw view (W3): the posts as plain text, the way they were written (marks, links and
//! tags as typed), one block per post under a line naming its author and when it was
//! written. A locked post shows only its placeholder line, so "copy all" never copies a
//! sealed text.

use crate::domain::{Post, PostId};

/// One post's block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawBlock {
    pub id: PostId,
    /// "author · when".
    pub heading: String,
    /// The text as written; the placeholder line for a locked post.
    pub text: String,
    pub locked: bool,
}

/// Separates a block's author from its time.
pub const HEADING_SEPARATOR: &str = " · ";

/// The blocks of `posts` in their order; `heading` names a post's author and time and
/// `locked` is what a locked post shows.
pub fn raw_blocks<'a>(posts: impl IntoIterator<Item = &'a Post>, heading: impl Fn(&Post) -> String, locked: &str) -> Vec<RawBlock> {
    posts
        .into_iter()
        .map(|post| RawBlock {
            id: post.id.clone(),
            heading: heading(post),
            text: if post.is_locked() { locked.to_string() } else { post.content.trim_end().to_string() },
            locked: post.is_locked(),
        })
        .collect()
}

/// Every block as one text, blocks apart by a blank line (what "copy all" copies).
pub fn raw_text(blocks: &[RawBlock]) -> String {
    blocks.iter().map(|block| format!("{}\n{}", block.heading, block.text)).collect::<Vec<_>>().join("\n\n")
}
