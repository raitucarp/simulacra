//! Manual edits of a map, each a pure step from one valid map to the next: adding,
//! removing, relabelling and re-rolling nodes, adding, re-kinding and removing edges, and
//! moving a node in the display order. An edit that would break the map is refused.

use super::model::{ArgEdge, ArgNode, ArgumentMap, EdgeKind, NodeRole};
use crate::domain::PostId;

/// Longest node text kept, in characters.
pub const MAX_NODE_CHARS: usize = 280;

/// One change the user makes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapEdit {
    AddNode { role: NodeRole, text: String, source: PostId },
    RemoveNode { id: String },
    Relabel { id: String, text: String },
    SetRole { id: String, role: NodeRole },
    AddEdge { from: String, to: String, kind: EdgeKind },
    SetEdgeKind { from: String, to: String, kind: EdgeKind },
    RemoveEdge { from: String, to: String },
    /// Moves node `id` to position `to` of the display order (clamped).
    MoveNode { id: String, to: usize },
}

/// Why an edit was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditError {
    UnknownNode,
    UnknownEdge,
    BlankText,
    SelfLoop,
    DuplicateEdge,
}

/// `text` trimmed, whitespace collapsed and cut to [`MAX_NODE_CHARS`].
pub fn clean_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(MAX_NODE_CHARS).collect()
}

/// `map` after `edit`, or why it cannot be applied.
pub fn apply_edit(map: &ArgumentMap, edit: &MapEdit) -> Result<ArgumentMap, EditError> {
    match edit {
        MapEdit::AddNode { role, text, source } => add_node(map, *role, text, source),
        MapEdit::RemoveNode { id } => remove_node(map, id),
        MapEdit::Relabel { id, text } => {
            let text = Some(clean_text(text)).filter(|t| !t.is_empty()).ok_or(EditError::BlankText)?;
            with_node(map, id, |node| ArgNode { text: text.clone(), ..node.clone() })
        }
        MapEdit::SetRole { id, role } => with_node(map, id, |node| ArgNode { role: *role, ..node.clone() }),
        MapEdit::AddEdge { from, to, kind } => add_edge(map, from, to, *kind),
        MapEdit::SetEdgeKind { from, to, kind } => with_edge(map, from, to, |edge| ArgEdge { kind: *kind, ..edge.clone() }),
        MapEdit::RemoveEdge { from, to } => {
            map.has_edge(from, to).then_some(()).ok_or(EditError::UnknownEdge)?;
            let edges = map.edges.iter().filter(|edge| !(edge.from == *from && edge.to == *to)).cloned().collect();
            Ok(ArgumentMap { edges, ..map.clone() })
        }
        MapEdit::MoveNode { id, to } => move_node(map, id, *to),
    }
}

fn add_node(map: &ArgumentMap, role: NodeRole, text: &str, source: &PostId) -> Result<ArgumentMap, EditError> {
    let text = Some(clean_text(text)).filter(|t| !t.is_empty()).ok_or(EditError::BlankText)?;
    let node = ArgNode { id: map.next_node_id(), role, text, source: source.clone() };
    let nodes = map.nodes.iter().cloned().chain(std::iter::once(node)).collect();
    Ok(ArgumentMap { nodes, ..map.clone() })
}

/// The node and every edge touching it are removed.
fn remove_node(map: &ArgumentMap, id: &str) -> Result<ArgumentMap, EditError> {
    map.index_of(id).ok_or(EditError::UnknownNode)?;
    let nodes = map.nodes.iter().filter(|node| node.id != id).cloned().collect();
    let edges = map.edges.iter().filter(|edge| edge.from != id && edge.to != id).cloned().collect();
    Ok(ArgumentMap { nodes, edges, root: map.root.clone() })
}

fn with_node(map: &ArgumentMap, id: &str, change: impl Fn(&ArgNode) -> ArgNode) -> Result<ArgumentMap, EditError> {
    map.index_of(id).ok_or(EditError::UnknownNode)?;
    let nodes = map.nodes.iter().map(|node| if node.id == id { change(node) } else { node.clone() }).collect();
    Ok(ArgumentMap { nodes, ..map.clone() })
}

fn with_edge(map: &ArgumentMap, from: &str, to: &str, change: impl Fn(&ArgEdge) -> ArgEdge) -> Result<ArgumentMap, EditError> {
    map.has_edge(from, to).then_some(()).ok_or(EditError::UnknownEdge)?;
    let edges = map.edges.iter().map(|edge| if edge.from == from && edge.to == to { change(edge) } else { edge.clone() }).collect();
    Ok(ArgumentMap { edges, ..map.clone() })
}

/// Edges join two different existing nodes, at most once per direction.
fn add_edge(map: &ArgumentMap, from: &str, to: &str, kind: EdgeKind) -> Result<ArgumentMap, EditError> {
    if map.node(from).is_none() || map.node(to).is_none() {
        return Err(EditError::UnknownNode);
    }
    if from == to {
        return Err(EditError::SelfLoop);
    }
    if map.has_edge(from, to) {
        return Err(EditError::DuplicateEdge);
    }
    let edge = ArgEdge { from: from.to_string(), to: to.to_string(), kind };
    let edges = map.edges.iter().cloned().chain(std::iter::once(edge)).collect();
    Ok(ArgumentMap { edges, ..map.clone() })
}

fn move_node(map: &ArgumentMap, id: &str, to: usize) -> Result<ArgumentMap, EditError> {
    let from = map.index_of(id).ok_or(EditError::UnknownNode)?;
    let mut nodes = map.nodes.clone();
    let node = nodes.remove(from);
    nodes.insert(to.min(nodes.len()), node);
    Ok(ArgumentMap { nodes, ..map.clone() })
}
