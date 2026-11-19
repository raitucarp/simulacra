mod bench;
mod metrics;

use super::Graph;

/// Undirected path 0 – 1 – … – (n − 1).
pub(super) fn path(n: usize) -> Graph {
    Graph::new(n, (1..n).map(|i| (i, i - 1)))
}

/// Two triangles {0,1,2} and {3,4,5} joined by the bridge 2 – 3.
pub(super) fn barbell() -> Graph {
    Graph::new(6, [(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3), (2, 3)])
}

/// A star: every leaf replies to the hub 0.
pub(super) fn star(leaves: usize) -> Graph {
    Graph::new(leaves + 1, (1..=leaves).map(|leaf| (leaf, 0)))
}

#[test]
fn csr_dedups_and_drops_loops_and_strays() {
    let g = Graph::new(3, [(0, 1), (0, 1), (1, 0), (1, 1), (2, 7)]);
    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 1);
    assert_eq!(g.arc_count(), 2);
    assert_eq!(g.neighbours(0), &[1]);
    assert_eq!((g.out_degree(0), g.in_degree(0)), (1, 1));
    assert_eq!(g.edges().collect::<Vec<_>>(), vec![(0, 1)]);
}

#[test]
fn degree_and_density() {
    use super::degree::{degrees, density, mean_degree};
    let g = star(4);
    let d = degrees(&g);
    assert_eq!((d[0].total, d[0].incoming, d[0].outgoing), (4, 4, 0));
    assert_eq!((d[1].incoming, d[1].outgoing), (0, 1));
    assert!((density(&g) - 0.4).abs() < 1e-12);
    assert!((mean_degree(&g) - 1.6).abs() < 1e-12);
    assert_eq!(density(&Graph::new(1, [])), 0.0);
}

#[test]
fn pagerank_flows_to_the_hub_and_sums_to_one() {
    let rank = super::pagerank::pagerank(&star(5));
    assert!((rank.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    assert!(rank[0] > rank[1] * 3.0);
    assert!(super::pagerank::pagerank(&Graph::default()).is_empty());
}

#[test]
fn shortest_paths_and_diameter() {
    use super::paths::{diameter_estimate, distances, shortest_path};
    let g = path(5);
    assert_eq!(shortest_path(&g, 0, 4), Some(vec![0, 1, 2, 3, 4]));
    assert_eq!(shortest_path(&g, 2, 2), Some(vec![2]));
    assert_eq!(distances(&g, 0)[4], Some(4));
    assert_eq!(diameter_estimate(&g, 2), 4);
    let split = Graph::new(3, [(0, 1)]);
    assert_eq!(shortest_path(&split, 0, 2), None);
    assert_eq!(distances(&split, 0)[2], None);
}

#[test]
fn layout_is_deterministic_and_in_the_unit_square() {
    let g = barbell();
    let (a, b) = (super::layout::force_layout(&g), super::layout::force_layout(&g));
    assert_eq!(a, b);
    assert!(a.iter().all(|p| (0.0..=1.0).contains(&p[0]) && (0.0..=1.0).contains(&p[1])));
    let gap = |i: usize, j: usize| ((a[i][0] - a[j][0]).powi(2) + (a[i][1] - a[j][1]).powi(2)).sqrt();
    assert!(gap(0, 1) < gap(0, 5), "a triangle sits closer together than the far end of the barbell");
    assert_eq!(super::layout::force_layout(&Graph::new(1, [])), vec![[0.5, 0.5]]);
}
