//! Where a card's knowledge comes from: a post, a place in a vault document, or a sentence
//! the learner typed or pasted. Every card cites one, so a review can lead back to context.

use crate::domain::embedding::EmbeddingOwner;
use crate::domain::post::SourceLocator;
use crate::domain::PostId;
use serde::{Deserialize, Serialize};

/// The cited source of a card.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CardSource {
    /// A post of the network (`id` = post id).
    Post { id: String },
    /// A Knowledge Vault document, optionally narrowed to a chapter, paragraph or sentence.
    Document {
        id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        locator: Option<SourceLocator>,
    },
    /// A free-standing sentence (no stored source).
    Sentence { text: String },
}

impl CardSource {
    pub fn post(id: &PostId) -> Self {
        Self::Post { id: id.as_str().to_string() }
    }

    pub fn document(id: impl Into<String>, locator: Option<SourceLocator>) -> Self {
        Self::Document { id: id.into(), locator }
    }

    /// The embedded item this source is, for meaning-based neighbours (none for a sentence).
    pub fn owner(&self) -> Option<EmbeddingOwner> {
        match self {
            Self::Post { id } => Some(EmbeddingOwner::post(&PostId::new(id.clone()))),
            Self::Document { id, .. } => Some(EmbeddingOwner::document(id.clone())),
            Self::Sentence { .. } => None,
        }
    }

    /// A key that groups cards drawn from the same item (interleaving spreads these apart).
    pub fn group_key(&self) -> String {
        match self {
            Self::Post { id } => format!("post:{id}"),
            Self::Document { id, .. } => format!("document:{id}"),
            Self::Sentence { text } => format!("sentence:{text}"),
        }
    }
}
