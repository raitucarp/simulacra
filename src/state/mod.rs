//! State management, store container, and seed data factories.

pub mod forge;
pub mod graph;
pub mod mock;
pub mod recommend;
pub mod search; // Q3: the search executor
pub mod seed;
pub mod semantic;
pub mod store;
pub mod threads;
pub mod unlocked; // X4: the session's unlocked posts

pub use store::{
    AppState, NavigationTab, NetworkNameError, PostEdit, ThreadAnchor, ThreadDraft, WikilinkTarget,
    MAX_NETWORK_NAME_CHARS,
};
pub use threads::{
    build_conversation, feed_entries, feed_position, group_conversations, ranked_feed_entries, ranked_feed_position,
    Conversation, FeedRanking, FeedTab, ReplyBranch, ReplyLimits, StreamCache,
};
