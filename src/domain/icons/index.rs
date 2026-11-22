//! What the icon store holds (S12): the installed sets, as kept in its `index.json`.

use serde::{Deserialize, Serialize};

/// One installed set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StoredSet {
    pub id: String,
    pub icons: u32,
    /// Bytes its icons take on disk.
    pub bytes: u64,
    /// The package version it came from.
    pub version: String,
    /// When it was installed (RFC 3339).
    pub installed_at: String,
}

/// Every installed set, by id order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StoreIndex {
    pub sets: Vec<StoredSet>,
}

impl StoreIndex {
    /// The installed set `id`.
    pub fn get(&self, id: &str) -> Option<&StoredSet> {
        self.sets.iter().find(|set| set.id == id)
    }

    /// Adds or replaces `set`, keeping the list sorted by id.
    pub fn put(&mut self, set: StoredSet) {
        self.sets.retain(|old| old.id != set.id);
        self.sets.push(set);
        self.sets.sort_by(|a, b| a.id.cmp(&b.id));
    }

    /// Removes the set `id`; whether it was listed.
    pub fn take(&mut self, id: &str) -> bool {
        let before = self.sets.len();
        self.sets.retain(|set| set.id != id);
        self.sets.len() != before
    }
}
