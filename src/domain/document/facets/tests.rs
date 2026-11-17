use super::{document_values, facet_counts, matches, Facet, FacetCount, FacetSelection};
use crate::domain::document::fixtures::{documents, library, METAPHYSICS, PHILOSOPHY, SHELF};
use crate::domain::document::GroundingDocument;

fn counts(docs: &[GroundingDocument], selection: &FacetSelection, facet: Facet) -> Vec<(String, usize, bool)> {
    let refs: Vec<&GroundingDocument> = docs.iter().collect();
    facet_counts(&refs, selection, facet, &library().tree())
        .into_iter()
        .map(|FacetCount { key, count, selected }| (key, count, selected))
        .collect()
}

#[test]
fn subjects_count_under_their_broader_terms() {
    let docs = documents();
    let tree = library().tree();
    assert_eq!(document_values(&docs[0], Facet::Subject, &tree), ["subj_ethics", "subj_metaphysics", "subj_philosophy"]);
    let subject_counts = counts(&docs, &FacetSelection::default(), Facet::Subject);
    assert_eq!(subject_counts[0], ("subj_philosophy".into(), 3, false));
    assert!(subject_counts.contains(&("subj_metaphysics".into(), 2, false)));
}

#[test]
fn counts_ignore_their_own_facet_but_respect_the_others() {
    let docs = documents();
    let books = FacetSelection::default().toggled(Facet::Kind, "book");
    assert_eq!(counts(&docs, &books, Facet::Kind), [("book".into(), 2, true), ("note".into(), 1, false)]);
    assert_eq!(counts(&docs, &books, Facet::Period), [("early_modern".into(), 1, false), ("modern".into(), 1, false)]);
    assert_eq!(counts(&docs, &books, Facet::Collection), [(SHELF.into(), 1, false)]);
}

#[test]
fn chosen_values_stay_listed_at_zero() {
    let docs = documents();
    let selection = FacetSelection::default().toggled(Facet::Status, "read");
    assert!(counts(&docs, &selection, Facet::Status).contains(&("read".into(), 0, true)));
}

#[test]
fn values_of_one_facet_are_alternatives_and_facets_narrow_each_other() {
    let docs = documents();
    let tree = library().tree();
    let matching = |selection: &FacetSelection| docs.iter().filter(|d| matches(d, selection, &tree)).map(|d| d.id.as_str()).collect::<Vec<_>>();
    let either = FacetSelection::default().toggled(Facet::Period, "modern").toggled(Facet::Period, "early_modern");
    assert_eq!(matching(&either), ["doc_ethics", "doc_tractatus", "doc_note"]);
    let narrowed = either.clone().toggled(Facet::Subject, METAPHYSICS);
    assert_eq!(matching(&narrowed), ["doc_ethics", "doc_note"]);
    let broad = FacetSelection::default().only(Facet::Subject, PHILOSOPHY);
    assert_eq!(matching(&broad).len(), 3);
    assert_eq!(narrowed.clone().toggled(Facet::Subject, METAPHYSICS), either, "toggling twice removes the value");
    assert_eq!(narrowed.len(), 3);
}
