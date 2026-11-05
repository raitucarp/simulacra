//! Network domain entity representing an isolated thought-space.

use super::ids::NetworkId;
use serde::{Deserialize, Serialize};

/// Represents a distinct domain of thought / problem space.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Network {
    pub id: NetworkId,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub doc_count: usize,
    pub agent_count: usize,
    pub color_accent: String,
}

impl Network {
    /// Creates a new network domain.
    pub fn new(
        id: impl Into<NetworkId>,
        name: impl Into<String>,
        slug: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            slug: slug.into(),
            description: description.into(),
            doc_count: 0,
            agent_count: 0,
            color_accent: "#8b5cf6".to_string(),
        }
    }

    /// Sets the accent color for this network.
    pub fn with_accent(mut self, color: impl Into<String>) -> Self {
        self.color_accent = color.into();
        self
    }

    /// Sets document and agent counts.
    pub fn with_counts(mut self, doc_count: usize, agent_count: usize) -> Self {
        self.doc_count = doc_count;
        self.agent_count = agent_count;
        self
    }
}
