//! One side of a tension: a post, or one chunk (passage) of a vault document.

use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// What a side is; the key is stored with a pair.
    SideKind {
        Post => "post",
        Passage => "passage",
    }
}

/// A text that takes part in a pair.
#[derive(Debug, Clone, PartialEq)]
pub struct TensionSide {
    pub kind: SideKind,
    /// The post id or the document id.
    pub id: String,
    /// The passage's chunk index within its document (0 for posts).
    pub chunk: usize,
    /// The author's name or the document's title.
    pub label: String,
    pub text: String,
}

impl TensionSide {
    /// A post side.
    pub fn post(id: impl Into<String>, label: impl Into<String>, text: impl Into<String>) -> Self {
        Self { kind: SideKind::Post, id: id.into(), chunk: 0, label: label.into(), text: text.into() }
    }

    /// A passage side (chunk `chunk` of document `id`).
    pub fn passage(id: impl Into<String>, chunk: usize, label: impl Into<String>, text: impl Into<String>) -> Self {
        Self { kind: SideKind::Passage, id: id.into(), chunk, label: label.into(), text: text.into() }
    }

    /// Stable identity: `post:<id>` or `passage:<id>#<chunk>`.
    pub fn key(&self) -> String {
        match self.kind {
            SideKind::Post => format!("{}:{}", self.kind.key(), self.id),
            SideKind::Passage => format!("{}:{}#{}", self.kind.key(), self.id, self.chunk),
        }
    }
}
