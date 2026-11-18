//! The argument graph: nodes in Toulmin's roles, joined by support and attack edges (a
//! bipolar argumentation framework). Every node cites the post it was read from, and the
//! node order is the display order the user arranges.

use crate::domain::document::keyed::keyed_enum;
use crate::domain::PostId;
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// What a node does in the argument (Toulmin's layout).
    NodeRole {
        /// What the author wants the reader to accept.
        Claim => "claim",
        /// The facts or reasons offered for a claim (Toulmin's "data").
        Ground => "ground",
        /// The general rule that licenses the step from grounds to claim.
        Warrant => "warrant",
        /// Why the warrant itself should be trusted.
        Backing => "backing",
        /// How strongly the claim is put ("probably", "in most cases").
        Qualifier => "qualifier",
        /// A circumstance or objection that would defeat the claim.
        Rebuttal => "rebuttal",
    }
}

keyed_enum! {
    /// How one node bears on another.
    EdgeKind {
        /// The source gives a reason to accept the target.
        Support => "support",
        /// The source gives a reason to reject the target.
        Attack => "attack",
    }
}

/// Prefix of generated node ids (`n1`, `n2`, …).
pub const NODE_ID_PREFIX: &str = "n";

/// One statement of the argument.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgNode {
    pub id: String,
    pub role: NodeRole,
    pub text: String,
    /// The post the statement comes from.
    pub source: PostId,
}

/// `from` supports or attacks `to`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgEdge {
    pub from: String,
    pub to: String,
    pub kind: EdgeKind,
}

/// The argument of one thread, keyed by its root post.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgumentMap {
    pub root: PostId,
    pub nodes: Vec<ArgNode>,
    pub edges: Vec<ArgEdge>,
}

impl ArgumentMap {
    /// An empty map of the thread rooted at `root`.
    pub fn empty(root: PostId) -> Self {
        Self { root, nodes: Vec::new(), edges: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Position of node `id` in display order.
    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.nodes.iter().position(|node| node.id == id)
    }

    pub fn node(&self, id: &str) -> Option<&ArgNode> {
        self.nodes.iter().find(|node| node.id == id)
    }

    /// Whether an edge joins `from` to `to` (of any kind).
    pub fn has_edge(&self, from: &str, to: &str) -> bool {
        self.edges.iter().any(|edge| edge.from == from && edge.to == to)
    }

    /// Edges as `(from index, to index, kind)`; edges naming a missing node are skipped.
    pub fn indexed_edges(&self) -> Vec<(usize, usize, EdgeKind)> {
        self.edges.iter().filter_map(|edge| Some((self.index_of(&edge.from)?, self.index_of(&edge.to)?, edge.kind))).collect()
    }

    /// A fresh node id, one past the largest numbered id in use.
    pub fn next_node_id(&self) -> String {
        let largest = self.nodes.iter().filter_map(|node| node.id.strip_prefix(NODE_ID_PREFIX)?.parse::<u32>().ok()).max().unwrap_or(0);
        format!("{NODE_ID_PREFIX}{}", largest + 1)
    }

    /// The map as stored (JSON).
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// A stored map, or `None` when the row is unreadable.
    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}
