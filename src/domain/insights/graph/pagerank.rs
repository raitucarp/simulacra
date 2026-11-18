//! Influence by PageRank: power iteration over the directed arcs (a reply passes influence to
//! the post it answers). Dangling nodes spread their rank evenly. O(k · (n + m)) for at most
//! [`MAX_ITERATIONS`] rounds, stopping early once the total change drops under [`TOLERANCE`].

use super::Graph;

/// Probability of following an arc rather than jumping anywhere.
pub const DAMPING: f64 = 0.85;
pub const MAX_ITERATIONS: usize = 100;
/// Stop when the L1 change of a round is below this.
pub const TOLERANCE: f64 = 1e-9;

/// Ranks summing to one (empty for an empty graph).
pub fn pagerank(graph: &Graph) -> Vec<f64> {
    let n = graph.node_count();
    if n == 0 {
        return Vec::new();
    }
    let uniform = 1.0 / n as f64;
    let mut rank = vec![uniform; n];
    for _ in 0..MAX_ITERATIONS {
        let next = step(graph, &rank, uniform);
        let change: f64 = next.iter().zip(&rank).map(|(a, b)| (a - b).abs()).sum();
        rank = next;
        if change < TOLERANCE {
            break;
        }
    }
    rank
}

/// One power-iteration round.
fn step(graph: &Graph, rank: &[f64], uniform: f64) -> Vec<f64> {
    let n = rank.len();
    let dangling: f64 = (0..n).filter(|&v| graph.out_degree(v) == 0).map(|v| rank[v]).sum();
    let base = (1.0 - DAMPING) * uniform + DAMPING * dangling * uniform;
    let mut next = vec![base; n];
    for (node, &share) in rank.iter().enumerate() {
        let out = graph.out_degree(node);
        if out == 0 {
            continue;
        }
        let passed = DAMPING * share / out as f64;
        graph.successors(node).iter().for_each(|&to| next[to as usize] += passed);
    }
    next
}
