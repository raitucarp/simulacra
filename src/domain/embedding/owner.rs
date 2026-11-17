//! What an embedding belongs to: a post or a vault document, by id.

use crate::domain::PostId;
use std::fmt;

/// The kind of item a vector was computed from (stored as its wire key).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OwnerKind {
    Post,
    Document,
}

impl OwnerKind {
    pub const ALL: [OwnerKind; 2] = [Self::Post, Self::Document];

    /// Stable key in the `embeddings.owner_kind` column. Never rename one.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Post => "post",
            Self::Document => "document",
        }
    }

    /// The kind stored under `key`.
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.key() == key)
    }
}

/// One embedded item.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmbeddingOwner {
    pub kind: OwnerKind,
    pub id: String,
}

impl EmbeddingOwner {
    pub fn post(id: &PostId) -> Self {
        Self { kind: OwnerKind::Post, id: id.as_str().to_string() }
    }

    pub fn document(id: impl Into<String>) -> Self {
        Self { kind: OwnerKind::Document, id: id.into() }
    }
}

impl fmt::Display for EmbeddingOwner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.kind.key(), self.id)
    }
}
