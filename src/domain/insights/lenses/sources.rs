//! Where concepts come from, each with its day: the analysed posts, the vault documents
//! (title, description and subjects) and review items (what the learner wrote, the quoted
//! passage or the map title, and the tags), under the same network and date range as the
//! posts. Concepts are read by the shared extractors (`domain::learning::concept`).

use super::{Analysed, LensInput};
use crate::domain::graph::excerpt;
use crate::domain::insights::filter::InsightsFilter;
use crate::domain::insights::report::EXCERPT_CHARS;
use crate::domain::learning::concept::{concept_key, concepts_of, dedup};
use crate::domain::review::{Anchor, ReviewItem, Subject};
use crate::domain::NetworkId;
use jiff::Timestamp;
use jiff::civil::Date;

/// Joins a document's title and description into one text to read.
const TEXT_JOINT: &str = ". ";

/// What a review item says, for the concept timeline.
#[derive(Debug, Clone, PartialEq)]
pub struct ReviewSource {
    pub network: NetworkId,
    pub text: String,
    pub tags: Vec<String>,
    pub created_at: Timestamp,
}

/// A review item's own words: the learner's text and a quoted passage, or a map's title.
/// A post anchor is not repeated: the post itself is a source already.
pub fn review_source(item: &ReviewItem) -> ReviewSource {
    let text = match &item.subject {
        Subject::Anchored { anchor: Anchor::Passage { text, .. }, written, .. } => format!("{written}{TEXT_JOINT}{text}"),
        Subject::Anchored { written, .. } => written.clone(),
        Subject::Connections { map } => map.title.clone(),
    };
    ReviewSource { network: item.network_id.clone(), text, tags: item.tags.clone(), created_at: item.created_at }
}

/// What a source is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceKind {
    Post,
    Document,
    Review,
}

/// One dated text and its concept keys.
#[derive(Debug, Clone, PartialEq)]
pub struct Source {
    pub kind: SourceKind,
    /// Post or document id (empty for review items, which open nothing).
    pub id: String,
    pub day: Date,
    pub excerpt: String,
    pub concepts: Vec<String>,
}

/// Every source under `filter`, oldest first.
pub fn all(input: &LensInput, analysed: &Analysed, filter: &InsightsFilter) -> Vec<Source> {
    let since = filter.range.days().map(|days| input.now - crate::time::days(days));
    let admits = |network: &NetworkId, at: Timestamp| (filter.all_networks || network == &input.network) && since.is_none_or(|since| at >= since);
    let posts = analysed.posts.iter().map(|post| Source { kind: SourceKind::Post, id: post.id.as_str().to_string(), day: analysed.day(post.created_at), excerpt: excerpt(&post.content, EXCERPT_CHARS), concepts: concepts_of(&post.content) });
    let documents = input.documents.iter().filter(|doc| admits(&doc.network_id, doc.created_at)).map(|doc| {
        let labels = doc.record.subjects.iter().filter_map(|id| input.subjects.iter().find(|(s, _)| s == id).map(|(_, label)| concept_key(label)));
        let concepts = dedup(concepts_of(&format!("{}{TEXT_JOINT}{}", doc.title, doc.record.description)).into_iter().chain(labels));
        Source { kind: SourceKind::Document, id: doc.id.clone(), day: analysed.day(doc.created_at), excerpt: excerpt(&doc.title, EXCERPT_CHARS), concepts }
    });
    let reviews = input.review.iter().filter(|item| admits(&item.network, item.created_at)).map(|item| {
        let concepts = dedup(concepts_of(&item.text).into_iter().chain(item.tags.iter().map(|tag| concept_key(tag))).filter(|key| !key.is_empty()));
        Source { kind: SourceKind::Review, id: String::new(), day: analysed.day(item.created_at), excerpt: excerpt(&item.text, EXCERPT_CHARS), concepts }
    });
    let mut sources: Vec<Source> = posts.chain(documents).chain(reviews).filter(|s| !s.concepts.is_empty()).collect();
    sources.sort_by_key(|s| s.day);
    sources
}
