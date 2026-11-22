//! The Search page's filter chips (Q4): one per filter of the query (clicking removes it),
//! and quick filters that add or remove themselves. Chips read as the query syntax itself,
//! so they teach it.

use crate::domain::search_query::{HasFilter, IsFilter, PostQuery, QueryFilter};

/// Quick filters offered under the search field.
pub const QUICK_FILTERS: [QueryFilter; 8] = [
    QueryFilter::Has(HasFilter::Image),
    QueryFilter::Has(HasFilter::File),
    QueryFilter::Has(HasFilter::Code),
    QueryFilter::Has(HasFilter::Todo),
    QueryFilter::Is(IsFilter::Open),
    QueryFilter::Is(IsFilter::Done),
    QueryFilter::Is(IsFilter::Archived),
    QueryFilter::Is(IsFilter::Locked),
];

/// One filter of the query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterChip {
    pub filter: QueryFilter,
    /// A `-` filter: posts matching it are left out.
    pub negated: bool,
    /// The filter as typed (`has:image`, `-from:sam`).
    pub label: String,
}

/// The chips of every filter of `query`, kept ones first.
pub fn active_chips(query: &PostQuery) -> Vec<FilterChip> {
    let kept = query.filters.iter().map(|filter| (filter, false));
    let dropped = query.excluded_filters.iter().map(|filter| (filter, true));
    kept.chain(dropped)
        .map(|(filter, negated)| {
            let text = filter.to_text();
            FilterChip { filter: filter.clone(), negated, label: if negated { format!("-{text}") } else { text } }
        })
        .collect()
}

/// The selected row after moving one step down (`forward`) or up, wrapping around; `None`
/// without rows.
pub fn stepped(selected: Option<usize>, rows: usize, forward: bool) -> Option<usize> {
    if rows == 0 {
        return None;
    }
    Some(match (selected, forward) {
        (None, true) => 0,
        (None, false) => rows - 1,
        (Some(at), true) => (at + 1) % rows,
        (Some(at), false) => (at + rows - 1) % rows,
    })
}
