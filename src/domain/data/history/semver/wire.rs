//! A version number and a bump as stored (V3): the number as its `major.minor.patch` text,
//! the bump as its key (`patch`, `minor`, `major`). A malformed number reads as the original.

use super::{Bump, SemVer};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for SemVer {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for SemVer {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Ok(Self::parse(&text).unwrap_or_default())
    }
}

impl Bump {
    /// Every bump, smallest first.
    pub const ALL: [Self; 3] = [Self::Patch, Self::Minor, Self::Major];

    /// Stable key (stored in `revisions.bump`, locale keys `data.bump.<key>`). Never rename one.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Patch => "patch",
            Self::Minor => "minor",
            Self::Major => "major",
        }
    }

    /// The bump stored as `key`, if it is one.
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|bump| bump.key() == key)
    }
}

impl Serialize for Bump {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.key())
    }
}

impl<'de> Deserialize<'de> for Bump {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let key = String::deserialize(deserializer)?;
        Ok(Self::from_key(&key).unwrap_or(Self::Minor))
    }
}
