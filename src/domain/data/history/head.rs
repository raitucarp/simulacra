//! The version a post shows (V3, V4): the post keeps its current version's number, the
//! version it was edited from, the bump and note of that edit, and the id the version keeps
//! as a revision once another one is current. A post never edited has no stored head: it
//! is its original, `1.0.0`.

use super::{Bump, Revision, RevisionId, RevisionSubject, SemVer};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// The current version of a post (`extras.head`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VersionHead {
    /// The id this version is stored under once another version is current.
    pub id: RevisionId,
    pub version: SemVer,
    /// The version this one was edited from (`None` for the original).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RevisionId>,
    /// The part of the version the edit bumped (`None` for the original).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bump: Option<Bump>,
    /// The edit note, as the user wrote it.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub note: String,
    /// When the edit was saved (the post's creation for the original).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_at: Option<Timestamp>,
}

impl VersionHead {
    /// The original of a post written at `created_at`.
    pub fn original(created_at: Timestamp) -> Self {
        Self { id: RevisionId::generate(), version: SemVer::ORIGINAL, edited_at: Some(created_at), ..Self::default() }
    }

    /// The head of a post edited before heads were stored: one minor version after its
    /// newest revision, edited from it when that revision was replaced.
    pub fn after(newest: &Revision) -> Self {
        let version = newest.version.unwrap_or_default().next_edit();
        Self { id: RevisionId::generate(), version, parent: Some(newest.id.clone()), bump: Some(Bump::Minor), note: String::new(), edited_at: Some(newest.saved_at) }
    }

    /// The head of a post: the stored one, else the one after its newest revision, else its
    /// original.
    pub fn of(stored: Option<&Self>, revisions: &[Revision], created_at: Timestamp) -> Self {
        let newest = revisions.iter().max_by(|a, b| a.saved_at.cmp(&b.saved_at).then_with(|| a.id.as_str().cmp(b.id.as_str())));
        match (stored, newest) {
            (Some(head), _) => head.clone(),
            (None, Some(newest)) => Self::after(newest),
            (None, None) => Self::original(created_at),
        }
    }

    /// This version kept as a revision of `subject` holding `content`, replaced at `now`.
    pub fn into_revision(self, subject: RevisionSubject, content: &str, now: Timestamp) -> Revision {
        let bump = self.bump.unwrap_or(Bump::Minor);
        Revision { id: self.id, parent_id: self.parent, version: Some(self.version), bump, note: self.note, edited_at: self.edited_at, ..Revision::of(subject, content, now) }
    }

    /// `revision` as the current version again (V9).
    pub fn from_revision(revision: &Revision) -> Self {
        let bump = revision.parent_id.as_ref().map(|_| revision.bump);
        let version = revision.version.unwrap_or_default();
        Self { id: revision.id.clone(), version, parent: revision.parent_id.clone(), bump, note: revision.note.clone(), edited_at: revision.edited_at }
    }

    /// The version an edit of this one by `bump` gets, among the `taken` versions of the post.
    pub fn next(&self, bump: Bump, taken: &[SemVer]) -> SemVer {
        free_version(self.version, bump, taken)
    }
}

/// The version `bump` makes of `base`, bumped again by the same part while another version
/// of the post already has it (an edit of an older version starts a branch, V4).
pub fn free_version(base: SemVer, bump: Bump, taken: &[SemVer]) -> SemVer {
    let mut next = base.bumped(bump);
    while taken.contains(&next) && next != next.bumped(bump) {
        next = next.bumped(bump);
    }
    next
}

/// The bump that turned `from` into `to`: the highest part that changed (a patch when none did).
pub fn step_bump(from: SemVer, to: SemVer) -> Bump {
    if to.major != from.major {
        Bump::Major
    } else if to.minor != from.minor {
        Bump::Minor
    } else {
        Bump::Patch
    }
}

/// The version a reply answered, when the post it replies to has since had a major edit
/// (V12: "replied to v1.x"); `None` while the reply still answers the same major version.
pub fn replied_to_older(replied: Option<SemVer>, parent_now: SemVer) -> Option<SemVer> {
    replied.filter(|replied| parent_now.major > replied.major)
}
