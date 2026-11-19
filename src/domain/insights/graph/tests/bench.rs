//! Benchmark-style guards: the target size (5 000 nodes, 20 000 edges) must be analysed and
//! laid out well inside a generous budget, even in an unoptimised test build.

use super::super::layout::force_layout;
use super::super::metrics::NetworkMetrics;
use super::super::Graph;
use std::time::{Duration, Instant};

const NODES: usize = 5_000;
const EDGES: usize = 20_000;
/// Generous enough for a loaded machine and a debug build; a quadratic regression blows it.
const BUDGET: Duration = Duration::from_secs(40);
/// How much more time an unoptimised build (`opt-level = 0`, see build.rs) gets. Layout runs about
/// 15 times slower there, yet a quadratic regression at the target size still overshoots by far.
#[cfg(unoptimized)]
const UNOPTIMISED_SLOWDOWN: u32 = 8;
#[cfg(not(unoptimized))]
const UNOPTIMISED_SLOWDOWN: u32 = 1;

/// A deterministic pseudo-random graph: a reply tree (every node answers an earlier one) plus
/// extra links between nearby and far nodes, like posts, quotes and hashtags.
pub fn synthetic(nodes: usize, edges: usize) -> Graph {
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut next = move |bound: usize| {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state % bound.max(1) as u64) as usize
    };
    let tree: Vec<(usize, usize)> = (1..nodes).map(|i| (i, next(i))).collect();
    let extra: Vec<(usize, usize)> = (0..edges.saturating_sub(nodes)).map(|_| (next(nodes), next(nodes))).collect();
    Graph::new(nodes, tree.into_iter().chain(extra))
}

#[test]
fn target_size_metrics_and_layout_fit_the_budget() {
    let graph = synthetic(NODES, EDGES);
    assert!(graph.edge_count() > EDGES * 9 / 10);
    let started = Instant::now();
    let metrics = NetworkMetrics::compute(&graph);
    let analysed = started.elapsed();
    let positions = force_layout(&graph);
    let total = started.elapsed();
    assert_eq!(metrics.pagerank.len(), NODES);
    assert!(metrics.betweenness_sampled);
    assert_eq!(positions.len(), NODES);
    assert!(total < BUDGET * UNOPTIMISED_SLOWDOWN, "metrics {analysed:?}, metrics + layout {total:?}");
}
