//! Trending hashtag and knowledge topic entities.

use serde::{Deserialize, Serialize};

/// Represents an aggregated hashtag or knowledge theme in a network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrendingTopic {
    pub tag: String,
    pub post_count: usize,
    pub category: String,
}

impl TrendingTopic {
    /// Creates a new trending topic record.
    pub fn new(tag: impl Into<String>, post_count: usize, category: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            post_count,
            category: category.into(),
        }
    }
}
