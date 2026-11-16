//! State management, store container, and seed data factories.

pub mod graph;
pub mod mock;
pub mod seed;
pub mod store;
pub mod threads;

pub use store::{
    AppState, NavigationTab, NetworkNameError, ThreadAnchor, ThreadDraft, WikilinkTarget,
    MAX_NETWORK_NAME_CHARS,
};
pub use threads::{
    build_conversation, feed_entries, feed_position, group_conversations, Conversation, FeedTab,
    ReplyBranch, ReplyLimits, StreamCache,
};
