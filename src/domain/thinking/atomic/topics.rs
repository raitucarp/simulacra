//! Topics of a note: its strongest concepts (wikilinks, hashtags, names and terms first, as
//! the concept extractor ranks them), grouped by the claims they appear in. Two concepts
//! named in one claim belong to one topic, and so does anything linked to them through
//! another claim (connected components); a note whose concepts fall into separate groups
//! talks about separate things.

use crate::domain::post::HASHTAG_SIGIL;
use crate::domain::post_actions::extract_concepts;
use std::collections::BTreeSet;

/// Concepts considered: the extractor's best few (later ones are mere long words).
pub const TOPIC_CONCEPTS: usize = 4;

/// One topic: its first concept names it; the claims it appears in hold it together.
struct Topic {
    label: String,
    claims: BTreeSet<usize>,
}

/// Claim positions mentioning `concept` (case-insensitive; a hashtag's `_` and `-` read as
/// spaces, as the extractor reads them).
fn mentions(concept: &str, claims: &[String]) -> BTreeSet<usize> {
    let needle = concept.to_lowercase();
    claims
        .iter()
        .enumerate()
        .filter(|(_, claim)| claim.to_lowercase().replace(HASHTAG_SIGIL, "").replace(['_', '-'], " ").contains(&needle))
        .map(|(at, _)| at)
        .collect()
}

/// Adds a concept seen in `claims`, merging every topic that shares one of them.
fn merged(topics: Vec<Topic>, concept: String, claims: BTreeSet<usize>) -> Vec<Topic> {
    let (joined, apart): (Vec<Topic>, Vec<Topic>) = topics.into_iter().partition(|topic| !topic.claims.is_disjoint(&claims));
    let label = joined.first().map_or(concept, |first| first.label.clone());
    let claims: BTreeSet<usize> = joined.iter().flat_map(|topic| topic.claims.iter().copied()).chain(claims).collect();
    let mut all = apart;
    all.push(Topic { label, claims });
    all
}

/// The topics of `text` given its `claims`, by the order their names first appear.
pub fn topics_of(text: &str, claims: &[String]) -> Vec<String> {
    let concepts = extract_concepts(text, |_| false).into_iter().take(TOPIC_CONCEPTS);
    let topics = concepts.fold(Vec::new(), |topics, concept| {
        let found = mentions(&concept, claims);
        if found.is_empty() { topics } else { merged(topics, concept, found) }
    });
    let mut ordered: Vec<(usize, String)> = topics.into_iter().map(|topic| (topic.claims.first().copied().unwrap_or_default(), topic.label)).collect();
    ordered.sort();
    ordered.into_iter().map(|(_, label)| label).collect()
}
