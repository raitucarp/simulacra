//! The graph the Network view analyses, in one of three scopes: the whole knowledge graph,
//! thoughts only (replies, quotes, reposts and links between posts), or concepts (hashtags
//! joined when they appear on the same post).

use super::graph::layout::force_layout;
use super::graph::{Graph, NetworkMetrics};
use crate::domain::graph::{EdgeKind, GraphModel, NodeKind, NodeRef};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Which graph the Network view shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphScope {
    #[default]
    Knowledge,
    Thoughts,
    Concepts,
}

impl GraphScope {
    pub const ALL: [Self; 3] = [Self::Knowledge, Self::Thoughts, Self::Concepts];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Knowledge => "knowledge",
            Self::Thoughts => "thoughts",
            Self::Concepts => "concepts",
        }
    }
}

/// An analysed, laid-out graph.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetworkReport {
    pub scope: GraphScope,
    pub nodes: Vec<NodeRef>,
    pub labels: Vec<String>,
    pub graph: Graph,
    pub metrics: NetworkMetrics,
    /// Unit-square positions, index-aligned with `nodes`.
    pub positions: Vec<[f32; 2]>,
}

impl NetworkReport {
    /// Builds, measures and lays out the `scope` graph of `model`.
    pub fn of(model: &GraphModel, scope: GraphScope) -> Self {
        let (nodes, labels, graph) = match scope {
            GraphScope::Knowledge => whole(model),
            GraphScope::Thoughts => thoughts(model),
            GraphScope::Concepts => concepts(model),
        };
        let metrics = NetworkMetrics::compute(&graph);
        let positions = force_layout(&graph);
        Self { scope, nodes, labels, graph, metrics, positions }
    }

    pub fn kind(&self, node: usize) -> NodeKind {
        self.nodes[node].kind()
    }
}

type Parts = (Vec<NodeRef>, Vec<String>, Graph);

fn whole(model: &GraphModel) -> Parts {
    let nodes = model.nodes().iter().map(|n| n.node.clone()).collect();
    let labels = model.nodes().iter().map(|n| n.label.clone()).collect();
    (nodes, labels, Graph::new(model.node_count(), model.edges().iter().map(|e| (e.source, e.target))))
}

/// Keeps the nodes `keep` accepts, renumbering them, and the arcs between kept nodes.
fn induced(model: &GraphModel, keep: impl Fn(&NodeRef) -> bool, arcs: Vec<(usize, usize)>) -> Parts {
    let kept: Vec<usize> = (0..model.node_count()).filter(|&i| keep(&model.nodes()[i].node)).collect();
    let mut rename = vec![usize::MAX; model.node_count()];
    kept.iter().enumerate().for_each(|(new, &old)| rename[old] = new);
    let arcs = arcs.into_iter().filter(|&(a, b)| rename[a] != usize::MAX && rename[b] != usize::MAX).map(|(a, b)| (rename[a], rename[b]));
    let graph = Graph::new(kept.len(), arcs);
    let nodes = kept.iter().map(|&i| model.nodes()[i].node.clone()).collect();
    (nodes, kept.iter().map(|&i| model.nodes()[i].label.clone()).collect(), graph)
}

fn thoughts(model: &GraphModel) -> Parts {
    let arcs = model.edges().iter().filter(|e| e.kind != EdgeKind::Authored).map(|e| (e.source, e.target)).collect();
    induced(model, |node| node.kind() == NodeKind::Post, arcs)
}

/// Hashtag pairs sharing a post (each pair once per post).
fn concepts(model: &GraphModel) -> Parts {
    let mut tags_of_post: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    model.edges().iter().filter(|e| e.kind == EdgeKind::Hashtag).for_each(|e| {
        tags_of_post.entry(e.source).or_default().insert(e.target);
    });
    let arcs = tags_of_post.values().flat_map(pairs).collect();
    induced(model, |node| node.kind() == NodeKind::Hashtag, arcs)
}

/// Every unordered pair of `tags`.
fn pairs(tags: &BTreeSet<usize>) -> Vec<(usize, usize)> {
    let tags: Vec<usize> = tags.iter().copied().collect();
    tags.iter().enumerate().flat_map(|(i, &a)| tags[i + 1..].iter().map(move |&b| (a, b))).collect()
}
