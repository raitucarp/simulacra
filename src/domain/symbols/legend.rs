//! The user's symbol key (S9): what each custom symbol means to them ("flask: an
//! experiment"), shown on the legend page beside the core symbols' fixed meanings and in a
//! custom symbol's tooltip. Saved in the vault's settings as JSON.

use super::kinds::SymbolRef;
use serde::{Deserialize, Serialize};

/// Longest meaning kept, in characters.
pub const MAX_MEANING_CHARS: usize = 80;
/// Most symbols the key holds.
pub const MAX_KEY_ENTRIES: usize = 200;

/// One custom symbol and what it means.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct KeyEntry {
    pub symbol: SymbolRef,
    pub meaning: String,
}

/// Every custom symbol the user gave a meaning, in the order they were added.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SymbolKey {
    pub entries: Vec<KeyEntry>,
}

/// `meaning` trimmed, its whitespace collapsed and cut at [`MAX_MEANING_CHARS`].
pub fn clean_meaning(meaning: &str) -> String {
    meaning.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(MAX_MEANING_CHARS).collect()
}

impl SymbolKey {
    /// What `symbol` means, if the key says.
    pub fn meaning_of(&self, symbol: &SymbolRef) -> Option<&str> {
        self.entries.iter().find(|entry| entry.symbol == *symbol).map(|entry| entry.meaning.as_str())
    }

    /// Gives `symbol` the meaning `meaning` (an empty meaning takes it off the key). Returns
    /// whether the key changed; a full key takes no new symbols.
    pub fn set(&mut self, symbol: SymbolRef, meaning: &str) -> bool {
        let meaning = clean_meaning(meaning);
        let at = self.entries.iter().position(|entry| entry.symbol == symbol);
        match (at, meaning.is_empty()) {
            (Some(at), true) => {
                self.entries.remove(at);
                true
            }
            (Some(at), false) if self.entries[at].meaning != meaning => {
                self.entries[at].meaning = meaning;
                true
            }
            (None, false) if self.entries.len() < MAX_KEY_ENTRIES => {
                self.entries.push(KeyEntry { symbol, meaning });
                true
            }
            _ => false,
        }
    }

    /// The key as read back: meanings cleaned, empty and repeated entries dropped, at most
    /// [`MAX_KEY_ENTRIES`].
    pub fn validated(self) -> Self {
        let mut key = Self::default();
        for entry in self.entries {
            if key.meaning_of(&entry.symbol).is_none() {
                key.set(entry.symbol, &entry.meaning);
            }
        }
        key
    }
}
