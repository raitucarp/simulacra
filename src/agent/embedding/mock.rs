//! Offline mock embeddings: a hashed bag of words. Texts that share words get similar
//! vectors, so search and "find similar" behave plausibly in tests without any network.

use crate::domain::embedding::{content_hash, l2_normalized};

/// Shortest word that counts.
const MIN_WORD_CHARS: usize = 3;
/// Hex digits of the hash used for the bucket and the sign.
const BUCKET_HEX_DIGITS: usize = 8;

/// The mock vector of `text` with `dims` components (unit length; blank text points along
/// the first axis).
pub fn mock_vector(text: &str, dims: usize) -> Vec<f32> {
    let dims = dims.max(1);
    let mut vector = vec![0.0_f32; dims];
    let words = text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| word.chars().count() >= MIN_WORD_CHARS)
        .map(|word| word.to_lowercase().trim_end_matches('s').to_string());
    for word in words {
        let hash = u64::from_str_radix(&content_hash(&[&word])[..BUCKET_HEX_DIGITS], 16).unwrap_or_default();
        let bucket = (hash as usize) % dims;
        let sign = if (hash >> 1) & 1 == 0 { 1.0 } else { -1.0 };
        vector[bucket] += sign;
    }
    if vector.iter().all(|x| *x == 0.0) {
        vector[0] = 1.0;
    }
    l2_normalized(vector)
}
