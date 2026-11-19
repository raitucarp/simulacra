//! What is known about each concept the learner refers to.
//!
//! **Which concepts:** those of the learner's own posts (wikilinks, hashtags and the concept
//! extractor's first picks) that appear in at least [`MIN_POST_MENTIONS`] posts or were
//! marked by the author, review item tags, typed tutor concepts and pinned concepts (a
//! goal's); at most [`MAX_CONCEPTS`], most mentioned first.

use crate::domain::learning::bloom::BloomLevel;
use crate::domain::learning::concept::{appears_in, concept_key, concepts_of, dedup, marked_concepts, searchable};
use crate::domain::learning::goals::OwnPost;
use crate::domain::learning::tutor::{TutorSession, TutorSource};
use jiff::civil::Date;
use std::collections::HashMap;

/// Posts an unmarked concept must appear in to be shown.
pub const MIN_POST_MENTIONS: usize = 2;
/// Concepts on the radar at most.
pub const MAX_CONCEPTS: usize = 24;

/// A vault document as the radar reads it (the caller may cut long texts).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceText {
    pub title: String,
    pub text: String,
}

/// A review item as the radar reads it: its faces' text, tags and memory.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemText {
    pub text: String,
    pub tags: Vec<String>,
    pub retrievability: Option<f64>,
    pub last_review: Option<Date>,
}

/// What the radar is computed from.
#[derive(Debug, Clone, Copy)]
pub struct GapInputs<'a> {
    pub own_posts: &'a [OwnPost],
    pub sources: &'a [SourceText],
    pub items: &'a [ItemText],
    pub sessions: &'a [TutorSession],
    /// Concepts always shown (a goal's).
    pub pinned: &'a [String],
    /// When set, only these concepts are shown (a goal's radar).
    pub only: Option<&'a [String]>,
    pub today: Date,
}

/// The evidence on one concept.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptEvidence {
    pub concept: String,
    /// Own posts, sources, items and sessions that refer to it.
    pub mentions: usize,
    pub own_posts: usize,
    pub sources: usize,
    pub items: usize,
    /// Mean retrievability of its items that have a memory.
    pub retention: Option<f64>,
    pub mastery: Option<BloomLevel>,
    /// Distinct other concepts it shares an own post with.
    pub degree: usize,
    /// Last day it was written about or reviewed.
    pub last_touch: Option<Date>,
}

fn typed_concept(session: &TutorSession) -> Option<String> {
    match &session.source {
        TutorSource::Concept { name } => Some(concept_key(name)),
        _ => None,
    }
}

/// The concept keys the radar shows (see the module notes).
pub fn radar_concepts(inputs: &GapInputs<'_>, post_concepts: &[Vec<String>]) -> Vec<String> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    post_concepts.iter().flatten().for_each(|key| *counts.entry(key.as_str()).or_default() += 1);
    let marked: Vec<String> = inputs.own_posts.iter().flat_map(|post| marked_concepts(&post.text)).collect();
    let pinned = inputs.pinned.iter().map(|name| concept_key(name));
    let tags = inputs.items.iter().flat_map(|item| item.tags.iter().map(|tag| concept_key(tag)));
    let typed = inputs.sessions.iter().filter_map(typed_concept);
    let mut wanted: Vec<String> = dedup(counts.iter().filter(|(_, n)| **n >= MIN_POST_MENTIONS).map(|(k, _)| k.to_string()).chain(marked).chain(tags).chain(typed));
    wanted.sort_by(|a, b| counts.get(b.as_str()).cmp(&counts.get(a.as_str())).then(a.cmp(b)));
    let chosen = dedup(pinned.chain(wanted).filter(|key| !key.is_empty())).into_iter().take(MAX_CONCEPTS.max(inputs.pinned.len()));
    match inputs.only {
        Some(only) => dedup(only.iter().map(|name| concept_key(name)).filter(|key| !key.is_empty())),
        None => chosen.collect(),
    }
}

/// Texts prepared once for the many concept checks.
struct Prepared {
    posts: Vec<String>,
    sources: Vec<String>,
    items: Vec<String>,
    sessions: Vec<String>,
}

fn prepare(inputs: &GapInputs<'_>) -> Prepared {
    Prepared {
        posts: inputs.own_posts.iter().map(|post| searchable(&post.text)).collect(),
        sources: inputs.sources.iter().map(|doc| searchable(&format!("{} {}", doc.title, doc.text))).collect(),
        items: inputs.items.iter().map(|item| searchable(&format!("{} {}", item.text, item.tags.join(" ")))).collect(),
        sessions: inputs.sessions.iter().map(|s| searchable(&format!("{} {}", s.title, s.material))).collect(),
    }
}

fn hits(texts: &[String], key: &str) -> Vec<usize> {
    texts.iter().enumerate().filter(|(_, text)| appears_in(text, key)).map(|(at, _)| at).collect()
}

/// The evidence on every radar concept.
pub fn collect_evidence(inputs: &GapInputs<'_>) -> Vec<ConceptEvidence> {
    let post_concepts: Vec<Vec<String>> = inputs.own_posts.iter().map(|post| concepts_of(&post.text)).collect();
    let prepared = prepare(inputs);
    radar_concepts(inputs, &post_concepts).into_iter().map(|key| evidence_of(key, inputs, &prepared, &post_concepts)).collect()
}

fn evidence_of(concept: String, inputs: &GapInputs<'_>, prepared: &Prepared, post_concepts: &[Vec<String>]) -> ConceptEvidence {
    let posts: Vec<usize> = (0..inputs.own_posts.len()).filter(|at| post_concepts[*at].contains(&concept) || appears_in(&prepared.posts[*at], &concept)).collect();
    let sources = hits(&prepared.sources, &concept).len();
    let items: Vec<usize> = hits(&prepared.items, &concept);
    let sessions: Vec<usize> = (0..inputs.sessions.len()).filter(|at| typed_concept(&inputs.sessions[*at]).as_deref() == Some(concept.as_str()) || appears_in(&prepared.sessions[*at], &concept)).collect();
    let recall: Vec<f64> = items.iter().filter_map(|at| inputs.items[*at].retrievability).collect();
    let mut neighbours: Vec<&String> = posts.iter().flat_map(|at| post_concepts[*at].iter()).filter(|other| **other != concept).collect();
    neighbours.sort();
    neighbours.dedup();
    let last_post = posts.iter().map(|at| inputs.own_posts[*at].day).max();
    let last_review = items.iter().filter_map(|at| inputs.items[*at].last_review).max();
    ConceptEvidence {
        mentions: posts.len() + sources + items.len() + sessions.len(),
        own_posts: posts.len(),
        sources,
        items: items.len(),
        retention: (!recall.is_empty()).then(|| recall.iter().sum::<f64>() / recall.len() as f64),
        mastery: sessions.iter().filter_map(|at| inputs.sessions[*at].mastery()).max(),
        degree: neighbours.len(),
        last_touch: last_post.max(last_review),
        concept,
    }
}
