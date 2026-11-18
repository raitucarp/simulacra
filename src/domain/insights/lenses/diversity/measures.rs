//! Variety measures over counts per source (authors, stances, schools, subjects):
//!
//! - **Shannon entropy** H = −Σ pᵢ·log₂ pᵢ, in bits (C. E. Shannon, 1948, "A mathematical
//!   theory of communication").
//! - **Evenness** H / log₂ k over the k sources seen (E. C. Pielou, 1966): 1 when every source
//!   is equally frequent, near 0 when one dominates.
//! - **Effective number** 2^H (M. O. Hill, 1973, "Diversity and evenness"): as varied as this
//!   many equally frequent sources.
//! - **Simpson index** 1 − Σ nᵢ(nᵢ−1) / (N(N−1)) (E. H. Simpson, 1949, "Measurement of
//!   diversity"): the chance that two items picked at random come from different sources.
//!
//! And the spread in meaning: the mean cosine distance (1 − cosine) between embedding vectors.

use crate::domain::embedding::cosine;

/// Vectors compared at most (an even stride through them), so the pairwise mean stays cheap.
pub const MAX_COMPARED: usize = 300;

/// How varied a distribution is.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Variety {
    /// Sources with at least one item.
    pub sources: usize,
    pub items: usize,
    pub entropy: f64,
    pub evenness: f64,
    pub effective: f64,
    pub simpson: f64,
}

/// The variety of `counts` (one count per source; zeros are ignored).
pub fn variety(counts: &[usize]) -> Variety {
    let present: Vec<f64> = counts.iter().filter(|&&n| n > 0).map(|&n| n as f64).collect();
    let items: f64 = present.iter().sum();
    if present.is_empty() {
        return Variety::default();
    }
    let entropy = -present.iter().map(|n| n / items).map(|p| p * p.log2()).sum::<f64>();
    let evenness = if present.len() > 1 { entropy / (present.len() as f64).log2() } else { 0.0 };
    let pairs = items * (items - 1.0);
    let simpson = if pairs > 0.0 { 1.0 - present.iter().map(|n| n * (n - 1.0)).sum::<f64>() / pairs } else { 0.0 };
    Variety { sources: present.len(), items: items as usize, entropy, evenness, effective: entropy.exp2(), simpson }
}

/// Mean cosine distance between every pair of (at most [`MAX_COMPARED`]) vectors, or `None`
/// for fewer than two.
pub fn mean_pairwise_distance(vectors: &[&[f32]]) -> Option<f64> {
    let stride = vectors.len().div_ceil(MAX_COMPARED).max(1);
    let sample: Vec<&[f32]> = vectors.iter().step_by(stride).copied().collect();
    let pairs = sample.len() * sample.len().saturating_sub(1) / 2;
    if pairs == 0 {
        return None;
    }
    let total: f64 = sample.iter().enumerate().flat_map(|(i, a)| sample[i + 1..].iter().map(move |b| f64::from(1.0 - cosine(a, b)))).sum();
    Some(total / pairs as f64)
}

/// The summed direction of `vectors` (cosine ignores length; `None` when empty).
pub fn centroid(vectors: &[&[f32]]) -> Option<Vec<f32>> {
    let first = vectors.first()?;
    let dims = first.len();
    let mut sum = vec![0.0f32; dims];
    vectors.iter().filter(|v| v.len() == dims).for_each(|v| sum.iter_mut().zip(v.iter()).for_each(|(s, x)| *s += x));
    Some(sum)
}
