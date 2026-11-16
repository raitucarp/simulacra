//! Read-side thread models: conversation grouping for the stream, the "For you" /
//! "Dialectic" stream filters, and the conversation view of a selected post.

mod conversation;
mod grouping;

pub use conversation::{build_conversation, Conversation, ReplyBranch, ReplyLimits};
pub use grouping::{feed_entries, feed_position, group_conversations};

use crate::domain::Post;

/// Stream tabs of the main timeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FeedTab {
    /// Every conversation of the active network.
    #[default]
    ForYou,
    /// Only debates: conversations with replies, or containing a quote.
    Dialectic,
}

impl FeedTab {
    /// Both tabs, in display order.
    pub const ALL: [Self; 2] = [Self::ForYou, Self::Dialectic];

    /// Whether a grouped conversation belongs to this tab.
    pub fn includes(self, conversation: &[&Post]) -> bool {
        match self {
            Self::ForYou => true,
            Self::Dialectic => conversation.len() > 1 || conversation.iter().any(|p| p.is_quote()),
        }
    }
}

#[cfg(test)]
mod tests;
