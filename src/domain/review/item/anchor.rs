//! What a review item is anchored to: a post of the stream (any author: agents, personas,
//! the user, anonymous accounts) or a passage of a vault document. The anchor is one face of
//! the item, so a review always shows the source itself, never a copy of it.

use crate::domain::embedding::EmbeddingOwner;
use crate::domain::post::SourceLocator;
use crate::domain::PostId;
use serde::{Deserialize, Serialize};

/// Prefix of a post anchor key.
const POST_KEY: &str = "post:";
/// Prefix of a document anchor key.
const DOCUMENT_KEY: &str = "document:";

/// The source a review item is anchored to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Anchor {
    /// A post (`id` = post id); the item always shows the post's current text.
    Post { id: String },
    /// A passage of a Knowledge Vault document, kept as quoted when it was picked.
    Passage {
        document_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        locator: Option<SourceLocator>,
        text: String,
    },
}

impl Anchor {
    pub fn post(id: &PostId) -> Self {
        Self::Post { id: id.as_str().to_string() }
    }

    pub fn passage(document_id: impl Into<String>, locator: Option<SourceLocator>, text: impl Into<String>) -> Self {
        Self::Passage { document_id: document_id.into(), locator, text: text.into() }
    }

    /// The post this anchor is, if it is one.
    pub fn post_id(&self) -> Option<PostId> {
        match self {
            Self::Post { id } => Some(PostId::new(id.clone())),
            Self::Passage { .. } => None,
        }
    }

    /// The embedded item behind the anchor (meaning-based quiz distractors).
    pub fn owner(&self) -> EmbeddingOwner {
        match self {
            Self::Post { id } => EmbeddingOwner::post(&PostId::new(id.clone())),
            Self::Passage { document_id, .. } => EmbeddingOwner::document(document_id.clone()),
        }
    }

    /// A key that groups items drawn from one post or one document (stored as a column so
    /// "is this post being learnt" is one lookup; interleaving spreads equal keys apart).
    pub fn key(&self) -> String {
        match self {
            Self::Post { id } => format!("{POST_KEY}{id}"),
            Self::Passage { document_id, .. } => format!("{DOCUMENT_KEY}{document_id}"),
        }
    }
}
