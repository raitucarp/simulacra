//! Local clustering coefficients by exact triangle counting.
//!
//! Edges are oriented from the lower to the higher (degree, index) rank, so each triangle is
//! found once from its lowest corner and hubs are never scanned from both ends: O(m · √m).
//! The local coefficient of `v` is `2 T(v) / (d (d − 1))`; nodes of degree below two get 0.

use super::Graph;

/// Triangles through every node.
pub fn triangles(graph: &Graph) -> Vec<usize> {
    let n = graph.node_count();
    let rank = |v: usize| (graph.degree(v), v);
    let forward: Vec<Vec<u32>> = (0..n)
        .map(|v| graph.neighbours(v).iter().copied().filter(|&w| rank(w as usize) > rank(v)).collect())
        .collect();
    let mut count = vec![0usize; n];
    let mut mark = vec![u32::MAX; n];
    for u in 0..n {
        forward[u].iter().for_each(|&w| mark[w as usize] = u as u32);
        for &v in &forward[u] {
            for &w in &forward[v as usize] {
                if mark[w as usize] == u as u32 {
                    count[u] += 1;
                    count[v as usize] += 1;
                    count[w as usize] += 1;
                }
            }
        }
    }
    count
}

/// Local clustering coefficient of every node.
pub fn clustering(graph: &Graph) -> Vec<f64> {
    triangles(graph)
        .into_iter()
        .enumerate()
        .map(|(v, t)| {
            let d = graph.degree(v) as f64;
            if d < 2.0 { 0.0 } else { 2.0 * t as f64 / (d * (d - 1.0)) }
        })
        .collect()
}

/// Mean local coefficient over nodes of degree two or more (0 when there are none).
pub fn average_clustering(graph: &Graph, local: &[f64]) -> f64 {
    let eligible: Vec<f64> = local.iter().enumerate().filter(|(v, _)| graph.degree(*v) >= 2).map(|(_, c)| *c).collect();
    if eligible.is_empty() {
        return 0.0;
    }
    eligible.iter().sum::<f64>() / eligible.len() as f64
}
