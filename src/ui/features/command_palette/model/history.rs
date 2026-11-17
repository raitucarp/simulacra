//! Recently and frequently opened palette items, persisted as JSON in the settings table.

use serde::{Deserialize, Serialize};

/// Most entries kept; the least recent fall off.
const HISTORY_CAPACITY: usize = 60;

/// One remembered target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub key: String,
    pub uses: u32,
}

/// Opened targets, most recent first.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PaletteHistory {
    entries: Vec<HistoryEntry>,
}

impl PaletteHistory {
    /// The history after opening `key`: moved to the front with one more use.
    pub fn recorded(&self, key: &str) -> Self {
        let uses = self.uses(key) + 1;
        let first = HistoryEntry { key: key.to_string(), uses };
        let rest = self.entries.iter().filter(|entry| entry.key != key).cloned();
        Self { entries: std::iter::once(first).chain(rest).take(HISTORY_CAPACITY).collect() }
    }

    /// How often `key` was opened.
    pub fn uses(&self, key: &str) -> u32 {
        self.entries.iter().find(|entry| entry.key == key).map_or(0, |entry| entry.uses)
    }

    /// Keys, most recent first.
    pub fn recent(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|entry| entry.key.as_str())
    }

    /// Keys opened more than once, most used first (ties: most recent first).
    pub fn frequent(&self) -> Vec<&str> {
        let mut repeated: Vec<(usize, &HistoryEntry)> = self.entries.iter().enumerate().filter(|(_, entry)| entry.uses > 1).collect();
        repeated.sort_by(|(a_at, a), (b_at, b)| b.uses.cmp(&a.uses).then(a_at.cmp(b_at)));
        repeated.into_iter().map(|(_, entry)| entry.key.as_str()).collect()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Parses stored JSON; anything unreadable starts an empty history.
    pub fn from_json(raw: &str) -> Self {
        serde_json::from_str(raw).unwrap_or_default()
    }
}
