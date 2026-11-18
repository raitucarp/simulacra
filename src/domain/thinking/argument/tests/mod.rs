//! Argument maps: edits, analysis and semantics, extraction parsing and layout.

mod analysis;
mod edit;
mod extract;

use super::model::{ArgEdge, ArgNode, ArgumentMap, EdgeKind, NodeRole};
use crate::domain::PostId;

/// A map of `nodes` (`(id, role)`, all citing the root) and `edges` (`(from, to, kind)`).
fn map(nodes: &[(&str, NodeRole)], edges: &[(&str, &str, EdgeKind)]) -> ArgumentMap {
    let root = PostId::from("root");
    ArgumentMap {
        root: root.clone(),
        nodes: nodes.iter().map(|(id, role)| ArgNode { id: id.to_string(), role: *role, text: format!("text of {id}"), source: root.clone() }).collect(),
        edges: edges.iter().map(|(from, to, kind)| ArgEdge { from: from.to_string(), to: to.to_string(), kind: *kind }).collect(),
    }
}

#[test]
fn layout_puts_claims_above_grounds_and_wraps_rows() {
    use NodeRole::*;
    let m = map(&[("n1", Claim), ("n2", Ground), ("n3", Ground), ("n4", Ground), ("n5", Warrant)], &[]);
    let wide = super::layout(&m, 4);
    assert_eq!(wide.rows, 3);
    assert_eq!(wide.positions[0], (0.5, 0.0));
    assert!(wide.positions[1].1 > wide.positions[0].1 && wide.positions[4].1 > wide.positions[1].1);
    assert_eq!((wide.positions[1].0, wide.positions[3].0), (0.0, 1.0));
    let narrow = super::layout(&m, 2);
    assert_eq!(narrow.rows, 4, "three grounds wrap into two rows");
    assert_eq!(narrow.positions[4].1, 1.0);
    assert!(narrow.positions.iter().all(|(x, y)| (0.0..=1.0).contains(x) && (0.0..=1.0).contains(y)));
}

#[test]
fn maps_round_trip_through_json() {
    let m = map(&[("n1", NodeRole::Claim), ("n2", NodeRole::Ground)], &[("n2", "n1", EdgeKind::Support)]);
    assert_eq!(ArgumentMap::from_json(&m.to_json()), Some(m.clone()));
    assert!(m.to_json().contains("\"support\"") && m.to_json().contains("\"ground\""), "wire keys: {}", m.to_json());
    assert_eq!(ArgumentMap::from_json("{not json"), None);
    assert_eq!(m.next_node_id(), "n3");
}
