//! The version list the history sheet shows: every version, newest edit first (the order a
//! version tree is drawn in, V5), each with its number, the version it was edited from, and
//! the bump and note of that edit. The version shown is marked current, a version edited
//! from none (the oldest, for chains stored before parents) the original.

use super::{Bump, Revision, RevisionId, SemVer, VersionHead};
use jiff::Timestamp;

/// Where a version stands in the history.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionKind {
    Current,
    /// An edit between the original and the current version.
    Earlier,
    /// The first version.
    Original,
}

/// One row of the history.
#[derive(Debug, Clone, PartialEq)]
pub struct Version {
    pub kind: VersionKind,
    /// The revision holding this version (`None` for the current one).
    pub revision: Option<RevisionId>,
    /// When this version stopped being current (`None` for the current one).
    pub replaced_at: Option<Timestamp>,
    pub title: String,
    pub content: String,
    pub record_json: String,
    /// The version's id in the tree (the revision's, or the head's), and its parent's.
    pub id: Option<RevisionId>,
    pub parent: Option<RevisionId>,
    pub number: Option<SemVer>,
    /// The bump and note of the edit that made it, and when it was saved (V3).
    pub bump: Option<Bump>,
    pub note: String,
    pub edited_at: Option<Timestamp>,
}

impl Version {
    /// When this version was written, as far as is known (the current one counts as newest).
    fn written(&self) -> Timestamp {
        self.edited_at.or(self.replaced_at).unwrap_or(Timestamp::MAX)
    }

    fn earlier(revision: &Revision) -> Self {
        let bump = revision.parent_id.as_ref().map(|_| revision.bump);
        Self { kind: VersionKind::Earlier, revision: Some(revision.id.clone()), replaced_at: Some(revision.saved_at), title: revision.title.clone(), content: revision.content.clone(), record_json: revision.record_json.clone(), id: Some(revision.id.clone()), parent: revision.parent_id.clone(), number: revision.version, bump, note: revision.note.clone(), edited_at: revision.edited_at }
    }

    fn current(title: &str, content: &str, head: Option<&VersionHead>) -> Self {
        let (id, parent, number) = (head.map(|h| h.id.clone()), head.and_then(|h| h.parent.clone()), head.map(|h| h.version));
        let (bump, note, edited_at) = (head.and_then(|h| h.bump), head.map(|h| h.note.clone()).unwrap_or_default(), head.and_then(|h| h.edited_at));
        Self { kind: VersionKind::Current, revision: None, replaced_at: None, title: title.to_string(), content: content.to_string(), record_json: String::new(), id, parent, number, bump, note, edited_at }
    }
}

/// The current text (with its `head`, for posts) and `revisions` (any order), newest edit
/// first; ties keep the current version first, then the higher number.
pub fn version_list(current_title: &str, current: &str, head: Option<&VersionHead>, revisions: &[Revision]) -> Vec<Version> {
    let mut list: Vec<Version> = std::iter::once(Version::current(current_title, current, head)).chain(revisions.iter().map(Version::earlier)).collect();
    list.sort_by(|a, b| b.written().cmp(&a.written()).then_with(|| (b.kind == VersionKind::Current).cmp(&(a.kind == VersionKind::Current))).then_with(|| b.number.cmp(&a.number)).then_with(|| b.replaced_at.cmp(&a.replaced_at)));
    for version in list.iter_mut().filter(|v| v.kind == VersionKind::Earlier && v.parent.is_none()) {
        version.kind = VersionKind::Original;
    }
    if head.is_none() {
        let newest = list.iter().find(|v| v.kind != VersionKind::Current).and_then(|v| v.id.clone());
        list.iter_mut().filter(|v| v.kind == VersionKind::Current).for_each(|v| v.parent = newest.clone()); // a chain without a head
    }
    list
}

/// The position of the current version in `versions`.
pub fn current_index(versions: &[Version]) -> usize {
    versions.iter().position(|v| v.kind == VersionKind::Current).unwrap_or(0)
}
