//! What a map says about its argument, all pure and by node index:
//!
//! - **Unsupported claims:** claims no support edge reaches.
//! - **Circular support:** groups of nodes that support each other in a cycle (strongly
//!   connected components of the support graph), so the argument leans on itself.
//! - **Unanswered objections:** rebuttals, and any node attacking a claim, that nothing
//!   attacks (a counter-argument answering an objection is not itself an objection).
//! - **Standing:** the grounded labelling ([`grounded`] documents the semantics).

pub mod graph;
pub mod grounded;

pub use graph::Relations;
pub use grounded::{grounded_labelling, Standing};

use super::model::{ArgumentMap, NodeRole};

/// Everything the analysis found.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ArgumentReport {
    /// One standing per node, in display order.
    pub standing: Vec<Standing>,
    pub unsupported: Vec<usize>,
    /// Each cycle of mutual support, its nodes in display order.
    pub circular: Vec<Vec<usize>>,
    pub unanswered: Vec<usize>,
}

impl ArgumentReport {
    /// Claims that stand (accepted under the grounded semantics).
    pub fn standing_claims(&self, map: &ArgumentMap) -> usize {
        self.claims_with(map, Standing::Accepted)
    }

    /// Claims of `map` labelled `standing`.
    pub fn claims_with(&self, map: &ArgumentMap, standing: Standing) -> usize {
        map.nodes.iter().zip(&self.standing).filter(|(node, label)| node.role == NodeRole::Claim && **label == standing).count()
    }

    /// Whether node `index` takes part in circular support.
    pub fn is_circular(&self, index: usize) -> bool {
        self.circular.iter().any(|cycle| cycle.contains(&index))
    }
}

/// The full analysis of `map`.
pub fn analyse(map: &ArgumentMap) -> ArgumentReport {
    let relations = Relations::of(map);
    ArgumentReport { standing: grounded_labelling(&relations), unsupported: unsupported_claims(map, &relations), circular: circular_support(&relations), unanswered: unanswered_objections(map, &relations) }
}

/// Claims with no incoming support edge.
pub fn unsupported_claims(map: &ArgumentMap, relations: &Relations) -> Vec<usize> {
    let supported: Vec<bool> = (0..relations.len()).map(|n| relations.supports.iter().any(|targets| targets.contains(&n))).collect();
    map.nodes.iter().enumerate().filter(|(n, node)| node.role == NodeRole::Claim && !supported[*n]).map(|(n, _)| n).collect()
}

/// Strongly connected components (more than one node) of the support graph: two nodes are
/// in one component when each reaches the other by support. Quadratic, fine for a thread.
pub fn circular_support(relations: &Relations) -> Vec<Vec<usize>> {
    let reach: Vec<Vec<bool>> = (0..relations.len()).map(|n| relations.support_reach(n)).collect();
    let mut placed = vec![false; relations.len()];
    let mut cycles = Vec::new();
    for start in 0..relations.len() {
        if placed[start] || !reach[start][start] {
            continue;
        }
        let cycle: Vec<usize> = (0..relations.len()).filter(|n| *n == start || (reach[start][*n] && reach[*n][start])).collect();
        cycle.iter().for_each(|n| placed[*n] = true);
        cycles.push(cycle);
    }
    cycles
}

/// Objections nobody has answered: rebuttals and attackers of claims that no edge attacks.
pub fn unanswered_objections(map: &ArgumentMap, relations: &Relations) -> Vec<usize> {
    let attacked: Vec<bool> = (0..relations.len()).map(|n| relations.attacks.iter().any(|targets| targets.contains(&n))).collect();
    let objects = |n: usize| map.nodes[n].role == NodeRole::Rebuttal || relations.attacks[n].iter().any(|t| map.nodes[*t].role == NodeRole::Claim);
    (0..relations.len()).filter(|n| objects(*n) && !attacked[*n]).collect()
}
