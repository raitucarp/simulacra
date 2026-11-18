use super::kmeans::{cluster_count, kmeans};
use super::EmbeddingMap;
use crate::domain::embedding::l2_normalized;

/// Two tight groups of unit vectors around the x and y axes.
fn two_groups() -> Vec<(String, Vec<f32>)> {
    let around = |axis: usize, wobble: f32| {
        let mut v = vec![wobble, wobble * 0.5, 0.0];
        v[axis] = 1.0;
        l2_normalized(v)
    };
    (0..6)
        .map(|i| (format!("p{i}"), around(i % 2, i as f32 * 0.01)))
        .collect()
}

#[test]
fn kmeans_separates_two_groups_deterministically() {
    let vectors: Vec<Vec<f32>> = two_groups().into_iter().map(|(_, v)| v).collect();
    let a = kmeans(&vectors, 2);
    assert_eq!(a, kmeans(&vectors, 2));
    assert!(a.iter().step_by(2).all(|&c| c == a[0]));
    assert!(a.iter().skip(1).step_by(2).all(|&c| c == a[1]));
    assert_ne!(a[0], a[1]);
    assert_eq!(kmeans(&vectors, 1), vec![0; 6]);
    assert!(kmeans(&[], 3).is_empty());
}

#[test]
fn cluster_count_grows_slowly() {
    assert_eq!(cluster_count(2), 1);
    assert_eq!(cluster_count(8), 2);
    assert_eq!(cluster_count(50), 5);
    assert_eq!(cluster_count(10_000), 8);
}

#[test]
fn map_projects_into_the_unit_square_and_finds_neighbours() {
    let map = EmbeddingMap::build(two_groups());
    assert_eq!((map.len(), map.is_empty()), (6, false));
    assert!(map.points.iter().all(|p| (0.0..=1.0).contains(&p[0]) && (0.0..=1.0).contains(&p[1])));
    let near = map.neighbours(0, 2);
    assert_eq!(near.len(), 2);
    assert!(near.iter().all(|(i, _)| i % 2 == 0), "neighbours of an x-axis post lie on the x axis: {near:?}");
    assert!(near[0].1 >= near[1].1);
    assert!(map.neighbours(99, 2).is_empty());
    let members = map.cluster_members();
    assert_eq!(members.iter().map(Vec::len).sum::<usize>(), 6);
}
