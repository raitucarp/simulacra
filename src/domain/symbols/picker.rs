//! What the composer's symbol picker offers (S5): the core set (the three entries and the
//! three signifiers), the symbols used most recently, and a search over the core set's names
//! and the downloaded icon library.

use super::kinds::{EntrySymbol, Signifier, SymbolRef};
use super::PostSymbols;
use serde::{Deserialize, Serialize};

/// Recently used symbols kept, newest first.
pub const RECENT_SYMBOLS: usize = 12;
/// Search results shown at most.
pub const SYMBOL_RESULTS: usize = 48;

/// One thing the picker can toggle on a post.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolChoice {
    Entry(EntrySymbol),
    Signifier(Signifier),
    Custom(SymbolRef),
}

/// The core set in picker order: the entries, then the signifiers.
pub fn core_choices() -> Vec<SymbolChoice> {
    let entries = EntrySymbol::ALL.into_iter().map(SymbolChoice::Entry);
    entries.chain(Signifier::ALL.into_iter().map(SymbolChoice::Signifier)).collect()
}

impl SymbolChoice {
    /// Whether `symbols` carries this choice.
    pub fn is_chosen(&self, symbols: &PostSymbols) -> bool {
        match self {
            Self::Entry(entry) => symbols.entry == Some(*entry),
            Self::Signifier(signifier) => symbols.signifiers.contains(signifier),
            Self::Custom(symbol) => symbols.custom.contains(symbol),
        }
    }

    /// Adds the choice to `symbols`, or takes it off when already there. Picking an entry
    /// replaces the previous one and starts it in its first state.
    pub fn toggle(&self, symbols: &mut PostSymbols) {
        let chosen = self.is_chosen(symbols);
        match self {
            Self::Entry(_) if chosen => (symbols.entry, symbols.state) = (None, None),
            Self::Entry(entry) => {
                let fresh = PostSymbols::entry(*entry);
                (symbols.entry, symbols.state) = (fresh.entry, fresh.state);
            }
            Self::Signifier(signifier) if chosen => symbols.remove_signifier(*signifier),
            Self::Signifier(signifier) => symbols.add_signifier(*signifier),
            Self::Custom(symbol) if chosen => symbols.custom.retain(|s| s != symbol),
            Self::Custom(symbol) => symbols.add_custom(symbol.clone()),
        }
    }

    /// Every choice `symbols` carries, in picker order.
    pub fn of(symbols: &PostSymbols) -> Vec<Self> {
        let entry = symbols.entry.map(Self::Entry);
        let signifiers = symbols.signifiers.iter().copied().map(Self::Signifier);
        entry.into_iter().chain(signifiers).chain(symbols.custom.iter().cloned().map(Self::Custom)).collect()
    }
}

/// The picker's memory: the symbols used most recently, newest first.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SymbolMemory {
    pub recent: Vec<SymbolChoice>,
}

impl SymbolMemory {
    /// Remembers `choice` as the most recent, keeping at most [`RECENT_SYMBOLS`].
    pub fn used(&mut self, choice: SymbolChoice) {
        self.recent.retain(|recent| *recent != choice);
        self.recent.insert(0, choice);
        self.recent.truncate(RECENT_SYMBOLS);
    }

    /// The memory as read back: repeats dropped, at most [`RECENT_SYMBOLS`].
    pub fn validated(mut self) -> Self {
        let mut seen = std::collections::HashSet::new();
        self.recent.retain(|choice| seen.insert(choice.clone()));
        self.recent.truncate(RECENT_SYMBOLS);
        self
    }
}

/// Choices matching `query`: core symbols whose name (`label`) has a word starting with
/// every term, then library icons whose name does, at most [`SYMBOL_RESULTS`]. An empty
/// query finds nothing.
pub fn search_symbols(query: &str, label: impl Fn(&SymbolChoice) -> String, library: &[SymbolRef]) -> Vec<SymbolChoice> {
    let query = query.trim().to_lowercase();
    let terms: Vec<&str> = query.split_whitespace().collect();
    if terms.is_empty() {
        return Vec::new();
    }
    let core = core_choices().into_iter().filter(|choice| words_match(&label(choice), &terms));
    let icons = library.iter().filter(|symbol| words_match(&symbol.name, &terms)).cloned().map(SymbolChoice::Custom);
    core.chain(icons).take(SYMBOL_RESULTS).collect()
}

/// Whether every term starts a word of `text` (words split at anything not alphanumeric).
pub fn words_match(text: &str, terms: &[&str]) -> bool {
    let text = text.to_lowercase();
    let words: Vec<&str> = text.split(|c: char| !c.is_alphanumeric()).filter(|word| !word.is_empty()).collect();
    terms.iter().all(|term| words.iter().any(|word| word.starts_with(term)))
}
