//! The automorphism group of a rooted forest (reply threads), computed exactly.
//!
//! Subtrees get canonical classes bottom-up (a class is the sorted multiset of its children's
//! classes), so two subtrees are isomorphic iff their classes are equal. Children of one class
//! can be permuted freely, which multiplies the group order by `m!`; the orbits follow the
//! classes along the path from the root. Breadth-first order keeps it iterative: O(n log n).

use std::collections::HashMap;

/// No parent: a root.
const ROOT: u32 = u32::MAX;

/// Symmetry of a forest given by each node's parent.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreeSymmetry {
    /// Isomorphism class of the subtree under each node.
    pub classes: Vec<u32>,
    /// Orbit of each node under the automorphism group.
    pub orbits: Vec<u32>,
    /// The group order as a product of factorials: `|Aut| = Π f!` over these `f` (each ≥ 2).
    pub factorials: Vec<u32>,
    /// `|Aut|` when it fits in 128 bits.
    pub order: Option<u128>,
    /// `log10 |Aut|` (always finite).
    pub log10_order: f64,
    /// Longest root-to-leaf path, in edges.
    pub depth: usize,
}

impl TreeSymmetry {
    /// Groups of two or more interchangeable nodes, largest first.
    pub fn nontrivial_orbits(&self) -> Vec<Vec<usize>> {
        super::refinement::equivalence_classes(&self.orbits)
    }
}

fn factorial(k: u32) -> Option<u128> {
    (2..=u128::from(k)).try_fold(1u128, |acc, f| acc.checked_mul(f))
}

fn log10_factorial(k: u32) -> f64 {
    (2..=k).map(|f| f64::from(f).log10()).sum()
}

/// Nodes in breadth-first order from every root, and each node's depth.
fn breadth_first(parents: &[Option<usize>], children: &[Vec<usize>]) -> (Vec<usize>, Vec<usize>) {
    let mut order: Vec<usize> = (0..parents.len()).filter(|&v| parents[v].is_none()).collect();
    let mut depth = vec![0usize; parents.len()];
    let mut head = 0;
    while let Some(&v) = order.get(head) {
        head += 1;
        for &c in &children[v] {
            depth[c] = depth[v] + 1;
            order.push(c);
        }
    }
    (order, depth)
}

/// Computes the symmetry of the forest where `parents[v]` is `v`'s parent. Nodes on a cycle
/// of parents (not a forest) are ignored.
pub fn tree_symmetry(parents: &[Option<usize>]) -> TreeSymmetry {
    let n = parents.len();
    let mut children = vec![Vec::new(); n];
    parents.iter().enumerate().filter_map(|(v, p)| p.filter(|&p| p < n).map(|p| (p, v))).for_each(|(p, v)| children[p].push(v));
    let (order, depth) = breadth_first(parents, &children);
    let (mut classes, mut signatures) = (vec![0u32; n], HashMap::<Vec<u32>, u32>::new());
    let mut factorials = Vec::new();
    for &v in order.iter().rev() {
        let mut signature: Vec<u32> = children[v].iter().map(|&c| classes[c]).collect();
        signature.sort_unstable();
        signature.chunk_by(|a, b| a == b).filter(|run| run.len() > 1).for_each(|run| factorials.push(run.len() as u32));
        let next = signatures.len() as u32;
        classes[v] = *signatures.entry(signature).or_insert(next);
    }
    let mut orbit_ids: HashMap<(u32, u32), u32> = HashMap::new();
    let mut orbits = vec![0u32; n];
    for &v in &order {
        let parent_orbit = parents[v].map_or(ROOT, |p| orbits[p]);
        let next = orbit_ids.len() as u32;
        orbits[v] = *orbit_ids.entry((parent_orbit, classes[v])).or_insert(next);
    }
    factorials.sort_unstable();
    let order_value = factorials.iter().try_fold(1u128, |acc, &f| factorial(f).and_then(|x| acc.checked_mul(x)));
    let log10_order = factorials.iter().map(|&f| log10_factorial(f)).sum();
    TreeSymmetry { classes, orbits, factorials, order: order_value, log10_order, depth: depth.into_iter().max().unwrap_or(0) }
}
