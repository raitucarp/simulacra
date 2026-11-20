//! Version numbers of a subject's history, stored as text in `revisions.version`.
//!
//! Numbering: the original text of a post (or vault document) is `1.0.0`, and each edit
//! bumps the minor number, so the text after the first edit is `1.1.0`, after the second
//! `1.2.0`, and so on. A revision row holds the version it kept, which makes the oldest
//! revision of a subject edited N times `1.0.0` and its current, unrevisioned text `1.N.0`.
//! `revisions.parent_id` points at the revision holding the version this one was edited from
//! (none for the original), so a history is a linear chain until branches exist.
//! Major and patch numbers are left at `1` and `0` for later use.

use std::fmt;

/// A `major.minor.patch` version number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemVer {
    /// The version of a subject's original text.
    pub const ORIGINAL: Self = Self { major: 1, minor: 0, patch: 0 };

    /// The version after `edits` edits of the original (`0` is the original itself).
    pub const fn after_edits(edits: u32) -> Self {
        Self { minor: edits, ..Self::ORIGINAL }
    }

    /// The version one edit after this one.
    pub const fn next_edit(self) -> Self {
        Self { minor: self.minor + 1, patch: 0, ..self }
    }

    /// The version written as `major.minor.patch`, if `raw` is one.
    pub fn parse(raw: &str) -> Option<Self> {
        let mut parts = raw.trim().split('.').map(|part| part.parse::<u32>().ok());
        let (major, minor, patch) = (parts.next()??, parts.next()??, parts.next()??);
        parts.next().is_none().then_some(Self { major, minor, patch })
    }
}

impl fmt::Display for SemVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
