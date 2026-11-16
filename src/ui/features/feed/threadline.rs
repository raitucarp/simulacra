//! Threadline style computation for linear feed streams.

use crate::domain::Post;
use crate::ui::molecules::ThreadlineStyle;

/// Computes threadline style for a linear sequence of thoughts in the feed stream.
///
/// Supports both reverse-chronological streams (standard feed ordering: `created_at DESC`,
/// where child replies precede parent posts) and chronological streams (e.g. thread inspection).
///
/// Threadline connectivity rules:
/// - Connects upward if the post above is a child replying to `current` (reverse-chronological)
///   or if `current` is a child replying to the post above (chronological).
/// - Connects downward if `current` is a child replying to the post below (reverse-chronological)
///   or if the post below is a child replying to `current` (chronological).
pub fn compute_stream_threadline_style(posts: &[&Post], index: usize) -> ThreadlineStyle {
    if posts.is_empty() || index >= posts.len() {
        return ThreadlineStyle::None;
    }
    let current = posts[index];
    let has_line_above = index > 0 && is_reply_pair(posts[index - 1], current);
    let has_line_below = index + 1 < posts.len() && is_reply_pair(current, posts[index + 1]);

    match (has_line_above, has_line_below) {
        (false, false) => ThreadlineStyle::None,
        (false, true) => ThreadlineStyle::ParentHasReplies,
        (true, true) => ThreadlineStyle::MiddleReply,
        (true, false) => ThreadlineStyle::LastReply,
    }
}

/// True when either adjacent post replies to the other (covers both stream orderings).
fn is_reply_pair(above: &Post, below: &Post) -> bool {
    above.reply_to_id.as_ref() == Some(&below.id) || below.reply_to_id.as_ref() == Some(&above.id)
}
