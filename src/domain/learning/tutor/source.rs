//! What a tutoring dialogue is about, the text the tutor reads, and the anchor review items
//! made from its gaps get (never free-floating: a post, the thread's first post, or a passage).

use crate::domain::post::SourceLocator;
use crate::domain::review::Anchor;
use serde::{Deserialize, Serialize};

/// Longest material the tutor reads, in characters.
pub const MAX_MATERIAL_CHARS: usize = 4_000;
/// Longest passage a document anchor quotes, in characters.
pub const MAX_PASSAGE_CHARS: usize = 600;
/// Between thread lines.
const LINE_BREAK: &str = "\n";
const AUTHOR_SEPARATOR: &str = ": ";

/// The subject of a dialogue.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TutorSource {
    Post { id: String },
    /// A thread, from its first post.
    Thread { root_id: String },
    Passage {
        document_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        locator: Option<SourceLocator>,
        text: String,
    },
    Document { id: String },
    /// A concept the learner typed.
    Concept { name: String },
}

impl TutorSource {
    /// Stable key of the variant (locale keys).
    pub const fn key(&self) -> &'static str {
        match self {
            Self::Post { .. } => "post",
            Self::Thread { .. } => "thread",
            Self::Passage { .. } => "passage",
            Self::Document { .. } => "document",
            Self::Concept { .. } => "concept",
        }
    }

    /// The post the source is (a thread's first post), if any.
    pub fn post_id(&self) -> Option<&str> {
        match self {
            Self::Post { id } | Self::Thread { root_id: id } => Some(id),
            _ => None,
        }
    }

    /// The vault document the source is in, if any.
    pub fn document_id(&self) -> Option<&str> {
        match self {
            Self::Passage { document_id: id, .. } | Self::Document { id } => Some(id),
            _ => None,
        }
    }

    /// Where review items for this dialogue's gaps are anchored; a typed concept has no
    /// source, so it offers none (the learner writes a post on it first).
    pub fn review_anchor(&self, material: &str) -> Option<Anchor> {
        match self {
            Self::Post { id } | Self::Thread { root_id: id } => Some(Anchor::Post { id: id.clone() }),
            Self::Passage { document_id, locator, text } => Some(Anchor::passage(document_id.clone(), locator.clone(), text.clone())),
            Self::Document { id } => Some(Anchor::passage(id.clone(), None, cut(material, MAX_PASSAGE_CHARS))),
            Self::Concept { .. } => None,
        }
    }
}

/// `text` trimmed and cut to `limit` characters.
pub fn cut(text: &str, limit: usize) -> String {
    text.trim().chars().take(limit).collect()
}

/// A thread as the tutor reads it: one `Author: text` line per post, cut to the limit.
pub fn thread_material(lines: &[(String, String)]) -> String {
    let joined = lines.iter().map(|(author, text)| format!("{author}{AUTHOR_SEPARATOR}{}", text.trim())).collect::<Vec<_>>().join(LINE_BREAK);
    cut(&joined, MAX_MATERIAL_CHARS)
}
