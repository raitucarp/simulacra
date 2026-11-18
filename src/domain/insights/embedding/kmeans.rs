//! Deterministic k-means (Lloyd's algorithm) for clustering embedding vectors.
//!
//! Seeds by farthest-point traversal from the first vector, so the same input always yields
//! the same clusters. Large inputs are fitted on an evenly strided sample of [`MAX_SAMPLE`]
//! vectors and every vector is then assigned to its nearest centre:
//! O(rounds · min(n, MAX_SAMPLE) · k · d + n · k · d).

/// Vectors fitted at most; the rest are only assigned.
pub const MAX_SAMPLE: usize = 1_500;
pub const MAX_ROUNDS: usize = 25;
pub const MIN_CLUSTERS: usize = 2;
pub const MAX_CLUSTERS: usize = 8;

/// A pleasant number of clusters for `n` points: `√(n / 2)`, within the bounds (1 for tiny inputs).
pub fn cluster_count(n: usize) -> usize {
    if n < MIN_CLUSTERS * 2 {
        return 1;
    }
    ((n as f64 / 2.0).sqrt().round() as usize).clamp(MIN_CLUSTERS, MAX_CLUSTERS)
}

fn distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| (x - y) * (x - y)).sum()
}

/// Index of the centre closest to `v`.
pub fn nearest_centre(v: &[f32], centres: &[Vec<f32>]) -> usize {
    centres
        .iter()
        .enumerate()
        .map(|(i, c)| (i, distance(v, c)))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map_or(0, |(i, _)| i)
}

fn farthest_point_seeds(sample: &[&Vec<f32>], k: usize) -> Vec<Vec<f32>> {
    let mut centres = vec![sample[0].clone()];
    let mut gap: Vec<f32> = sample.iter().map(|v| distance(v, &centres[0])).collect();
    while centres.len() < k {
        let Some((far, _)) = gap.iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)) else { break };
        centres.push(sample[far].clone());
        let newest = centres.last().cloned().unwrap_or_default();
        gap.iter_mut().zip(sample).for_each(|(g, v)| *g = g.min(distance(v, &newest)));
    }
    centres
}

/// Mean of the members of each cluster (an empty cluster keeps its centre).
fn recentre(sample: &[&Vec<f32>], assignment: &[usize], centres: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let dims = centres.first().map_or(0, Vec::len);
    let mut sums = vec![vec![0.0f32; dims]; centres.len()];
    let mut counts = vec![0usize; centres.len()];
    for (v, &c) in sample.iter().zip(assignment) {
        counts[c] += 1;
        sums[c].iter_mut().zip(v.iter()).for_each(|(s, x)| *s += x);
    }
    sums.into_iter()
        .zip(&counts)
        .zip(centres)
        .map(|((sum, &count), old)| if count == 0 { old.clone() } else { sum.into_iter().map(|s| s / count as f32).collect() })
        .collect()
}

/// The cluster of every vector, with `k` clusters (at most the number of vectors).
pub fn kmeans(vectors: &[Vec<f32>], k: usize) -> Vec<usize> {
    if vectors.is_empty() || k <= 1 {
        return vec![0; vectors.len()];
    }
    let stride = vectors.len().div_ceil(MAX_SAMPLE);
    let sample: Vec<&Vec<f32>> = vectors.iter().step_by(stride).collect();
    let mut centres = farthest_point_seeds(&sample, k.min(sample.len()));
    let mut assignment: Vec<usize> = sample.iter().map(|v| nearest_centre(v, &centres)).collect();
    for _ in 0..MAX_ROUNDS {
        centres = recentre(&sample, &assignment, &centres);
        let next: Vec<usize> = sample.iter().map(|v| nearest_centre(v, &centres)).collect();
        if next == assignment {
            break;
        }
        assignment = next;
    }
    vectors.iter().map(|v| nearest_centre(v, &centres)).collect()
}
