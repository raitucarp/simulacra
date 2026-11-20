//! What a manual agent run asks for: a reply (the default), or a new top-level post that
//! starts from an optional seed (free text and/or a vault document).

use crate::domain::document::{DocumentId, GroundingDocument};
use crate::domain::graph::excerpt;

/// Longest document excerpt, in characters, handed to the model as a seed.
pub const SEED_EXCERPT_CHARS: usize = 1200;

/// What a manual agent trigger generates.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TriggerMode {
    /// Reply to the network's top post (the historical behaviour).
    #[default]
    Reply,
    /// Start a new conversation from `seed` (which may be empty).
    NewPost { seed: PostSeed },
}

/// The requested seed of a new post: a topic and/or a vault document by id.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PostSeed {
    pub topic: String,
    pub document: Option<DocumentId>,
}

/// A seed document resolved for the prompt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedDocument {
    pub id: String,
    pub title: String,
    /// The opening of the document, whitespace collapsed, at most [`SEED_EXCERPT_CHARS`].
    pub excerpt: String,
}

/// A seed with its document looked up (an unknown document id is dropped).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResolvedSeed {
    pub topic: String,
    pub document: Option<SeedDocument>,
}

impl PostSeed {
    /// A seed of free text only.
    pub fn topic(topic: impl Into<String>) -> Self {
        Self { topic: topic.into(), document: None }
    }

    /// A seed of one vault document only.
    pub fn document(id: impl Into<DocumentId>) -> Self {
        Self { topic: String::new(), document: Some(id.into()) }
    }

    /// The seed with its document found among `documents`.
    pub fn resolve(&self, documents: &[GroundingDocument]) -> ResolvedSeed {
        let document = self.document.as_ref().and_then(|id| documents.iter().find(|doc| doc.id == id.as_str()));
        ResolvedSeed {
            topic: self.topic.trim().to_string(),
            document: document.map(|doc| SeedDocument {
                id: doc.id.clone(),
                title: doc.title.trim().to_string(),
                excerpt: excerpt(&doc.content, SEED_EXCERPT_CHARS),
            }),
        }
    }
}

impl ResolvedSeed {
    /// Whether neither a topic nor a document was given (the agent picks its own subject).
    pub fn is_empty(&self) -> bool {
        self.topic.is_empty() && self.document.is_none()
    }
}
