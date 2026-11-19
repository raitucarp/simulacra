//! A 2D projection of vectors by principal component analysis: the two leading eigenvectors
//! of the covariance, found by power iteration with deflation (no matrix is ever formed;
//! each step is two passes over the data).

use super::vector::{dot, l2_normalized};

/// Power-iteration steps per component.
const MAX_ITERATIONS: usize = 200;
/// Stop once a step moves the direction less than this.
const CONVERGENCE: f32 = 1e-6;

/// Projected points and the two components (unit vectors) they were projected on.
#[derive(Debug, Clone, PartialEq)]
pub struct Projection {
    pub points: Vec<[f32; 2]>,
    pub components: [Vec<f32>; 2],
}

/// Projects same-length `vectors` onto their first two principal components (all points at
/// the origin when there are fewer than two vectors or a single dimension).
pub fn project_2d(vectors: &[Vec<f32>]) -> Projection {
    let dims = vectors.first().map_or(0, Vec::len);
    let centered = centered(vectors, dims);
    let first = leading_component(&centered, dims, &[]);
    let second = leading_component(&centered, dims, std::slice::from_ref(&first));
    let points = centered.iter().map(|row| [dot(row, &first), dot(row, &second)]).collect();
    Projection { points, components: [first, second] }
}

/// The rows minus their mean.
fn centered(vectors: &[Vec<f32>], dims: usize) -> Vec<Vec<f32>> {
    let count = vectors.len().max(1) as f32;
    let mean: Vec<f32> = (0..dims).map(|d| vectors.iter().map(|v| v.get(d).copied().unwrap_or(0.0)).sum::<f32>() / count).collect();
    vectors.iter().map(|v| (0..dims).map(|d| v.get(d).copied().unwrap_or(0.0) - mean[d]).collect()).collect()
}

/// The leading eigenvector of the covariance of `rows`, orthogonal to `found`.
fn leading_component(rows: &[Vec<f32>], dims: usize, found: &[Vec<f32>]) -> Vec<f32> {
    let mut direction = orthogonalized(seed(dims), found);
    for _ in 0..MAX_ITERATIONS {
        let next = orthogonalized(covariance_times(rows, &direction, dims), found);
        if next.iter().all(|x| x.abs() <= f32::EPSILON) {
            return direction;
        }
        let moved: f32 = next.iter().zip(&direction).map(|(a, b)| (a - b).abs()).sum();
        direction = next;
        if moved < CONVERGENCE {
            break;
        }
    }
    direction
}

/// A deterministic start that is not orthogonal to typical data.
fn seed(dims: usize) -> Vec<f32> {
    (0..dims).map(|d| 1.0 / (d as f32 + 1.0)).collect()
}

/// `Xᵀ (X v)`: the covariance (up to scale) applied to `v`.
fn covariance_times(rows: &[Vec<f32>], v: &[f32], dims: usize) -> Vec<f32> {
    let weights: Vec<f32> = rows.iter().map(|row| dot(row, v)).collect();
    (0..dims).map(|d| rows.iter().zip(&weights).map(|(row, w)| row[d] * w).sum()).collect()
}

/// `v` with its projections on `basis` removed, scaled to unit length.
fn orthogonalized(v: Vec<f32>, basis: &[Vec<f32>]) -> Vec<f32> {
    let reduced = basis.iter().fold(v, |v, b| {
        let along = dot(&v, b);
        v.iter().zip(b).map(|(x, y)| x - along * y).collect()
    });
    l2_normalized(reduced)
}
