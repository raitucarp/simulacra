//! Version history (sb F2): every edit of one of the user's posts, a vault document's text or
//! its catalogue record keeps the previous version as a [`Revision`]. Nothing is overwritten:
//! restoring an old version is itself an edit, so the version it replaces is kept too.

mod diff;
mod edit;
mod head; // V3, V4: the version a post shows
mod semver;
mod versions;

pub use edit::{edited_post, EditBase};
pub use head::{free_version, step_bump, VersionHead};
pub use semver::{Bump, ParseSemVerError, SemVer, MAJOR_REMOVED_PERCENT, PATCH_MAX_WORDS};
pub use diff::{diff_stats, tokens, word_diff, DiffOp, DiffStats, MAX_DIFF_CELLS};
pub use versions::{version_list, Version, VersionKind};

use crate::domain::PostId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Prefix of generated revision ids.
pub const REVISION_ID_PREFIX: &str = "rev_";
/// `subject_kind` of a post's revisions.
pub const POST_SUBJECT: &str = "post";
/// `subject_kind` of a vault document's revisions (text, title and record).
pub const DOCUMENT_SUBJECT: &str = "document";

/// A revision's identifier (the `revisions.id` column).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RevisionId(String);

impl RevisionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// A fresh random id.
    pub fn generate() -> Self {
        Self(format!("{REVISION_ID_PREFIX}{}", uuid::Uuid::new_v4().simple()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// What a revision is a version of.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RevisionSubject {
    Post(PostId),
    /// A vault document (by `documents.id`).
    Document(String),
}

impl RevisionSubject {
    /// Stable key stored in `revisions.subject_kind`. Never rename one.
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Post(_) => POST_SUBJECT,
            Self::Document(_) => DOCUMENT_SUBJECT,
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Post(id) => id.as_str(),
            Self::Document(id) => id,
        }
    }

    /// The subject stored as `(kind, id)`, if the kind is known.
    pub fn from_parts(kind: &str, id: &str) -> Option<Self> {
        match kind {
            POST_SUBJECT => Some(Self::Post(PostId::new(id))),
            DOCUMENT_SUBJECT => Some(Self::Document(id.to_string())),
            _ => None,
        }
    }
}

/// One earlier version: the content as it was until `saved_at`, when it was replaced.
#[derive(Debug, Clone, PartialEq)]
pub struct Revision {
    pub id: RevisionId,
    pub subject: RevisionSubject,
    pub saved_at: Timestamp,
    /// The document title (empty for posts).
    pub title: String,
    pub content: String,
    /// The document's catalogue record as JSON (empty for posts).
    pub record_json: String,
    /// The revision holding the version this one was edited from (`None` for the original).
    pub parent_id: Option<RevisionId>,
    /// The version this revision holds; `None` until stored, when the vault numbers it as
    /// the next one of the subject's chain (see [`SemVer`]).
    pub version: Option<SemVer>,
    /// The part of the version the edit making this version bumped (V3; unused for the
    /// original). A revision stored without a version is numbered by it.
    pub bump: Bump,
    /// What the edit making this version was about, as the user wrote it (V3; may be empty).
    pub note: String,
    /// When the edit making this version was saved (V3; `None` when unknown).
    pub edited_at: Option<Timestamp>,
}

impl Revision {
    /// The version of `subject` replaced at `saved_at`.
    pub fn of(subject: RevisionSubject, content: &str, saved_at: Timestamp) -> Self {
        Self { id: RevisionId::generate(), subject, saved_at, title: String::new(), content: content.to_string(), record_json: String::new(), parent_id: None, version: None, bump: Bump::Minor, note: String::new(), edited_at: None }
    }

    /// The same revision, bumping `bump` when it is numbered.
    pub fn with_bump(self, bump: Bump) -> Self {
        Self { bump, ..self }
    }

    /// The same revision with a document's title and record.
    pub fn with_record(self, title: &str, record_json: String) -> Self {
        Self { title: title.to_string(), record_json, ..self }
    }
}

/// Whether an edit changes anything worth a revision (surrounding whitespace is ignored).
pub fn is_real_edit(before: &str, after: &str) -> bool {
    before.trim() != after.trim()
}

#[cfg(test)]
mod tests;
