//! How documents connect: posts that link to them (backlinks) and related entries.

mod related;
#[cfg(test)]
mod tests;

pub use related::{related_entries, RelatedEntry, RelatedReason, ScoredDocument};

use super::GroundingDocument;
use crate::domain::post::SourceKind;
use crate::domain::Post;
use std::collections::HashMap;

/// Whether `post` links to `doc`: a `[[wikilink]]` naming its id or title, a source chip, or the
/// root of a thread distilled into it (N15).
pub fn links_to(post: &Post, doc: &GroundingDocument) -> bool {
    let title = doc.title.trim().to_lowercase();
    let by_wikilink = post.linked_posts.iter().any(|target| {
        let target = target.as_str().trim();
        target == doc.id || target.to_lowercase() == title
    });
    let distilled = post.extras.distilled.as_ref().is_some_and(|d| d.doc_id == doc.id); // N15: the thread it was distilled from
    by_wikilink || distilled || post.extras.sources.iter().any(|s| s.kind == SourceKind::VaultDoc && s.id == doc.id)
}

/// Posts of the document's network that link to it, newest first (as `posts` are ordered).
pub fn backlinks<'a>(doc: &GroundingDocument, posts: &'a [Post]) -> Vec<&'a Post> {
    posts.iter().filter(|post| post.network_id == doc.network_id && links_to(post, doc)).collect()
}

/// Number of linking posts per document id, in one pass over `posts`.
pub fn link_counts(docs: &[GroundingDocument], posts: &[Post]) -> HashMap<String, usize> {
    let by_key: HashMap<(String, &str), &str> = docs
        .iter()
        .flat_map(|doc| {
            let network = doc.network_id.as_str();
            [((doc.id.clone(), network), doc.id.as_str()), ((doc.title.trim().to_lowercase(), network), doc.id.as_str())]
        })
        .collect();
    let mut counts: HashMap<String, usize> = HashMap::new();
    for post in posts {
        let network = post.network_id.as_str();
        let wikilinked = post.linked_posts.iter().filter_map(|t| {
            let target = t.as_str().trim();
            by_key.get(&(target.to_string(), network)).or_else(|| by_key.get(&(target.to_lowercase(), network)))
        });
        let sourced = post.extras.sources.iter().filter(|s| s.kind == SourceKind::VaultDoc).filter_map(|s| {
            docs.iter().find(|d| d.id == s.id && d.network_id == post.network_id).map(|d| d.id.as_str())
        });
        let mut linked: Vec<&str> = wikilinked.copied().chain(sourced).collect();
        linked.sort_unstable();
        linked.dedup();
        linked.into_iter().for_each(|id| *counts.entry(id.to_string()).or_default() += 1);
    }
    counts
}
