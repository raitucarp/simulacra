//! Tool calls and source references of an agent post (`extras.tools`, `extras.sources`).

use super::super::provenance::SourceLocator;
use serde::{Deserialize, Serialize};

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
    /// The cited place inside a document (imported passages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<SourceLocator>,
}

impl SourceRef {
    pub fn new(kind: SourceKind, id: impl Into<String>, title: impl Into<String>) -> Self {
        Self { kind, id: id.into(), title: title.into(), locator: None }
    }

    /// The reference narrowed to a place inside the document.
    #[must_use]
    pub fn at(self, locator: SourceLocator) -> Self {
        Self { locator: Some(locator), ..self }
    }
}
