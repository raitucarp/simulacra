//! Which pairs are worth a look. A contradiction needs a shared topic first, so candidates
//! are pairs that are close in meaning or, without vectors, that share words:
//!
//! - **By meaning:** every item with a vector takes its [`NEIGHBOURS`] nearest items (cosine
//!   at least the minimum similarity) among the other items with a vector.
//! - **By words:** a pair in which at least one item has no vector is a candidate when the two
//!   share at least [`MIN_SHARED_TERMS`] content terms (found through an inverted index; terms
//!   used by more than [`COMMON_TERM_LIMIT`] items are too common to mark a topic). Again each
//!   item keeps its [`NEIGHBOURS`] best.
//!
//! Pairs are unordered and deduplicated (meaning wins over words); an item is never paired
//! with itself, nor two sides with the same key or the same text.

use super::side::TensionSide;
use crate::domain::embedding::cosine;
use std::collections::{BTreeMap, BTreeSet};

/// Nearest items kept per item.
pub const NEIGHBOURS: usize = 8;
/// Content terms two items must share to pair up by words.
pub const MIN_SHARED_TERMS: usize = 2;
/// A term used by more items than this says nothing about a topic.
pub const COMMON_TERM_LIMIT: usize = 200;

/// One item the finder may pair: its side and its vector (unit length, one model) if any.
#[derive(Debug, Clone, PartialEq)]
pub struct CandidateInput {
    pub side: TensionSide,
    pub vector: Option<Vec<f32>>,
}

/// An unordered pair of item indices, smaller first.
pub type IndexPair = (usize, usize);

fn ordered(a: usize, b: usize) -> IndexPair {
    (a.min(b), a.max(b))
}

/// Candidate pairs of `items` (with their content `terms`), with the cosine of pairs found
/// by meaning; `focus` keeps only pairs including that item.
pub fn candidate_pairs(items: &[CandidateInput], terms: &[BTreeSet<String>], focus: Option<usize>, min_similarity: f32) -> BTreeMap<IndexPair, Option<f32>> {
    let by_words = word_pairs(items, terms, focus).into_iter().map(|pair| (pair, None));
    let by_meaning = meaning_pairs(items, focus, min_similarity).into_iter().map(|(pair, score)| (pair, Some(score)));
    by_words.chain(by_meaning).filter(|((a, b), _)| pairable(&items[*a].side, &items[*b].side)).collect()
}

/// Whether two sides may form a pair.
fn pairable(a: &TensionSide, b: &TensionSide) -> bool {
    let normal = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase();
    a.key() != b.key() && normal(&a.text) != normal(&b.text)
}

/// The rows to look at: the focus only, or every index passing `keep`.
fn rows(count: usize, focus: Option<usize>, keep: impl Fn(usize) -> bool) -> impl Iterator<Item = usize> {
    (0..count).filter(move |i| focus.is_none_or(|f| f == *i) && keep(*i))
}

/// Each vector item's nearest vector items.
fn meaning_pairs(items: &[CandidateInput], focus: Option<usize>, min_similarity: f32) -> Vec<(IndexPair, f32)> {
    let with_vector: Vec<usize> = (0..items.len()).filter(|i| items[*i].vector.is_some()).collect();
    rows(items.len(), focus, |i| items[i].vector.is_some()).flat_map(|i| nearest(items, &with_vector, i, min_similarity)).collect()
}

fn nearest(items: &[CandidateInput], with_vector: &[usize], row: usize, min_similarity: f32) -> Vec<(IndexPair, f32)> {
    let Some(vector) = items[row].vector.as_deref() else { return Vec::new() };
    let mut scored: Vec<(usize, f32)> = with_vector
        .iter()
        .filter(|other| **other != row)
        .filter_map(|&other| items[other].vector.as_deref().map(|v| (other, cosine(vector, v))))
        .filter(|(_, score)| *score >= min_similarity)
        .collect();
    scored.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    scored.into_iter().take(NEIGHBOURS).map(|(other, score)| (ordered(row, other), score)).collect()
}

/// Items by content term (terms of more than [`COMMON_TERM_LIMIT`] items left out).
fn inverted_index(terms: &[BTreeSet<String>]) -> BTreeMap<&str, Vec<usize>> {
    let mut postings: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (item, set) in terms.iter().enumerate() {
        set.iter().for_each(|term| postings.entry(term.as_str()).or_default().push(item));
    }
    postings.retain(|_, items| items.len() <= COMMON_TERM_LIMIT);
    postings
}

/// Pairs sharing enough words where at least one side has no vector.
fn word_pairs(items: &[CandidateInput], terms: &[BTreeSet<String>], focus: Option<usize>) -> Vec<IndexPair> {
    let postings = inverted_index(terms);
    let lacks_vector = |i: usize| items[i].vector.is_none();
    rows(items.len(), focus, |_| true).flat_map(|row| sharing(row, &terms[row], &postings, &lacks_vector)).collect()
}

/// `row`'s best word partners.
fn sharing(row: usize, own: &BTreeSet<String>, postings: &BTreeMap<&str, Vec<usize>>, lacks_vector: &dyn Fn(usize) -> bool) -> Vec<IndexPair> {
    let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
    own.iter().filter_map(|term| postings.get(term.as_str())).flatten().filter(|other| **other != row).for_each(|other| *counts.entry(*other).or_default() += 1);
    let mut partners: Vec<(usize, usize)> = counts
        .into_iter()
        .filter(|(other, shared)| *shared >= MIN_SHARED_TERMS && (lacks_vector(row) || lacks_vector(*other)))
        .collect();
    partners.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    partners.into_iter().take(NEIGHBOURS).map(|(other, _)| ordered(row, other)).collect()
}
