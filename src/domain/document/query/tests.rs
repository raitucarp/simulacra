use super::{filing_title, run, CatalogueQuery, RankingSignals, SortKey};
use crate::domain::document::facets::Facet;
use crate::domain::document::fixtures::{documents, library, NET};
use crate::domain::NetworkId;
use std::collections::HashMap;

fn ids(query: &CatalogueQuery, links: Option<&HashMap<String, usize>>) -> Vec<String> {
    let docs = documents();
    run(&docs, query, &library().tree(), RankingSignals { link_counts: links }).iter().map(|d| d.id.clone()).collect()
}

fn query() -> CatalogueQuery {
    CatalogueQuery::new(NetworkId::new(NET))
}

#[test]
fn without_text_relevance_falls_back_to_newest_first() {
    assert_eq!(query().effective_sort(), (SortKey::DateAdded, true));
    assert_eq!(ids(&query(), None), ["doc_ethics", "doc_note", "doc_tractatus"]);
}

#[test]
fn titles_file_without_leading_articles_and_reverse_on_a_second_click() {
    assert_eq!(filing_title("A Note on Modes"), "note on modes");
    assert_eq!(filing_title("The  Ethics"), "ethics");
    let by_title = query().sorted_by(SortKey::Title);
    assert!(!by_title.descending);
    assert_eq!(ids(&by_title, None), ["doc_ethics", "doc_note", "doc_tractatus"]);
    let reversed = by_title.sorted_by(SortKey::Title);
    assert_eq!(ids(&reversed, None), ["doc_tractatus", "doc_note", "doc_ethics"]);
}

#[test]
fn published_and_linked_orders() {
    let published = query().sorted_by(SortKey::DatePublished);
    assert_eq!(ids(&published, None), ["doc_tractatus", "doc_ethics", "doc_note"]);
    let links: HashMap<String, usize> = [("doc_note".to_string(), 5), ("doc_tractatus".to_string(), 1)].into();
    assert_eq!(ids(&query().sorted_by(SortKey::MostLinked), Some(&links)), ["doc_note", "doc_tractatus", "doc_ethics"]);
}

#[test]
fn text_needs_every_term_and_ranks_title_hits_first() {
    let mut text = query();
    text.text = "wittgenstein".into();
    assert_eq!(ids(&text, None), ["doc_tractatus"]);
    text.text = "modes substance".into();
    assert_eq!(ids(&text, None), ["doc_note"]);
    text.text = "substance".into();
    assert_eq!(ids(&text, None).len(), 3);
    text.text = "a ! ?".into();
    assert!(text.terms().is_empty() && !text.is_filtering());
}

#[test]
fn facets_and_network_scope_filter_results() {
    let mut notes = query();
    notes.selection = notes.selection.toggled(Facet::Kind, "note");
    assert_eq!(ids(&notes, None), ["doc_note"]);
    assert!(ids(&CatalogueQuery::new(NetworkId::new("elsewhere")), None).is_empty());
}
