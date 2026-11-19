//! Degree statistics and density: O(n + m).

use super::Graph;

/// Undirected, incoming and outgoing degree of one node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Degree {
    pub total: usize,
    pub incoming: usize,
    pub outgoing: usize,
}

/// The degree of every node.
pub fn degrees(graph: &Graph) -> Vec<Degree> {
    (0..graph.node_count())
        .map(|node| Degree { total: graph.degree(node), incoming: graph.in_degree(node), outgoing: graph.out_degree(node) })
        .collect()
}

/// Share of the possible undirected edges that exist, `2m / (n (n − 1))` (zero below two nodes).
pub fn density(graph: &Graph) -> f64 {
    let n = graph.node_count() as f64;
    if n < 2.0 {
        return 0.0;
    }
    2.0 * graph.edge_count() as f64 / (n * (n - 1.0))
}

/// Mean undirected degree, `2m / n`.
pub fn mean_degree(graph: &Graph) -> f64 {
    let n = graph.node_count();
    if n == 0 {
        return 0.0;
    }
    2.0 * graph.edge_count() as f64 / n as f64
}
