//! Faceted browsing: the facets a document is described by, a selection of facet values
//! (values of one facet are alternatives, different facets narrow each other), matching,
//! and counts.
//!
//! Subjects are hierarchical: a document filed under "Substance" also counts under its
//! broader terms ("Metaphysics", "Philosophy"), so choosing a broad term finds the narrow ones.

mod count;
#[cfg(test)]
mod tests;

pub use count::{facet_counts, FacetCount};

use super::keyed::keyed_enum;
use super::vocabulary::SubjectTree;
use super::GroundingDocument;
use std::collections::BTreeSet;

keyed_enum! {
    /// A dimension documents can be filtered by.
    Facet {
        Kind => "kind",
        Subject => "subject",
        Collection => "collection",
        Discipline => "discipline",
        Form => "form",
        Period => "period",
        Place => "place",
        Language => "language",
        Status => "status",
    }
}

/// The chosen facet values.
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct FacetSelection(BTreeSet<(Facet, String)>);

impl FacetSelection {
    /// This selection with `key` of `facet` switched on or off.
    pub fn toggled(mut self, facet: Facet, key: &str) -> Self {
        let value = (facet, key.to_string());
        if !self.0.remove(&value) {
            self.0.insert(value);
        }
        self
    }

    /// This selection with only `key` chosen for `facet`.
    pub fn only(self, facet: Facet, key: &str) -> Self {
        self.without(facet).toggled(facet, key)
    }

    pub fn contains(&self, facet: Facet, key: &str) -> bool {
        self.0.contains(&(facet, key.to_string()))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Number of chosen values.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Chosen values of `facet`.
    pub fn values(&self, facet: Facet) -> Vec<&str> {
        self.0.iter().filter(|(f, _)| *f == facet).map(|(_, key)| key.as_str()).collect()
    }

    /// Every chosen (facet, value) pair.
    pub fn pairs(&self) -> impl Iterator<Item = (Facet, &str)> {
        self.0.iter().map(|(facet, key)| (*facet, key.as_str()))
    }

    /// This selection without any value of `facet`.
    pub fn without(mut self, facet: Facet) -> Self {
        self.0.retain(|(f, _)| *f != facet);
        self
    }

    /// The facets that have a chosen value.
    pub fn facets(&self) -> BTreeSet<Facet> {
        self.0.iter().map(|(facet, _)| *facet).collect()
    }
}

/// The value keys `doc` has for `facet` (subjects include their broader terms).
pub fn document_values(doc: &GroundingDocument, facet: Facet, tree: &SubjectTree) -> Vec<String> {
    let record = &doc.record;
    let class = &record.classification;
    let one = |key: Option<&str>| key.filter(|k| !k.trim().is_empty()).map(|k| vec![k.trim().to_string()]).unwrap_or_default();
    match facet {
        Facet::Kind => one(Some(record.kind.key())),
        Facet::Discipline => one(class.discipline.map(|d| d.key())),
        Facet::Form => one(class.form.map(|f| f.key())),
        Facet::Period => one(class.period.map(|p| p.key())),
        Facet::Place => one(Some(class.place.as_str())),
        Facet::Language => one(Some(record.language.as_str())).into_iter().map(|l| l.to_lowercase()).collect(),
        Facet::Status => one(Some(record.reading.status.key())),
        Facet::Collection => record.collections.iter().map(|c| c.as_str().to_string()).collect(),
        Facet::Subject => {
            let expanded: BTreeSet<String> = record
                .subjects
                .iter()
                .flat_map(|id| {
                    let path: Vec<String> = tree.path(id).iter().map(|s| s.id.as_str().to_string()).collect();
                    if path.is_empty() { vec![id.as_str().to_string()] } else { path }
                })
                .collect();
            expanded.into_iter().collect()
        }
    }
}

/// Whether `doc` has one of the chosen values of every facet in `selection`.
pub fn matches(doc: &GroundingDocument, selection: &FacetSelection, tree: &SubjectTree) -> bool {
    selection.facets().into_iter().all(|facet| {
        let values = document_values(doc, facet, tree);
        selection.values(facet).iter().any(|chosen| values.iter().any(|value| value == chosen))
    })
}
