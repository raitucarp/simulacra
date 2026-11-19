//! Connected components of the undirected graph by breadth-first search: O(n + m).

use super::Graph;

/// Component of every node, numbered by size (largest first, ties by first node).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Components {
    pub of_node: Vec<usize>,
    /// Node count of each component, descending.
    pub sizes: Vec<usize>,
}

impl Components {
    pub fn count(&self) -> usize {
        self.sizes.len()
    }

    /// Share of the nodes in the largest component.
    pub fn giant_share(&self) -> f64 {
        let total: usize = self.sizes.iter().sum();
        self.sizes.first().map_or(0.0, |&largest| largest as f64 / total.max(1) as f64)
    }
}

/// Labels every node with its component.
pub fn components(graph: &Graph) -> Components {
    let n = graph.node_count();
    let mut raw = vec![usize::MAX; n];
    let mut sizes: Vec<usize> = Vec::new();
    for start in 0..n {
        if raw[start] != usize::MAX {
            continue;
        }
        let id = sizes.len();
        let (mut stack, mut size) = (vec![start], 0usize);
        raw[start] = id;
        while let Some(v) = stack.pop() {
            size += 1;
            for &w in graph.neighbours(v) {
                if raw[w as usize] == usize::MAX {
                    raw[w as usize] = id;
                    stack.push(w as usize);
                }
            }
        }
        sizes.push(size);
    }
    let mut order: Vec<usize> = (0..sizes.len()).collect();
    order.sort_by_key(|&c| (std::cmp::Reverse(sizes[c]), c));
    let mut rename = vec![0usize; sizes.len()];
    order.iter().enumerate().for_each(|(new, &old)| rename[old] = new);
    Components { of_node: raw.into_iter().map(|c| rename[c]).collect(), sizes: order.iter().map(|&c| sizes[c]).collect() }
}
