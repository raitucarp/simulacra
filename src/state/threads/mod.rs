//! Read-side thread models: conversation grouping for the stream, the "For you" /
//! "Dialectic" stream filters, the recommended "For you" order, and the conversation view of
//! a selected post.

mod conversation;
mod grouping;
mod ranking;
mod stream_cache;

pub use conversation::{build_conversation, Conversation, ReplyBranch, ReplyLimits};
pub use grouping::{feed_entries, feed_position, group_conversations};
pub use ranking::{ranked_feed_entries, ranked_feed_position, FeedRanking};
pub use stream_cache::{stream_posts, StreamCache};

use crate::domain::Post;

/// Stream tabs of the main timeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FeedTab {
    /// Every conversation of the active network.
    #[default]
    ForYou,
    /// Only debates: conversations with replies, or containing a quote.
    Dialectic,
    /// Captured posts (quick capture, paste, drop): the second brain's inbox.
    Inbox,
}

impl FeedTab {
    /// Every tab, in display order.
    pub const ALL: [Self; 3] = [Self::ForYou, Self::Dialectic, Self::Inbox];

    /// Whether a grouped conversation belongs to this tab.
    pub fn includes(self, conversation: &[&Post]) -> bool {
        match self {
            Self::ForYou => true,
            Self::Dialectic => conversation.len() > 1 || conversation.iter().any(|p| p.is_quote()),
            Self::Inbox => conversation.iter().any(|p| crate::state::AppState::is_capture(p)),
        }
    }
}

#[cfg(test)]
mod tests;
