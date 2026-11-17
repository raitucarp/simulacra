//! Where a human post came from (an import, a quick capture) and the exact place in a
//! source document a cited passage sits.

use serde::{Deserialize, Serialize};

/// How a post entered the network, when not typed into a composer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrigin {
    /// Cut from an imported file.
    Import,
    /// Dumped through quick capture, paste or drop; listed in the Inbox.
    Capture,
}

/// A position inside a cited document (one-based, as people count).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SourceLocator {
    pub chapter: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentence: Option<u32>,
}
