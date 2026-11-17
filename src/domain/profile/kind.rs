//! What kind of identity a user-controlled profile is: a regular persona, a pseudonym made on
//! the fly for imports, or the one stable anonymous account.

use super::Profile;
use crate::domain::ids::ProfileId;
use serde::{Deserialize, Serialize};

/// Stable id of the anonymous system profile.
pub const ANONYMOUS_PROFILE_ID: &str = "user_anonymous";
/// Display name and handle of the anonymous profile (a fixed identity, not localized).
const ANONYMOUS_NAME: &str = "Anonymous";
const ANONYMOUS_HANDLE: &str = "@anonymous";
const ANONYMOUS_INITIAL: &str = "?";

/// The identity a profile stands for. Persisted in the `profiles.kind` column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProfileKind {
    /// A regular profile (the primary user, an alter ego or an agent).
    #[default]
    Standard,
    /// A pen name created from a name the user typed.
    Pseudonym,
    /// The single anonymous account.
    Anonymous,
}

impl ProfileKind {
    /// Wire key (`profiles.kind`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Pseudonym => "pseudonym",
            Self::Anonymous => "anonymous",
        }
    }

    /// Whether this is a regular profile (serde skips the field then).
    pub fn is_standard(&self) -> bool {
        *self == Self::Standard
    }

    /// The kind stored under `key` (`Standard` for anything unknown).
    pub fn from_key(key: &str) -> Self {
        [Self::Pseudonym, Self::Anonymous].into_iter().find(|kind| kind.key() == key).unwrap_or_default()
    }
}

impl Profile {
    /// The stable anonymous account.
    pub fn new_anonymous() -> Self {
        let mut profile = Self::new_alt(ProfileId::new(ANONYMOUS_PROFILE_ID), ANONYMOUS_NAME, ANONYMOUS_HANDLE);
        profile.avatar_initial = ANONYMOUS_INITIAL.to_string();
        profile.kind = ProfileKind::Anonymous;
        profile
    }

    /// A pseudonym called `name` with the given id and handle.
    pub fn new_pseudonym(id: impl Into<ProfileId>, name: &str, handle: &str) -> Self {
        let mut profile = Self::new_alt(id, name.trim(), handle);
        profile.kind = ProfileKind::Pseudonym;
        profile
    }

    /// Whether this is a pseudonym or the anonymous account.
    pub fn is_masked(&self) -> bool {
        self.kind != ProfileKind::Standard
    }
}
