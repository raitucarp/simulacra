//! Grounding document entity for NotebookLM-style vector memory.

use super::ids::NetworkId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Document stored in a network's knowledge vault for grounding agents and semantic retrieval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroundingDocument {
    pub id: String,
    pub network_id: NetworkId,
    pub title: String,
    pub content: String,
    pub chunk_count: usize,
    pub created_at: DateTime<Utc>,
}

impl GroundingDocument {
    /// Instantiates a new document and computes its chunk count.
    pub fn new(
        id: impl Into<String>,
        network_id: impl Into<NetworkId>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        let content_str = content.into();
        // Naive 300-word chunker count for demonstration and indexing
        let word_count = content_str.split_whitespace().count();
        let chunks = (word_count / 150).max(1);

        Self {
            id: id.into(),
            network_id: network_id.into(),
            title: title.into(),
            content: content_str,
            chunk_count: chunks,
            created_at: Utc::now(),
        }
    }
}
