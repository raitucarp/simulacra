//! Saved searches (Q5): queries the user keeps, stored as one JSON row of the settings table.
//! A saved search is its query text (re-parsed when opened, so a newer filter reads as the
//! language reads it then) and a name, which starts as the query itself.

use serde::{Deserialize, Serialize};

/// Most searches kept; saving one more drops the oldest.
pub const MAX_SAVED_SEARCHES: usize = 24;
/// Longest name kept, in characters.
pub const MAX_SAVED_NAME_CHARS: usize = 60;

/// One saved query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SavedSearch {
    pub name: String,
    /// The query as typed.
    pub query: String,
}

/// Every saved search, newest first.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SavedSearches {
    pub items: Vec<SavedSearch>,
}

/// `query` with its spaces folded, the form searches are saved and compared in.
pub fn normalized(query: &str) -> String {
    query.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn clipped(name: &str) -> String {
    normalized(name).chars().take(MAX_SAVED_NAME_CHARS).collect()
}

impl SavedSearches {
    /// Reads the stored JSON; anything unreadable reads as none.
    pub fn from_json(raw: &str) -> Self {
        serde_json::from_str(raw).unwrap_or_default()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Whether `query` is saved.
    pub fn contains(&self, query: &str) -> bool {
        let query = normalized(query);
        self.items.iter().any(|item| item.query == query)
    }

    /// Saves `query` (blank queries are not saved), named after itself, as the newest; saving
    /// it again only moves it to the front. Returns whether anything changed.
    pub fn save(&mut self, query: &str) -> bool {
        let query = normalized(query);
        if query.is_empty() {
            return false;
        }
        let kept = match self.items.iter().position(|item| item.query == query) {
            Some(0) => return false,
            Some(at) => self.items.remove(at),
            None => SavedSearch { name: clipped(&query), query },
        };
        self.items.insert(0, kept);
        self.items.truncate(MAX_SAVED_SEARCHES);
        true
    }

    /// Forgets the saved `query`; returns whether it was saved.
    pub fn remove(&mut self, query: &str) -> bool {
        let query = normalized(query);
        let before = self.items.len();
        self.items.retain(|item| item.query != query);
        self.items.len() != before
    }

    /// Renames the saved `query` (a blank name falls back to the query); returns whether it
    /// was saved.
    pub fn rename(&mut self, query: &str, name: &str) -> bool {
        let query = normalized(query);
        let Some(item) = self.items.iter_mut().find(|item| item.query == query) else { return false };
        let name = clipped(name);
        item.name = if name.is_empty() { clipped(&query) } else { name };
        true
    }
}
