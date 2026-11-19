//! State management, store container, and seed data factories.

pub mod forge;
pub mod graph;
pub mod mock;
pub mod recommend;
pub mod seed;
pub mod semantic;
pub mod store;
pub mod threads;

pub use store::{
    AppState, NavigationTab, NetworkNameError, ThreadAnchor, ThreadDraft, WikilinkTarget,
    MAX_NETWORK_NAME_CHARS,
};
pub use threads::{
    build_conversation, feed_entries, feed_position, group_conversations, ranked_feed_entries, ranked_feed_position,
    Conversation, FeedRanking, FeedTab, ReplyBranch, ReplyLimits, StreamCache,
};
