use super::super::betweenness::betweenness;
use super::super::bridges::cut_structure;
use super::super::community::communities;
use super::super::components::components;
use super::super::metrics::{Measure, NetworkMetrics};
use super::super::triangles::{average_clustering, clustering, triangles};
use super::super::Graph;
use super::{barbell, path, star};

#[test]
fn betweenness_peaks_in_the_middle_of_a_path() {
    let (score, sampled) = betweenness(&path(5));
    assert!(!sampled);
    // Node 2 lies on 4 of the 6 pairs that do not include it.
    assert!((score[2] - 4.0 / 6.0).abs() < 1e-9, "{score:?}");
    assert_eq!(score[0], 0.0);
    assert!(score[1] < score[2]);
}

#[test]
fn betweenness_of_a_star_hub_is_one() {
    let (score, _) = betweenness(&star(6));
    assert!((score[0] - 1.0).abs() < 1e-9);
    assert!(score[1..].iter().all(|&s| s == 0.0));
}

#[test]
fn large_graphs_sample_betweenness() {
    let (_, sampled) = betweenness(&path(400));
    assert!(sampled);
}

#[test]
fn triangles_and_clustering() {
    let g = barbell();
    assert_eq!(triangles(&g), vec![1, 1, 1, 1, 1, 1]);
    let local = clustering(&g);
    assert_eq!(local[0], 1.0);
    assert!((local[2] - 1.0 / 3.0).abs() < 1e-12);
    let mean = average_clustering(&g, &local);
    assert!((mean - (4.0 + 2.0 / 3.0) / 6.0).abs() < 1e-12);
    let k4 = Graph::new(4, [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    assert_eq!(triangles(&k4), vec![3; 4]);
}

#[test]
fn components_are_numbered_by_size() {
    let g = Graph::new(6, [(4, 5), (0, 1), (1, 2)]);
    let c = components(&g);
    assert_eq!(c.sizes, vec![3, 2, 1]);
    assert_eq!(c.of_node, vec![0, 0, 0, 2, 1, 1]);
    assert!((c.giant_share() - 0.5).abs() < 1e-12);
}

#[test]
fn communities_split_the_barbell() {
    let found = communities(&barbell());
    assert_eq!(found.count(), 2, "{found:?}");
    assert_eq!(found.of_node[0], found.of_node[1]);
    assert_ne!(found.of_node[0], found.of_node[5]);
    assert!(found.modularity > 0.3, "{}", found.modularity);
    assert_eq!(communities(&Graph::new(3, [])).modularity, 0.0);
}

#[test]
fn bridges_and_articulation_points() {
    let cuts = cut_structure(&barbell());
    assert_eq!(cuts.bridges, vec![(2, 3)]);
    assert_eq!(cuts.articulation_points, vec![2, 3]);
    let line = cut_structure(&path(4));
    assert_eq!(line.bridges.len(), 3);
    assert_eq!(line.articulation_points, vec![1, 2]);
    let deep = cut_structure(&path(50_000));
    assert_eq!(deep.bridges.len(), 49_999, "an explicit stack survives a long chain");
}

#[test]
fn metrics_bundle_and_rankings() {
    let m = NetworkMetrics::compute(&star(4));
    assert_eq!(m.top(Measure::Degree, 1, |_| true), vec![(0, 4.0)]);
    assert_eq!(m.top(Measure::Influence, 2, |v| v != 0).len(), 2);
    assert_eq!(m.diameter, 2);
    assert_eq!(m.cuts.articulation_points, vec![0]);
    assert!((m.max(Measure::Betweenness) - 1.0).abs() < 1e-9);
    assert_eq!(Measure::ALL.map(Measure::key), ["degree", "influence", "betweenness", "clustering"]);
}
