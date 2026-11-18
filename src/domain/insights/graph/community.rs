//! Communities by the Louvain method, scored by Newman modularity.
//!
//! Level by level: every node starts alone and, in index order, moves to the neighbouring
//! community with the largest modularity gain `k_in − Σ_tot · k / 2m` (staying put unless a
//! move is strictly better, ties going to the smallest community id), until nothing moves
//! (at most [`MAX_ROUNDS`] rounds); then each community becomes one weighted node and the
//! next level starts, until a level merges nothing (at most [`MAX_LEVELS`]). Each round is
//! O(n + m) on its level's graph, which shrinks quickly. Deterministic, a local optimum.

use super::Graph;
use std::collections::{BTreeMap, HashMap};

pub const MAX_ROUNDS: usize = 30;
pub const MAX_LEVELS: usize = 8;
/// Gains closer than this count as equal.
const GAIN_EPSILON: f64 = 1e-12;

/// Community of every node, numbered by size (largest first), and the partition's modularity.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Communities {
    pub of_node: Vec<usize>,
    pub sizes: Vec<usize>,
    /// Newman modularity in `-0.5..=1` (0 for an edgeless graph).
    pub modularity: f64,
}

impl Communities {
    pub fn count(&self) -> usize {
        self.sizes.len()
    }
}

/// A weighted graph of one level: adjacency (self entries carry internal weight), weighted
/// degrees and their sum `2m`.
struct Level {
    adjacency: Vec<Vec<(usize, f64)>>,
    degree: Vec<f64>,
    twice_edges: f64,
}

impl Level {
    fn of(graph: &Graph) -> Self {
        let adjacency: Vec<Vec<(usize, f64)>> = (0..graph.node_count()).map(|v| graph.neighbours(v).iter().map(|&w| (w as usize, 1.0)).collect()).collect();
        Self::from_adjacency(adjacency)
    }

    fn from_adjacency(adjacency: Vec<Vec<(usize, f64)>>) -> Self {
        let degree: Vec<f64> = adjacency.iter().map(|row| row.iter().map(|(_, w)| w).sum()).collect();
        let twice_edges = degree.iter().sum();
        Self { adjacency, degree, twice_edges }
    }

    /// Local moving: the community of every node of this level.
    fn local_moving(&self) -> Vec<usize> {
        let n = self.adjacency.len();
        let (mut community, mut total) = ((0..n).collect::<Vec<usize>>(), self.degree.clone());
        for _ in 0..MAX_ROUNDS {
            let mut moved = false;
            for v in 0..n {
                let (k, current) = (self.degree[v], community[v]);
                total[current] -= k;
                let mut links: BTreeMap<usize, f64> = BTreeMap::new();
                self.adjacency[v].iter().filter(|(w, _)| *w != v).for_each(|&(w, weight)| *links.entry(community[w]).or_default() += weight);
                let gain = |c: usize, k_in: f64| k_in - total[c] * k / self.twice_edges;
                let mut best = (current, gain(current, links.get(&current).copied().unwrap_or_default()));
                for (&c, &k_in) in &links {
                    if gain(c, k_in) > best.1 + GAIN_EPSILON {
                        best = (c, gain(c, k_in));
                    }
                }
                (community[v], total[best.0]) = (best.0, total[best.0] + k);
                moved |= best.0 != current;
            }
            if !moved {
                break;
            }
        }
        community
    }

    /// The next level: one node per community (`labels` renumbered `0..k`).
    fn aggregate(&self, labels: &[usize], count: usize) -> Self {
        let mut rows: Vec<BTreeMap<usize, f64>> = vec![BTreeMap::new(); count];
        for (v, row) in self.adjacency.iter().enumerate() {
            row.iter().for_each(|&(w, weight)| *rows[labels[v]].entry(labels[w]).or_default() += weight);
        }
        Self::from_adjacency(rows.into_iter().map(|row| row.into_iter().collect()).collect())
    }
}

/// Detects communities.
pub fn communities(graph: &Graph) -> Communities {
    let mut assignment: Vec<usize> = (0..graph.node_count()).collect();
    let mut level = Level::of(graph);
    for _ in 0..MAX_LEVELS {
        if level.twice_edges == 0.0 {
            break;
        }
        let (labels, sizes) = renumber(&level.local_moving());
        if sizes.len() == level.adjacency.len() {
            break;
        }
        assignment = assignment.iter().map(|&a| labels[a]).collect();
        level = level.aggregate(&labels, sizes.len());
    }
    let (of_node, sizes) = renumber(&assignment);
    let modularity = modularity(graph, &of_node, sizes.len());
    Communities { of_node, sizes, modularity }
}

/// Renames labels to `0..k` by descending size (ties by smallest original label).
fn renumber(labels: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let mut sizes: HashMap<usize, usize> = HashMap::new();
    labels.iter().for_each(|&l| *sizes.entry(l).or_default() += 1);
    let mut order: Vec<(usize, usize)> = sizes.into_iter().collect();
    order.sort_by_key(|&(label, size)| (std::cmp::Reverse(size), label));
    let rename: HashMap<usize, usize> = order.iter().enumerate().map(|(new, &(old, _))| (old, new)).collect();
    (labels.iter().map(|l| rename[l]).collect(), order.into_iter().map(|(_, size)| size).collect())
}

/// `Q = Σ_c (L_c / m − (D_c / 2m)²)` with `L_c` internal edges and `D_c` total degree of `c`.
pub fn modularity(graph: &Graph, of_node: &[usize], count: usize) -> f64 {
    let m = graph.edge_count() as f64;
    if m == 0.0 {
        return 0.0;
    }
    let (mut internal, mut degree) = (vec![0.0f64; count], vec![0.0f64; count]);
    for (a, b) in graph.edges() {
        if of_node[a] == of_node[b] {
            internal[of_node[a]] += 1.0;
        }
    }
    (0..graph.node_count()).for_each(|v| degree[of_node[v]] += graph.degree(v) as f64);
    internal.iter().zip(&degree).map(|(l, d)| l / m - (d / (2.0 * m)).powi(2)).sum()
}
