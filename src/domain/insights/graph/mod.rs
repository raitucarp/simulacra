//! Graph theory over a simple graph in compressed adjacency form.
//!
//! [`Graph`] keeps both views of the same edges: the directed arcs as given (a reply points
//! at its parent, a post at the hashtag it carries) and their undirected closure without
//! duplicates or self-loops. Every metric works on `n` nodes and `m` undirected edges:
//!
//! | metric | module | cost |
//! |---|---|---|
//! | degree, in/out degree, density | [`degree`] | O(n + m) |
//! | influence (PageRank) | [`pagerank`] | O(k · (n + m)), k ≤ 100 iterations |
//! | betweenness (Brandes) | [`betweenness`] | O(s · (n + m)), s ≤ 256 sampled sources |
//! | clustering coefficient | [`triangles`] | O(m · √m) (degree-ordered triangles) |
//! | connected components | [`components`] | O(n + m) |
//! | communities (Louvain) | [`community`] | O(k · (n + m)) per level, k ≤ 30 rounds, ≤ 8 levels |
//! | bridges, articulation points | [`bridges`] | O(n + m), iterative DFS |
//! | shortest paths, eccentricity | [`paths`] | O(n + m) per breadth-first search |
//! | force layout | [`layout`] | O(k · (n + m)), grid-bucketed repulsion |

pub mod betweenness;
pub mod bridges;
pub mod community;
pub mod components;
pub mod degree;
pub mod layout;
pub mod metrics;
pub mod pagerank;
pub mod paths;
pub mod triangles;

#[cfg(test)]
mod tests;

pub use metrics::NetworkMetrics;

/// A node index, compact so large graphs stay cache friendly.
pub type Node = u32;

/// A simple graph: directed arcs plus their undirected closure, both in CSR form.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Graph {
    offsets: Vec<usize>,
    neighbours: Vec<Node>,
    out_offsets: Vec<usize>,
    successors: Vec<Node>,
    in_degree: Vec<usize>,
}

/// Compressed rows of `pairs` (sorted, deduplicated) over `count` nodes.
fn compressed(count: usize, mut pairs: Vec<(Node, Node)>) -> (Vec<usize>, Vec<Node>) {
    pairs.sort_unstable();
    pairs.dedup();
    let mut offsets = vec![0usize; count + 1];
    pairs.iter().for_each(|&(from, _)| offsets[from as usize + 1] += 1);
    (0..count).for_each(|i| offsets[i + 1] += offsets[i]);
    (offsets, pairs.into_iter().map(|(_, to)| to).collect())
}

impl Graph {
    /// Builds the graph of `count` nodes from directed `arcs`; arcs touching a node outside
    /// `0..count` and self-loops are dropped.
    pub fn new(count: usize, arcs: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let arcs: Vec<(Node, Node)> = arcs
            .into_iter()
            .filter(|&(a, b)| a != b && a < count && b < count)
            .map(|(a, b)| (a as Node, b as Node))
            .collect();
        let both = arcs.iter().flat_map(|&(a, b)| [(a, b), (b, a)]).collect();
        let (offsets, neighbours) = compressed(count, both);
        let (out_offsets, successors) = compressed(count, arcs);
        let mut in_degree = vec![0usize; count];
        successors.iter().for_each(|&to| in_degree[to as usize] += 1);
        Self { offsets, neighbours, out_offsets, successors, in_degree }
    }

    pub fn node_count(&self) -> usize {
        self.offsets.len().saturating_sub(1)
    }

    /// Undirected edges (each pair once).
    pub fn edge_count(&self) -> usize {
        self.neighbours.len() / 2
    }

    /// Distinct directed arcs.
    pub fn arc_count(&self) -> usize {
        self.successors.len()
    }

    /// Undirected neighbours of `node`, ascending.
    pub fn neighbours(&self, node: usize) -> &[Node] {
        &self.neighbours[self.offsets[node]..self.offsets[node + 1]]
    }

    /// Targets of the arcs leaving `node`, ascending.
    pub fn successors(&self, node: usize) -> &[Node] {
        &self.successors[self.out_offsets[node]..self.out_offsets[node + 1]]
    }

    pub fn degree(&self, node: usize) -> usize {
        self.offsets[node + 1] - self.offsets[node]
    }

    pub fn out_degree(&self, node: usize) -> usize {
        self.out_offsets[node + 1] - self.out_offsets[node]
    }

    pub fn in_degree(&self, node: usize) -> usize {
        self.in_degree[node]
    }

    /// Every undirected edge once, as `(low, high)`.
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.node_count()).flat_map(move |a| {
            self.neighbours(a).iter().map(|&b| b as usize).filter(move |&b| a < b).map(move |b| (a, b))
        })
    }
}
