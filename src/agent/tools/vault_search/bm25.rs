//! Okapi BM25 ranking over document chunks.

use super::chunk::{tokenize, Chunk};
use std::collections::HashMap;

/// Term-frequency saturation.
const K1: f64 = 1.2;
/// Length normalisation strength.
const B: f64 = 0.75;
/// Added inside the IDF ratio (standard BM25 smoothing).
const IDF_SMOOTHING: f64 = 0.5;

/// A scored chunk.
#[derive(Debug, Clone, PartialEq)]
pub struct Hit {
    pub chunk: Chunk,
    pub score: f64,
}

struct IndexedChunk {
    chunk: Chunk,
    term_freq: HashMap<String, u32>,
    length: usize,
}

/// An in-memory BM25 index.
pub struct Bm25Index {
    chunks: Vec<IndexedChunk>,
    doc_freq: HashMap<String, u32>,
    avg_length: f64,
}

impl Bm25Index {
    /// Indexes `chunks`.
    pub fn new(chunks: Vec<Chunk>) -> Self {
        let mut doc_freq: HashMap<String, u32> = HashMap::new();
        let indexed: Vec<IndexedChunk> = chunks
            .into_iter()
            .map(|chunk| {
                let tokens = tokenize(&format!("{} {}", chunk.title, chunk.text));
                let mut term_freq: HashMap<String, u32> = HashMap::new();
                for token in &tokens {
                    *term_freq.entry(token.clone()).or_default() += 1;
                }
                for term in term_freq.keys() {
                    *doc_freq.entry(term.clone()).or_default() += 1;
                }
                IndexedChunk { chunk, term_freq, length: tokens.len() }
            })
            .collect();
        let total: usize = indexed.iter().map(|c| c.length).sum();
        let avg_length = if indexed.is_empty() { 0.0 } else { total as f64 / indexed.len() as f64 };
        Self { chunks: indexed, doc_freq, avg_length }
    }

    /// Number of indexed chunks.
    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    /// Whether nothing is indexed.
    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    /// The `limit` best chunks for `query` with a positive score, best first.
    pub fn search(&self, query: &str, limit: usize) -> Vec<Hit> {
        let mut terms = tokenize(query);
        terms.sort();
        terms.dedup();
        let mut hits: Vec<Hit> = self
            .chunks
            .iter()
            .map(|indexed| Hit { chunk: indexed.chunk.clone(), score: self.score(indexed, &terms) })
            .filter(|hit| hit.score > 0.0)
            .collect();
        hits.sort_by(|a, b| b.score.total_cmp(&a.score));
        hits.truncate(limit);
        hits
    }

    fn score(&self, indexed: &IndexedChunk, terms: &[String]) -> f64 {
        let n = self.chunks.len() as f64;
        let norm = if self.avg_length > 0.0 { indexed.length as f64 / self.avg_length } else { 1.0 };
        terms
            .iter()
            .filter_map(|term| {
                let tf = f64::from(*indexed.term_freq.get(term)?);
                let df = f64::from(*self.doc_freq.get(term).unwrap_or(&0));
                let idf = (1.0 + (n - df + IDF_SMOOTHING) / (df + IDF_SMOOTHING)).ln();
                Some(idf * tf * (K1 + 1.0) / (tf + K1 * (1.0 - B + B * norm)))
            })
            .sum()
    }
}
