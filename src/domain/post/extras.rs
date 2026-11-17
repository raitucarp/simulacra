//! Engine metadata of a generated post: its mood, image prompt, the tools the agent used and
//! the sources it drew on. Persisted as one JSON column; empty for human posts.

use serde::{Deserialize, Serialize};

/// Structured-output fields and tool provenance attached to an agent post.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostExtras {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt: Option<String>,
    /// Tool calls made while composing, in order.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolUse>,
    /// Documents, posts, profiles and pages the tools returned.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceRef>,
}

impl PostExtras {
    /// Whether nothing is recorded (the column then stores `{}`).
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// Adds `source` unless an identical reference is already listed.
    pub fn add_source(&mut self, source: SourceRef) {
        if !self.sources.contains(&source) {
            self.sources.push(source);
        }
    }
}

/// One tool call: the model-facing tool name and a short summary of its arguments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ToolUse {
    pub name: String,
    pub summary: String,
    /// Whether the tool reported an error.
    pub failed: bool,
}

/// What a source reference points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// A Knowledge Vault document (`id` = document id).
    #[default]
    VaultDoc,
    /// A post (`id` = post id).
    Post,
    /// A profile (`id` = profile id).
    Profile,
    /// A web page (`id` = URL).
    Web,
}

/// A source the agent consulted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SourceRef {
    pub kind: SourceKind,
    pub id: String,
    pub title: String,
}

impl SourceRef {
    pub fn new(kind: SourceKind, id: impl Into<String>, title: impl Into<String>) -> Self {
        Self { kind, id: id.into(), title: title.into() }
    }
}
