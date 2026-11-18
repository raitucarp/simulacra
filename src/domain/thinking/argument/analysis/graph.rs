//! The map as index-based relations: who supports and who attacks whom, reachability over
//! support edges, and the attack relation the acceptability semantics use.

use super::super::model::{ArgumentMap, EdgeKind};

/// Support and attack successors of every node, by node index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relations {
    pub supports: Vec<Vec<usize>>,
    pub attacks: Vec<Vec<usize>>,
}

impl Relations {
    pub fn of(map: &ArgumentMap) -> Self {
        let empty = vec![Vec::new(); map.nodes.len()];
        let (mut supports, mut attacks) = (empty.clone(), empty);
        for (from, to, kind) in map.indexed_edges() {
            match kind {
                EdgeKind::Support => supports[from].push(to),
                EdgeKind::Attack => attacks[from].push(to),
            }
        }
        Self { supports, attacks }
    }

    pub fn len(&self) -> usize {
        self.supports.len()
    }

    /// Nodes reachable from `start` by one or more support edges.
    pub fn support_reach(&self, start: usize) -> Vec<bool> {
        let mut seen = vec![false; self.len()];
        let mut stack: Vec<usize> = self.supports[start].clone();
        while let Some(node) = stack.pop() {
            if !std::mem::replace(&mut seen[node], true) {
                stack.extend(self.supports[node].iter().copied());
            }
        }
        seen
    }

    /// The attack relation of the semantics: direct attacks plus supported attacks (a node
    /// attacks whatever a node it reaches by support attacks).
    pub fn effective_attacks(&self) -> Vec<Vec<usize>> {
        (0..self.len())
            .map(|from| {
                let reach = self.support_reach(from);
                let mut targets: Vec<usize> = std::iter::once(from)
                    .chain((0..self.len()).filter(|n| reach[*n]))
                    .flat_map(|n| self.attacks[n].iter().copied())
                    .collect();
                targets.sort_unstable();
                targets.dedup();
                targets
            })
            .collect()
    }

    /// Who attacks each node, under `attacks`.
    pub fn attackers(attacks: &[Vec<usize>]) -> Vec<Vec<usize>> {
        let mut attackers = vec![Vec::new(); attacks.len()];
        for (from, targets) in attacks.iter().enumerate() {
            targets.iter().for_each(|to| attackers[*to].push(from));
        }
        attackers
    }
}
