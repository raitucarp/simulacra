//! Searching the downloaded icons by name (S15): every icon's name split into tokens
//! (`arrow-up-right` → `arrow`, `up`, `right`), matched by token prefix, optionally within one
//! set (`set:<id>` in the query, or the caller's filter). Best first: the exact name, then
//! names starting with the query's first term, then the rest, each in set and name order.

use crate::domain::symbols::SymbolRef;

/// The query term that narrows a search to one set (`set:fi`).
pub const SET_TERM: &str = "set:";
/// Results returned at most.
pub const SEARCH_LIMIT: usize = 96;

/// One indexed icon.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    symbol: SymbolRef,
    tokens: Vec<String>,
}

/// The name index of the installed icons.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SymbolIndex {
    entries: Vec<Entry>,
}

/// Lower-case tokens of an icon name.
pub fn name_tokens(name: &str) -> Vec<String> {
    name.split(|c: char| !c.is_ascii_alphanumeric()).filter(|token| !token.is_empty()).map(str::to_ascii_lowercase).collect()
}

impl SymbolIndex {
    /// The index of `sets`: each set id with its icon names.
    pub fn build<'a>(sets: impl IntoIterator<Item = (&'a str, Vec<String>)>) -> Self {
        let mut entries: Vec<Entry> = sets
            .into_iter()
            .flat_map(|(set, names)| names.into_iter().map(move |name| Entry { tokens: name_tokens(&name), symbol: SymbolRef::new(set, name) }))
            .collect();
        entries.sort_by(|a, b| (&a.symbol.set, &a.symbol.name).cmp(&(&b.symbol.set, &b.symbol.name)));
        Self { entries }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Every icon, in set and name order.
    pub fn symbols(&self) -> impl Iterator<Item = &SymbolRef> {
        self.entries.iter().map(|entry| &entry.symbol)
    }

    /// Icons matching `query` within `set` (when given, or named by a `set:` term), best
    /// first, at most [`SEARCH_LIMIT`]. A query without name terms finds nothing.
    pub fn search(&self, query: &str, set: Option<&str>) -> Vec<SymbolRef> {
        let query = query.to_ascii_lowercase();
        let named_set = query.split_whitespace().find_map(|term| term.strip_prefix(SET_TERM));
        let set = set.map(str::to_ascii_lowercase).or(named_set.map(str::to_string));
        let terms: Vec<&str> = query.split_whitespace().filter(|term| !term.starts_with(SET_TERM)).collect();
        if terms.is_empty() {
            return Vec::new();
        }
        let mut found: Vec<(u8, &Entry)> = self
            .entries
            .iter()
            .filter(|entry| set.as_deref().is_none_or(|set| entry.symbol.set == set))
            .filter(|entry| terms.iter().all(|term| entry.tokens.iter().any(|token| token.starts_with(term))))
            .map(|entry| (rank(entry, &terms), entry))
            .collect();
        found.sort_by_key(|(rank, _)| *rank);
        found.into_iter().take(SEARCH_LIMIT).map(|(_, entry)| entry.symbol.clone()).collect()
    }
}

/// 0: the name is the query; 1: the name starts with the first term; 2: anything else.
fn rank(entry: &Entry, terms: &[&str]) -> u8 {
    if entry.tokens.len() == terms.len() && entry.tokens.iter().zip(terms).all(|(token, term)| token == term) {
        0
    } else if entry.tokens.first().is_some_and(|first| first.starts_with(terms[0])) {
        1
    } else {
        2
    }
}
