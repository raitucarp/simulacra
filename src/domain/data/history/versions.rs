//! The version list the history sheet shows: the current version first, then every earlier
//! one, newest first, the oldest marked as the original.

use super::Revision;
use chrono::{DateTime, Utc};

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
    pub revision: Option<super::RevisionId>,
    /// When this version stopped being current (`None` for the current one).
    pub replaced_at: Option<DateTime<Utc>>,
    pub title: String,
    pub content: String,
    pub record_json: String,
}

/// The current text followed by `revisions` (any order), newest first.
pub fn version_list(current_title: &str, current: &str, revisions: &[Revision]) -> Vec<Version> {
    let mut earlier: Vec<&Revision> = revisions.iter().collect();
    earlier.sort_by(|a, b| b.saved_at.cmp(&a.saved_at).then_with(|| b.id.as_str().cmp(a.id.as_str())));
    let last = earlier.len().saturating_sub(1);
    let now = Version { kind: VersionKind::Current, revision: None, replaced_at: None, title: current_title.to_string(), content: current.to_string(), record_json: String::new() };
    let past = earlier.into_iter().enumerate().map(|(at, revision)| Version {
        kind: if at == last { VersionKind::Original } else { VersionKind::Earlier },
        revision: Some(revision.id.clone()),
        replaced_at: Some(revision.saved_at),
        title: revision.title.clone(),
        content: revision.content.clone(),
        record_json: revision.record_json.clone(),
    });
    std::iter::once(now).chain(past).collect()
}
