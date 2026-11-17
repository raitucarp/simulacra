//! In-memory evaluation of a [`CatalogueQuery`]: text scoring, filtering and sorting.

use super::{CatalogueQuery, SortKey};
use crate::domain::document::facets::matches;
use crate::domain::document::vocabulary::SubjectTree;
use crate::domain::document::GroundingDocument;
use std::cmp::Ordering;
use std::collections::HashMap;

/// Leading articles skipped when filing titles ("The Ethics" files under E).
const FILING_ARTICLES: [&str; 3] = ["the ", "an ", "a "];
/// Weights of where a term was found.
const TITLE_WEIGHT: f64 = 4.0;
const CREATOR_WEIGHT: f64 = 3.0;
const ABSTRACT_WEIGHT: f64 = 2.0;
const TEXT_WEIGHT: f64 = 1.0;

/// Per-document numbers that order results.
#[derive(Debug, Clone, Copy, Default)]
pub struct RankingSignals<'a> {
    /// Posts linking to each document id.
    pub link_counts: Option<&'a HashMap<String, usize>>,
}

/// The title as filed: lower-cased, without a leading article.
pub fn filing_title(title: &str) -> String {
    let lower = title.trim().to_lowercase();
    FILING_ARTICLES
        .iter()
        .find_map(|article| lower.strip_prefix(article))
        .map_or(lower.clone(), |rest| rest.trim_start().to_string())
}

/// Weighted score of `doc` for `terms`: zero unless every term occurs somewhere.
pub fn text_score(doc: &GroundingDocument, terms: &[String]) -> f64 {
    let record = &doc.record;
    let creators: String = record.creators.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(" ");
    let fields = [
        (format!("{} {}", doc.title, record.subtitle).to_lowercase(), TITLE_WEIGHT),
        (creators.to_lowercase(), CREATOR_WEIGHT),
        (format!("{} {}", record.description, record.publisher).to_lowercase(), ABSTRACT_WEIGHT),
        (doc.content.to_lowercase(), TEXT_WEIGHT),
    ];
    terms
        .iter()
        .map(|term| fields.iter().filter(|(text, _)| text.contains(term.as_str())).map(|(_, weight)| weight).sum::<f64>())
        .try_fold(0.0, |total, score| (score > 0.0).then_some(total + score))
        .unwrap_or_default()
}

fn compare(a: &GroundingDocument, b: &GroundingDocument, key: SortKey, scores: &HashMap<&str, f64>, signals: RankingSignals<'_>) -> Ordering {
    let links = |doc: &GroundingDocument| signals.link_counts.and_then(|counts| counts.get(&doc.id)).copied().unwrap_or_default();
    match key {
        SortKey::Relevance => scores.get(a.id.as_str()).partial_cmp(&scores.get(b.id.as_str())).unwrap_or(Ordering::Equal),
        SortKey::Title => filing_title(&a.title).cmp(&filing_title(&b.title)),
        SortKey::DateAdded => a.created_at.cmp(&b.created_at),
        SortKey::DatePublished => a.record.published_year().cmp(&b.record.published_year()),
        SortKey::MostLinked => links(a).cmp(&links(b)),
    }
}

/// The documents of the query's network that match it, in its order (ties by title).
pub fn run<'a>(docs: &'a [GroundingDocument], query: &CatalogueQuery, tree: &SubjectTree, signals: RankingSignals<'_>) -> Vec<&'a GroundingDocument> {
    let terms = query.terms();
    let scored: Vec<(&GroundingDocument, f64)> = docs
        .iter()
        .filter(|doc| doc.network_id == query.network_id && matches(doc, &query.selection, tree))
        .map(|doc| (doc, if terms.is_empty() { 0.0 } else { text_score(doc, &terms) }))
        .filter(|(_, score)| terms.is_empty() || *score > 0.0)
        .collect();
    let scores: HashMap<&str, f64> = scored.iter().map(|(doc, score)| (doc.id.as_str(), *score)).collect();
    let (key, descending) = query.effective_sort();
    let mut found: Vec<&GroundingDocument> = scored.into_iter().map(|(doc, _)| doc).collect();
    found.sort_by(|a, b| {
        let order = compare(a, b, key, &scores, signals);
        let order = if descending { order.reverse() } else { order };
        order.then_with(|| filing_title(&a.title).cmp(&filing_title(&b.title)))
    });
    found
}
