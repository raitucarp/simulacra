//! Betweenness centrality by Brandes' algorithm on the undirected, unweighted graph.
//!
//! Exact for graphs of at most [`MAX_SOURCES`] nodes: one breadth-first search per source,
//! O(n · (n + m)). Larger graphs use [`MAX_SOURCES`] evenly strided sources and scale the
//! result by `n / s` (the standard pivot estimate), so the cost stays O(s · (n + m)).
//! Scores are normalised to `0..=1` by the pair count `(n − 1)(n − 2) / 2`.

use super::Graph;
use std::collections::VecDeque;

/// Breadth-first searches run at most.
pub const MAX_SOURCES: usize = 256;

/// Normalised betweenness of every node, and whether it was estimated from sampled sources.
pub fn betweenness(graph: &Graph) -> (Vec<f64>, bool) {
    let n = graph.node_count();
    let sources = sample_sources(n);
    let sampled = sources.len() < n;
    let mut score = vec![0.0f64; n];
    for &source in &sources {
        accumulate(graph, source, &mut score);
    }
    let scale = if sampled { n as f64 / sources.len().max(1) as f64 } else { 1.0 };
    let pairs = ((n.saturating_sub(1)) * (n.saturating_sub(2))) as f64;
    // Each undirected pair is counted from both ends, hence `pairs` rather than `pairs / 2`.
    let norm = if pairs > 0.0 { scale / pairs } else { 0.0 };
    (score.into_iter().map(|s| (s * norm).min(1.0)).collect(), sampled)
}

/// Every node, or `MAX_SOURCES` nodes spread evenly over the index range.
fn sample_sources(n: usize) -> Vec<usize> {
    if n <= MAX_SOURCES {
        return (0..n).collect();
    }
    (0..MAX_SOURCES).map(|i| i * n / MAX_SOURCES).collect()
}

/// Adds the dependencies of `source` (one Brandes pass) to `score`.
fn accumulate(graph: &Graph, source: usize, score: &mut [f64]) {
    let n = graph.node_count();
    let (mut sigma, mut distance) = (vec![0.0f64; n], vec![usize::MAX; n]);
    let mut predecessors: Vec<Vec<u32>> = vec![Vec::new(); n];
    let mut order = Vec::with_capacity(n);
    let mut queue = VecDeque::from([source]);
    (sigma[source], distance[source]) = (1.0, 0);
    while let Some(v) = queue.pop_front() {
        order.push(v);
        for &w in graph.neighbours(v) {
            let w = w as usize;
            if distance[w] == usize::MAX {
                distance[w] = distance[v] + 1;
                queue.push_back(w);
            }
            if distance[w] == distance[v] + 1 {
                sigma[w] += sigma[v];
                predecessors[w].push(v as u32);
            }
        }
    }
    let mut delta = vec![0.0f64; n];
    for &w in order.iter().rev() {
        for &v in &predecessors[w] {
            let v = v as usize;
            delta[v] += sigma[v] / sigma[w] * (1.0 + delta[w]);
        }
        if w != source {
            score[w] += delta[w];
        }
    }
}
