//! Knowledge graph of one network: posts, grounding documents, hashtags and profiles,
//! connected by reply, quote, repost, wikilink, hashtag, mention and authorship edges.
//!
//! Construction is pure and deterministic (stable node and edge order), so the same
//! store contents always yield the same graph and, through [`layout`], the same picture.

mod build;
mod edges;
mod label;
pub mod layout;
mod resolve;
#[cfg(test)]
mod tests;

pub use build::GraphSource;
pub use label::excerpt;

use crate::domain::{PostId, ProfileId};
use std::collections::HashMap;

/// Category of a graph node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeKind {
    Post,
    Document,
    Hashtag,
    Profile,
}

/// Every node kind, in node order.
pub const NODE_KINDS: [NodeKind; 4] =
    [NodeKind::Post, NodeKind::Document, NodeKind::Hashtag, NodeKind::Profile];

/// Relation carried by a graph edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EdgeKind {
    Reply,
    Quote,
    Repost,
    Wikilink,
    Hashtag,
    Mention,
    Authored,
}

/// Identity of the entity behind a node.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeRef {
    Post(PostId),
    Document(String),
    /// Lower-cased tag without its `#` sigil.
    Hashtag(String),
    Profile(ProfileId),
}

impl NodeRef {
    /// Category of this node.
    pub fn kind(&self) -> NodeKind {
        match self {
            Self::Post(_) => NodeKind::Post,
            Self::Document(_) => NodeKind::Document,
            Self::Hashtag(_) => NodeKind::Hashtag,
            Self::Profile(_) => NodeKind::Profile,
        }
    }
}

/// A node with its display label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphNode {
    pub node: NodeRef,
    pub label: String,
}

/// An edge between two node indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GraphEdge {
    pub source: usize,
    pub target: usize,
    pub kind: EdgeKind,
}

/// The graph of one network with stable node and edge order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GraphModel {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
    index: HashMap<NodeRef, usize>,
}

impl GraphModel {
    pub fn nodes(&self) -> &[GraphNode] {
        &self.nodes
    }

    pub fn edges(&self) -> &[GraphEdge] {
        &self.edges
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Number of nodes of `kind`.
    pub fn count_of(&self, kind: NodeKind) -> usize {
        self.nodes.iter().filter(|n| n.node.kind() == kind).count()
    }

    /// Index of the node for `node`, if present.
    pub fn index_of(&self, node: &NodeRef) -> Option<usize> {
        self.index.get(node).copied()
    }

    /// Number of edges touching each node.
    pub fn degrees(&self) -> Vec<usize> {
        let mut degrees = vec![0; self.nodes.len()];
        for edge in &self.edges {
            degrees[edge.source] += 1;
            degrees[edge.target] += 1;
        }
        degrees
    }

    /// Whether nodes `a` and `b` share an edge.
    pub fn are_adjacent(&self, a: usize, b: usize) -> bool {
        self.edges
            .iter()
            .any(|e| (e.source == a && e.target == b) || (e.source == b && e.target == a))
    }
}
