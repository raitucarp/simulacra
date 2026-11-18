//! The 2D embedding map: posts placed by the two principal components of their embedding
//! vectors (`domain::embedding::project_2d`), clustered by k-means in the full space, with
//! nearest neighbours in meaning (cosine) looked up for the hovered point.

pub mod kmeans;
#[cfg(test)]
mod tests;

use super::graph::layout::fit_unit;
use crate::domain::embedding::{dot, project_2d};
use std::sync::Arc;

/// Projected points (unit square), their clusters and the vectors they came from.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmbeddingMap {
    /// Owner id of each point (a post id).
    pub ids: Vec<String>,
    pub points: Vec<[f32; 2]>,
    pub clusters: Vec<usize>,
    pub cluster_count: usize,
    /// Unit vectors, shared with the hover lookup.
    pub vectors: Arc<Vec<Vec<f32>>>,
}

impl EmbeddingMap {
    /// Projects and clusters `items` (id and unit vector, all of one model and length).
    pub fn build(items: Vec<(String, Vec<f32>)>) -> Self {
        let (ids, vectors): (Vec<String>, Vec<Vec<f32>>) = items.into_iter().unzip();
        let projection = project_2d(&vectors);
        let cluster_count = kmeans::cluster_count(vectors.len());
        let clusters = kmeans::kmeans(&vectors, cluster_count);
        let points = fit_unit(&projection.points);
        Self { ids, points, clusters, cluster_count, vectors: Arc::new(vectors) }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// The `k` points most similar in meaning to point `index` (cosine of unit vectors), best
    /// first, with their similarity. O(n · d).
    pub fn neighbours(&self, index: usize, k: usize) -> Vec<(usize, f32)> {
        let Some(query) = self.vectors.get(index) else { return Vec::new() };
        let mut scored: Vec<(usize, f32)> =
            self.vectors.iter().enumerate().filter(|(i, _)| *i != index).map(|(i, v)| (i, dot(query, v))).collect();
        let k = k.min(scored.len());
        if k == 0 {
            return Vec::new();
        }
        scored.select_nth_unstable_by(k - 1, |a, b| b.1.total_cmp(&a.1));
        scored.truncate(k);
        scored.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        scored
    }

    /// Members of each cluster.
    pub fn cluster_members(&self) -> Vec<Vec<usize>> {
        let last = self.cluster_count.max(1) - 1;
        let mut members = vec![Vec::new(); last + 1];
        self.clusters.iter().enumerate().for_each(|(i, &c)| members[c.min(last)].push(i));
        members
    }
}
