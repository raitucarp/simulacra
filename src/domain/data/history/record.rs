//! What a post's version keeps beside its text (V10): its symbols and its tint, stored as the
//! revision's record (`revisions.record_json`). The symbols are the record's top level, so a
//! record kept by an entry state move before tints were kept (S7) reads the same. A post
//! revision without a record (a text edit before V10) keeps nothing to restore.

use crate::domain::post::Tint;
use crate::domain::symbols::PostSymbols;
use crate::domain::Post;
use serde::{Deserialize, Serialize};

/// The symbols and tint of one version of a post.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostRecord {
    #[serde(flatten)]
    pub symbols: PostSymbols,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "crate::domain::post::lenient_tint")]
    pub tint: Option<Tint>,
}

impl PostRecord {
    /// What `post` shows beside its text.
    pub fn of(post: &Post) -> Self {
        Self { symbols: post.extras.symbols.clone(), tint: post.extras.tint }
    }

    /// The record as stored.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// The record stored as `json`; `None` when nothing was kept (or it cannot be read).
    pub fn parse(json: &str) -> Option<Self> {
        (!json.trim().is_empty()).then(|| serde_json::from_str(json).ok()).flatten()
    }

    /// `post` showing this record's symbols and tint.
    pub fn apply(&self, post: &mut Post) {
        post.extras.symbols = self.symbols.clone();
        post.extras.tint = self.tint;
    }
}

/// What changed beside the text between two versions' records (V10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RecordChanges {
    /// The tint before and after, when it changed.
    pub tint: Option<(Option<Tint>, Option<Tint>)>,
    pub symbols: bool,
}

/// The changes from record `from` to record `to` (stored JSON); nothing when either version
/// kept no record.
pub fn record_changes(from: &str, to: &str) -> RecordChanges {
    let (Some(a), Some(b)) = (PostRecord::parse(from), PostRecord::parse(to)) else { return RecordChanges::default() };
    RecordChanges { tint: (a.tint != b.tint).then_some((a.tint, b.tint)), symbols: a.symbols != b.symbols }
}
