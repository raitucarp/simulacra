//! Colour refinement (1-dimensional Weisfeiler–Leman): repeatedly recolour every node by its
//! colour and the multiset of its neighbours' colours until the partition stops splitting.
//! At most `n` rounds (capped at [`MAX_ROUNDS`]), each O(m log Δ); colours are renumbered by
//! sorted signature every round, so the result does not depend on node order.

use crate::domain::insights::graph::Graph;
use std::collections::BTreeMap;

/// Upper bound of refinement rounds (the partition is stable long before on real threads).
pub const MAX_ROUNDS: usize = 64;

/// The stable colour of every node, starting from `initial` (for instance the node kind).
pub fn colour_refinement(graph: &Graph, initial: &[u32]) -> Vec<u32> {
    let n = graph.node_count();
    let mut colours: Vec<u32> = (0..n).map(|v| initial.get(v).copied().unwrap_or_default()).collect();
    let mut classes = distinct(&colours);
    for _ in 0..MAX_ROUNDS.min(n.max(1)) {
        let signatures: Vec<(u32, Vec<u32>)> = (0..n)
            .map(|v| {
                let mut around: Vec<u32> = graph.neighbours(v).iter().map(|&w| colours[w as usize]).collect();
                around.sort_unstable();
                (colours[v], around)
            })
            .collect();
        let ids: BTreeMap<&(u32, Vec<u32>), u32> = {
            let mut sorted: Vec<&(u32, Vec<u32>)> = signatures.iter().collect();
            sorted.sort();
            sorted.dedup();
            sorted.into_iter().enumerate().map(|(i, s)| (s, i as u32)).collect()
        };
        colours = signatures.iter().map(|s| ids[s]).collect();
        let refined = ids.len();
        if refined == classes {
            break;
        }
        classes = refined;
    }
    colours
}

fn distinct(colours: &[u32]) -> usize {
    let mut sorted = colours.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    sorted.len()
}

/// Groups of two or more nodes sharing a colour, largest first (ties by first node).
pub fn equivalence_classes(colours: &[u32]) -> Vec<Vec<usize>> {
    let mut groups: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
    colours.iter().enumerate().for_each(|(v, &c)| groups.entry(c).or_default().push(v));
    let mut classes: Vec<Vec<usize>> = groups.into_values().filter(|g| g.len() > 1).collect();
    classes.sort_by_key(|g| (std::cmp::Reverse(g.len()), g[0]));
    classes
}
