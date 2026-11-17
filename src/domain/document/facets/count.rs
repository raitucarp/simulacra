//! Facet counts: how many documents each value would leave, given the other facets.

use super::{document_values, matches, Facet, FacetSelection};
use crate::domain::document::vocabulary::SubjectTree;
use crate::domain::document::GroundingDocument;
use std::collections::BTreeMap;

/// One value of a facet with the number of documents it matches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacetCount {
    pub key: String,
    pub count: usize,
    pub selected: bool,
}

/// Counts of every value of `facet` among `docs`, under `selection` minus `facet` itself
/// (so choosing one value still shows its alternatives). Chosen values stay listed even
/// at zero. Sorted by count, then key.
pub fn facet_counts(docs: &[&GroundingDocument], selection: &FacetSelection, facet: Facet, tree: &SubjectTree) -> Vec<FacetCount> {
    let others = selection.clone().without(facet);
    let mut counts: BTreeMap<String, usize> = docs
        .iter()
        .filter(|doc| matches(doc, &others, tree))
        .flat_map(|doc| document_values(doc, facet, tree))
        .fold(BTreeMap::new(), |mut counts, key| {
            *counts.entry(key).or_default() += 1;
            counts
        });
    for chosen in selection.values(facet) {
        counts.entry(chosen.to_string()).or_default();
    }
    let mut out: Vec<FacetCount> = counts
        .into_iter()
        .map(|(key, count)| FacetCount { selected: selection.contains(facet, &key), key, count })
        .collect();
    out.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.key.cmp(&b.key)));
    out
}
