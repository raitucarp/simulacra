//! Set families over the analysed posts (by concept, author, kind and vault subject) and the
//! embedding map of those posts with a concept label per cluster.

use super::{InsightsInput, InsightsReport};
use crate::domain::graph::{EdgeKind, GraphModel, NodeRef};
use crate::domain::insights::embedding::EmbeddingMap;
use crate::domain::insights::facts::{AuthorFacts, PostFacts, PostKind};
use crate::domain::insights::sets::{NamedSet, SetDimension};
use crate::domain::Post;
use std::collections::{BTreeMap, HashMap};

/// Sets kept per family (the largest ones).
pub const SETS_PER_FAMILY: usize = 12;

/// Keeps the largest non-empty sets (ties by key).
fn largest(groups: BTreeMap<String, (String, Vec<u32>)>) -> Vec<NamedSet> {
    let mut sets: Vec<NamedSet> = groups.into_iter().map(|(key, (label, members))| NamedSet::new(key, label, members)).filter(|s| !s.is_empty()).collect();
    sets.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.key.cmp(&b.key)));
    sets.truncate(SETS_PER_FAMILY);
    sets
}

/// Post index of every post node of `model`.
fn post_nodes(model: &GraphModel, posts: &[Post]) -> HashMap<usize, u32> {
    let index: HashMap<&str, u32> = posts.iter().enumerate().map(|(i, p)| (p.id.as_str(), i as u32)).collect();
    model.nodes().iter().enumerate().filter_map(|(n, node)| match &node.node {
        NodeRef::Post(id) => index.get(id.as_str()).map(|&i| (n, i)),
        _ => None,
    }).collect()
}

/// The four families.
pub(super) fn families(posts: &[Post], facts: &[PostFacts], authors: &[AuthorFacts], model: &GraphModel, input: &InsightsInput) -> Vec<(SetDimension, Vec<NamedSet>)> {
    let of_node = post_nodes(model, posts);
    let mut concepts: BTreeMap<String, (String, Vec<u32>)> = BTreeMap::new();
    let mut cited: BTreeMap<usize, Vec<u32>> = BTreeMap::new();
    for edge in model.edges() {
        let Some(&post) = of_node.get(&edge.source) else { continue };
        match (&model.nodes()[edge.target].node, edge.kind) {
            (NodeRef::Hashtag(tag), EdgeKind::Hashtag) => concepts.entry(tag.clone()).or_insert_with(|| (model.nodes()[edge.target].label.clone(), Vec::new())).1.push(post),
            (NodeRef::Document(_), EdgeKind::Wikilink) => cited.entry(edge.target).or_default().push(post),
            _ => {}
        }
    }
    let mut by_author: BTreeMap<String, (String, Vec<u32>)> = BTreeMap::new();
    let mut by_kind: BTreeMap<String, (String, Vec<u32>)> = BTreeMap::new();
    for (i, f) in facts.iter().enumerate() {
        let author = &authors[f.author];
        by_author.entry(author.id.as_str().to_string()).or_insert_with(|| (author.name.clone(), Vec::new())).1.push(i as u32);
        by_kind.entry(f.kind.key().to_string()).or_insert_with(|| (f.kind.key().to_string(), Vec::new())).1.push(i as u32);
    }
    vec![
        (SetDimension::Concept, largest(concepts)),
        (SetDimension::Author, largest(by_author)),
        (SetDimension::Kind, kinds_in_order(by_kind)),
        (SetDimension::Subject, largest(subjects(model, cited, input))),
    ]
}

/// Kind sets in the fixed kind order (labels are the kind keys, localised by the view).
fn kinds_in_order(mut groups: BTreeMap<String, (String, Vec<u32>)>) -> Vec<NamedSet> {
    PostKind::ALL.iter().filter_map(|k| groups.remove(k.key()).map(|(label, members)| NamedSet::new(k.key(), label, members))).collect()
}

/// Posts citing a document, grouped by the document's subjects.
fn subjects(model: &GraphModel, cited: BTreeMap<usize, Vec<u32>>, input: &InsightsInput) -> BTreeMap<String, (String, Vec<u32>)> {
    let mut groups: BTreeMap<String, (String, Vec<u32>)> = BTreeMap::new();
    for (node, posts) in cited {
        let NodeRef::Document(id) = &model.nodes()[node].node else { continue };
        let Some(doc) = input.documents.iter().find(|d| &d.id == id) else { continue };
        for subject in &doc.record.subjects {
            let label = input.subjects.iter().find(|(s, _)| s == subject).map_or_else(|| subject.as_str().to_string(), |(_, l)| l.clone());
            groups.entry(subject.as_str().to_string()).or_insert_with(|| (label, Vec::new())).1.extend(&posts);
        }
    }
    groups
}

/// Projects the analysed posts that have a vector, labelling each cluster by its top concept.
pub(super) fn attach_embedding(report: &mut InsightsReport, input: &InsightsInput) {
    let index: HashMap<&str, usize> = report.posts.iter().enumerate().map(|(i, p)| (p.id.as_str(), i)).collect();
    let (items, posts): (Vec<(String, Vec<f32>)>, Vec<usize>) =
        input.vectors.iter().filter_map(|(id, v)| index.get(id.as_str()).map(|&i| ((id.clone(), v.clone()), i))).unzip();
    report.embedding = EmbeddingMap::build(items);
    let concepts = report.families.iter().find(|(d, _)| *d == SetDimension::Concept).map(|(_, s)| s.clone()).unwrap_or_default();
    report.cluster_labels = report.embedding.cluster_members().iter().map(|members| {
        let in_cluster = |set: &NamedSet| members.iter().filter(|&&m| set.members.binary_search(&(posts[m] as u32)).is_ok()).count();
        concepts.iter().map(|s| (in_cluster(s), s)).filter(|(c, _)| *c > 0).max_by_key(|(c, s)| (*c, std::cmp::Reverse(s.key.clone()))).map(|(_, s)| s.label.clone()).unwrap_or_default()
    }).collect();
    report.embedding_posts = posts;
}
