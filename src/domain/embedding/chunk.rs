//! Sentence-aware chunking with overlap. Whole sentences are packed into a chunk up to the
//! size limit; the next chunk starts with the trailing sentences that fit in the overlap.
//! A sentence longer than a chunk is cut by words.

use super::sentence::split_sentences;
use crate::domain::preferences::EmbeddingPrefs;

/// Characters per token in the token estimate (a common rule of thumb for English text).
const CHARS_PER_TOKEN: usize = 4;
/// Joins the pieces of a chunk.
const PIECE_SEPARATOR: &str = " ";

/// What a chunk size counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkUnit {
    /// Estimated tokens: each word counts `ceil(chars / 4)`, at least one.
    Tokens,
    Characters,
}

/// The chunk geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkSpec {
    pub size: usize,
    pub overlap: usize,
    pub unit: ChunkUnit,
}

impl ChunkSpec {
    /// Token chunks from the Embeddings preferences.
    pub fn from_prefs(prefs: &EmbeddingPrefs) -> Self {
        Self { size: prefs.chunk_size as usize, overlap: prefs.chunk_overlap as usize, unit: ChunkUnit::Tokens }
    }

    /// The size of `text` in this spec's unit.
    pub fn measure(&self, text: &str) -> usize {
        match self.unit {
            ChunkUnit::Characters => text.chars().count(),
            ChunkUnit::Tokens => text.split_whitespace().map(|word| word.chars().count().div_ceil(CHARS_PER_TOKEN).max(1)).sum(),
        }
    }
}

/// One chunk of a text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextChunk {
    pub index: usize,
    pub text: String,
}

/// The chunks of `text` (none for blank text).
pub fn chunk_text(text: &str, spec: ChunkSpec) -> Vec<TextChunk> {
    let size = spec.size.max(1);
    let pieces: Vec<(String, usize)> = split_sentences(text)
        .into_iter()
        .flat_map(|sentence| cut_long(sentence, size, &spec))
        .map(|piece| {
            let length = spec.measure(&piece);
            (piece, length)
        })
        .collect();
    pack(&pieces, size, spec.overlap.min(size / 2))
        .into_iter()
        .enumerate()
        .map(|(index, text)| TextChunk { index, text })
        .collect()
}

/// `sentence` itself, or word windows of at most `size` when it is longer.
fn cut_long(sentence: &str, size: usize, spec: &ChunkSpec) -> Vec<String> {
    if spec.measure(sentence) <= size {
        return vec![sentence.to_string()];
    }
    sentence.split_whitespace().fold(Vec::new(), |mut windows: Vec<String>, word| {
        match windows.last_mut() {
            Some(last) if spec.measure(last) + spec.measure(word) <= size => {
                last.push_str(PIECE_SEPARATOR);
                last.push_str(word);
            }
            _ => windows.push(word.to_string()),
        }
        windows
    })
}

/// Greedy packing of measured pieces into chunks, carrying up to `overlap` into the next.
fn pack(pieces: &[(String, usize)], size: usize, overlap: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut fresh = false;
    for (at, (_, length)) in pieces.iter().enumerate() {
        let used: usize = current.iter().map(|&i| pieces[i].1).sum();
        if fresh && used + length > size {
            chunks.push(joined(pieces, &current));
            current = overlap_tail(pieces, &current, overlap);
        }
        while !current.is_empty() && current.iter().map(|&i| pieces[i].1).sum::<usize>() + length > size {
            current.remove(0);
        }
        current.push(at);
        fresh = true;
    }
    if fresh {
        chunks.push(joined(pieces, &current));
    }
    chunks
}

/// The trailing pieces of `current` that fit in `overlap` (never all of them).
fn overlap_tail(pieces: &[(String, usize)], current: &[usize], overlap: usize) -> Vec<usize> {
    let mut total = 0;
    let tail: Vec<usize> = current
        .iter()
        .rev()
        .take(current.len().saturating_sub(1))
        .take_while(|&&i| {
            total += pieces[i].1;
            total <= overlap
        })
        .copied()
        .collect();
    tail.into_iter().rev().collect()
}

fn joined(pieces: &[(String, usize)], indices: &[usize]) -> String {
    indices.iter().map(|&i| pieces[i].0.as_str()).collect::<Vec<_>>().join(PIECE_SEPARATOR)
}
