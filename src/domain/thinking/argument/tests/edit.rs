//! Manual edits: every step keeps the map valid, invalid steps are refused.

use super::super::edit::{apply_edit, EditError, MapEdit, MAX_NODE_CHARS};
use super::super::model::{EdgeKind, NodeRole};
use super::map;
use crate::domain::PostId;

fn edit(m: &super::ArgumentMap, edit: MapEdit) -> super::ArgumentMap {
    apply_edit(m, &edit).expect("edit applies")
}

#[test]
fn adding_and_removing_nodes_keeps_edges_consistent() {
    let m = map(&[("n1", NodeRole::Claim), ("n2", NodeRole::Ground)], &[("n2", "n1", EdgeKind::Support)]);
    let added = edit(&m, MapEdit::AddNode { role: NodeRole::Rebuttal, text: "  but   not always ".into(), source: PostId::from("p2") });
    assert_eq!(added.nodes[2].id, "n3");
    assert_eq!(added.nodes[2].text, "but not always");
    let removed = edit(&added, MapEdit::RemoveNode { id: "n1".into() });
    assert_eq!(removed.nodes.len(), 2);
    assert!(removed.edges.is_empty(), "the edge into n1 went with it");
    assert_eq!(apply_edit(&m, &MapEdit::RemoveNode { id: "zz".into() }), Err(EditError::UnknownNode));
    let blank = MapEdit::AddNode { role: NodeRole::Claim, text: "   ".into(), source: PostId::from("p") };
    assert_eq!(apply_edit(&m, &blank), Err(EditError::BlankText));
}

#[test]
fn relabelling_and_re_rolling_change_one_node() {
    let m = map(&[("n1", NodeRole::Claim), ("n2", NodeRole::Ground)], &[]);
    let long: String = "x".repeat(MAX_NODE_CHARS + 20);
    let relabelled = edit(&m, MapEdit::Relabel { id: "n2".into(), text: long });
    assert_eq!(relabelled.nodes[1].text.chars().count(), MAX_NODE_CHARS);
    assert_eq!(relabelled.nodes[0], m.nodes[0]);
    let rerolled = edit(&m, MapEdit::SetRole { id: "n1".into(), role: NodeRole::Qualifier });
    assert_eq!(rerolled.nodes[0].role, NodeRole::Qualifier);
    assert_eq!(apply_edit(&m, &MapEdit::Relabel { id: "n1".into(), text: "".into() }), Err(EditError::BlankText));
}

#[test]
fn edges_are_validated() {
    let m = map(&[("n1", NodeRole::Claim), ("n2", NodeRole::Ground)], &[]);
    let joined = edit(&m, MapEdit::AddEdge { from: "n2".into(), to: "n1".into(), kind: EdgeKind::Support });
    assert_eq!(joined.edges.len(), 1);
    assert_eq!(apply_edit(&joined, &MapEdit::AddEdge { from: "n2".into(), to: "n1".into(), kind: EdgeKind::Attack }), Err(EditError::DuplicateEdge));
    assert_eq!(apply_edit(&m, &MapEdit::AddEdge { from: "n1".into(), to: "n1".into(), kind: EdgeKind::Attack }), Err(EditError::SelfLoop));
    assert_eq!(apply_edit(&m, &MapEdit::AddEdge { from: "n1".into(), to: "n9".into(), kind: EdgeKind::Attack }), Err(EditError::UnknownNode));
    let flipped = edit(&joined, MapEdit::SetEdgeKind { from: "n2".into(), to: "n1".into(), kind: EdgeKind::Attack });
    assert_eq!(flipped.edges[0].kind, EdgeKind::Attack);
    let cut = edit(&flipped, MapEdit::RemoveEdge { from: "n2".into(), to: "n1".into() });
    assert!(cut.edges.is_empty());
    assert_eq!(apply_edit(&cut, &MapEdit::RemoveEdge { from: "n2".into(), to: "n1".into() }), Err(EditError::UnknownEdge));
}

#[test]
fn moving_reorders_and_clamps() {
    let m = map(&[("a", NodeRole::Claim), ("b", NodeRole::Ground), ("c", NodeRole::Ground)], &[]);
    let ids = |m: &super::ArgumentMap| m.nodes.iter().map(|n| n.id.clone()).collect::<Vec<_>>().join(",");
    assert_eq!(ids(&edit(&m, MapEdit::MoveNode { id: "c".into(), to: 0 })), "c,a,b");
    assert_eq!(ids(&edit(&m, MapEdit::MoveNode { id: "a".into(), to: 99 })), "b,c,a");
    assert_eq!(ids(&edit(&m, MapEdit::MoveNode { id: "b".into(), to: 1 })), "a,b,c");
}
