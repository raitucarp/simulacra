//! Engine metadata of a generated post: its mood, image prompt, the tools the agent used and
//! the sources it drew on. Persisted as one JSON column; empty for typed human posts.
//! Imported and captured posts record their origin and the passage they cite.

use super::provenance::{PostOrigin, SourceLocator};
use crate::domain::symbols::PostSymbols;
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
    /// Set on imported and captured posts (captures make up the Inbox).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<PostOrigin>,
    /// Images attached in the composer, with alt text (sb C3); `image_path` holds the first.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<crate::domain::media::ImageAttachment>,
    /// Local files attached in the composer (L2), linked where they are.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<crate::domain::media::FileRef>,
    /// Entry kind, state, signifiers and custom icons (shown in place of the avatar).
    #[serde(skip_serializing_if = "PostSymbols::is_empty")]
    pub symbols: PostSymbols,
    /// The colour the post's container is tinted with (sb C1); unknown keys read as none.
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "super::tint::lenient")]
    pub tint: Option<super::tint::Tint>,
    /// The version the post shows (V3, V4); `None` until its first edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<crate::domain::data::history::VersionHead>,
    /// The version of the post it replies to when it was written (V12).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replied_version: Option<crate::domain::data::history::SemVer>,
    /// The boxes the reader ticked (D6): kept out of the text, so ticking makes no version.
    #[serde(skip_serializing_if = "crate::domain::todo::state::TodoState::is_empty")]
    pub todo_state: crate::domain::todo::state::TodoState,
    /// The sealed text of a locked post (X2); `content` then holds only the placeholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<super::lock::PostLock>,
    /// The latest run of each code block (P6); saving one never makes a new version.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub code_runs: Vec<crate::domain::code::CodeRun>,
    /// N2: the span of the parent a margin reply answers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<super::anchor::SpanAnchor>,
    /// N3: the post is a garden note, out of the stream, at this stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub garden: Option<super::garden::GardenNote>,
    /// N6: the reply starts a what-if branch of the thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<super::branch::BranchPoint>,
    /// N7: a vault passage the post quotes verbatim, pinned to its document (boxed: posts stay small).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<Box<super::pinned::PinnedQuote>>,
    /// N8: who besides the author may read it (`None`: everyone; boxed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Box<super::audience::Audience>>,
    /// N9: the post was asked of the room: who was asked, and the accepted answer (boxed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<Box<super::question::Question>>,
}

impl PostExtras {
    /// Whether nothing is recorded (the column then stores `{}`).
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// Whether there is anything to show behind the post (the origin, images, files, symbols, tint,
    /// version, ticks, lock, code runs, margin anchor, garden note, branch pointer, pinned quote, audience and question alone are not).
    pub fn has_details(&self) -> bool {
        Self { origin: None, images: Vec::new(), files: Vec::new(), symbols: PostSymbols::default(), tint: None, head: None, replied_version: None, todo_state: Default::default(), lock: None, code_runs: Vec::new(), anchor: None, garden: None, branch: None, pinned: None, audience: None, question: None, ..self.clone() } != Self::default()
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
