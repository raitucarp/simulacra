//! The identifier shared by every private record of the Journal page (entries, check-ins,
//! focus sessions, time capsules): an opaque string, fresh ones are random.

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// The id of a journal entry, a mood check-in, a focus session or a time capsule.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RecordId(String);

impl RecordId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// A fresh random id.
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for RecordId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
