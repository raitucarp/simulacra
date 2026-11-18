use super::*;
use crate::domain::insights::graph::Graph;

#[test]
fn a_cherry_has_two_automorphisms() {
    // 0 is the root with two leaf replies.
    let s = tree_symmetry(&[None, Some(0), Some(0)]);
    assert_eq!(s.order, Some(2));
    assert_eq!(s.factorials, vec![2]);
    assert_eq!(s.orbits[1], s.orbits[2]);
    assert_ne!(s.orbits[0], s.orbits[1]);
    assert_eq!(s.nontrivial_orbits(), vec![vec![1, 2]]);
    assert_eq!(s.depth, 1);
}

#[test]
fn nested_symmetry_multiplies() {
    // Root 0 with two identical cherries (1: 3,4) and (2: 5,6), plus a lone leaf 7 under 0.
    let parents = [None, Some(0), Some(0), Some(1), Some(1), Some(2), Some(2), Some(0)];
    let s = tree_symmetry(&parents);
    // 2! (swap leaves under 1) · 2! (under 2) · 2! (swap the cherries) = 8.
    assert_eq!(s.order, Some(8));
    assert!((s.log10_order - 8f64.log10()).abs() < 1e-12);
    assert_eq!(s.orbits[3], s.orbits[6], "leaves of both cherries are interchangeable");
    assert_ne!(s.orbits[7], s.orbits[1], "a leaf is never a cherry");
    assert_eq!(s.classes[1], s.classes[2]);
}

#[test]
fn a_chain_is_rigid_and_huge_stars_stay_finite() {
    let chain = tree_symmetry(&[None, Some(0), Some(1), Some(2)]);
    assert_eq!((chain.order, chain.depth), (Some(1), 3));
    assert!(chain.nontrivial_orbits().is_empty());
    let mut star = vec![None];
    star.extend((0..60).map(|_| Some(0)));
    let big = tree_symmetry(&star);
    assert_eq!(big.order, None, "60! overflows 128 bits");
    assert!((big.log10_order - 81.919).abs() < 0.01);
}

#[test]
fn refinement_separates_ends_from_middles() {
    let path = Graph::new(5, [(1, 0), (2, 1), (3, 2), (4, 3)]);
    let colours = colour_refinement(&path, &[0; 5]);
    assert_eq!(colours[0], colours[4]);
    assert_eq!(colours[1], colours[3]);
    assert_ne!(colours[0], colours[1]);
    assert_ne!(colours[1], colours[2]);
    assert_eq!(equivalence_classes(&colours), vec![vec![0, 4], vec![1, 3]]);
}

#[test]
fn refinement_respects_initial_kinds() {
    let pair = Graph::new(2, [(0, 1)]);
    assert_ne!(colour_refinement(&pair, &[0, 1])[0], colour_refinement(&pair, &[0, 1])[1]);
    assert_eq!(colour_refinement(&pair, &[0, 0]), vec![0, 0]);
}

#[test]
fn rotation_finds_cycles_order_and_sign() {
    // 0 ↔ 1 duel, 2 → 3 → 4 → 2 round, 5 answers 0 (transient).
    let replies = [(0, 1), (1, 0), (0, 1), (2, 3), (3, 4), (4, 2), (5, 0)];
    let r = rotation(&replies);
    assert_eq!(r.cycles, vec![vec![0, 1], vec![2, 3, 4]]);
    assert_eq!(r.order, 6);
    assert!(!r.even, "a transposition and a 3-cycle are odd");
    assert_eq!(r.transient, vec![5]);
}

#[test]
fn rotation_breaks_ties_and_handles_self_answers() {
    let r = rotation(&[(0, 2), (0, 1), (1, 0), (2, 2)]);
    assert_eq!(r.cycles, vec![vec![0, 1], vec![2]]);
    assert_eq!(r.order, 2);
    assert_eq!(rotation(&[]), Rotation { order: 1, even: true, ..Rotation::default() });
}
