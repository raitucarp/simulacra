//! A catalogue search: free text, facet filters and a sort order, plus the in-memory
//! evaluation used when no database is attached (the database path builds the same query
//! in SQL, see `storage::db::catalogue_query`).

mod rank;
#[cfg(test)]
mod tests;

pub use rank::{filing_title, run, text_score, RankingSignals};

use super::facets::FacetSelection;
use super::keyed::keyed_enum;
use crate::domain::NetworkId;

keyed_enum! {
    /// Order of the results.
    SortKey {
        Relevance => "relevance",
        Title => "title",
        DateAdded => "date_added",
        DatePublished => "date_published",
        MostLinked => "most_linked",
    }
}

impl SortKey {
    /// The direction a fresh sort on this key starts in (A–Z for titles, newest or most first
    /// otherwise).
    pub const fn starts_descending(self) -> bool {
        !matches!(self, Self::Title)
    }
}

/// Characters that end a search term.
fn is_term_char(c: char) -> bool {
    c.is_alphanumeric()
}

/// Shortest search term kept (single letters match almost everything).
pub const MIN_TERM_CHARS: usize = 2;

/// What to look for and how to order it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogueQuery {
    pub network_id: NetworkId,
    pub text: String,
    pub selection: FacetSelection,
    pub sort: SortKey,
    pub descending: bool,
}

impl CatalogueQuery {
    /// Everything in `network`, by relevance (the newest first while there is no text).
    pub fn new(network_id: NetworkId) -> Self {
        Self { network_id, text: String::new(), selection: FacetSelection::default(), sort: SortKey::Relevance, descending: true }
    }

    /// The sort actually applied: relevance needs search text, else the newest come first.
    pub fn effective_sort(&self) -> (SortKey, bool) {
        if self.sort == SortKey::Relevance && self.terms().is_empty() {
            (SortKey::DateAdded, true)
        } else {
            (self.sort, self.descending)
        }
    }

    /// This query sorted by `key` in the given direction.
    pub fn sorted(mut self, key: SortKey, descending: bool) -> Self {
        (self.sort, self.descending) = (key, descending);
        self
    }

    /// This query sorted by `key`; choosing the current key again reverses the direction.
    pub fn sorted_by(mut self, key: SortKey) -> Self {
        if self.sort == key {
            self.descending = !self.descending;
        } else {
            self.sort = key;
            self.descending = key.starts_descending();
        }
        self
    }

    /// The lower-cased search terms (letters and digits, at least [`MIN_TERM_CHARS`] long).
    pub fn terms(&self) -> Vec<String> {
        self.text
            .split(|c: char| !is_term_char(c))
            .filter(|term| term.chars().count() >= MIN_TERM_CHARS)
            .map(str::to_lowercase)
            .collect()
    }

    /// Whether the query narrows anything (text or facets).
    pub fn is_filtering(&self) -> bool {
        !self.terms().is_empty() || !self.selection.is_empty()
    }
}
