//! Bridges (edges whose removal disconnects their component) and articulation points (nodes
//! whose removal does) by Tarjan's low-link method. The depth-first search keeps an explicit
//! stack, so a long reply chain cannot overflow the call stack: O(n + m).

use super::Graph;

/// Cut edges and cut nodes of the graph.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CutStructure {
    /// Each bridge as `(low, high)`, ascending.
    pub bridges: Vec<(usize, usize)>,
    /// Articulation points, ascending.
    pub articulation_points: Vec<usize>,
}

/// One frame of the explicit depth-first stack: node, parent, next neighbour to visit.
struct Frame {
    node: usize,
    parent: usize,
    next: usize,
}

/// Search state shared by every tree of the forest.
struct Search<'g> {
    graph: &'g Graph,
    discovered: Vec<usize>,
    low: Vec<usize>,
    clock: usize,
    is_cut: Vec<bool>,
    bridges: Vec<(usize, usize)>,
}

impl Search<'_> {
    fn visit(&mut self, node: usize) {
        self.clock += 1;
        (self.discovered[node], self.low[node]) = (self.clock, self.clock);
    }

    /// Runs one depth-first tree from `root`.
    fn tree(&mut self, root: usize) {
        self.visit(root);
        let mut stack = vec![Frame { node: root, parent: usize::MAX, next: 0 }];
        let mut root_children = 0usize;
        while let Some(frame) = stack.last_mut() {
            let (v, parent) = (frame.node, frame.parent);
            if let Some(&w) = self.graph.neighbours(v).get(frame.next) {
                frame.next += 1;
                let w = w as usize;
                if self.discovered[w] == 0 {
                    self.visit(w);
                    root_children += usize::from(v == root);
                    stack.push(Frame { node: w, parent: v, next: 0 });
                } else if w != parent {
                    self.low[v] = self.low[v].min(self.discovered[w]);
                }
                continue;
            }
            stack.pop();
            if parent != usize::MAX {
                self.finish_child(parent, v, root);
            }
        }
        self.is_cut[root] = root_children > 1;
    }

    /// Propagates `child`'s low link to `parent` and records a bridge or cut node.
    fn finish_child(&mut self, parent: usize, child: usize, root: usize) {
        self.low[parent] = self.low[parent].min(self.low[child]);
        if self.low[child] > self.discovered[parent] {
            self.bridges.push((parent.min(child), parent.max(child)));
        }
        if parent != root && self.low[child] >= self.discovered[parent] {
            self.is_cut[parent] = true;
        }
    }
}

/// Finds every bridge and articulation point.
pub fn cut_structure(graph: &Graph) -> CutStructure {
    let n = graph.node_count();
    let mut search = Search { graph, discovered: vec![0; n], low: vec![0; n], clock: 0, is_cut: vec![false; n], bridges: Vec::new() };
    for root in 0..n {
        if search.discovered[root] == 0 {
            search.tree(root);
        }
    }
    let mut bridges = search.bridges;
    bridges.sort_unstable();
    let articulation_points = search.is_cut.iter().enumerate().filter(|(_, cut)| **cut).map(|(v, _)| v).collect();
    CutStructure { bridges, articulation_points }
}
