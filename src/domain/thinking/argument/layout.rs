//! Where each node sits on the canvas, as fractions of the free area (`0..=1` both ways):
//! one band per tier of Toulmin's layout, claims on top, the reasons for them below, the
//! warrants and their backing under those. Within a band nodes keep their display order and
//! wrap into further rows when more than `columns` fit; empty tiers take no room.

use super::model::{ArgumentMap, NodeRole};

/// Bands from top to bottom.
const TIERS: &[&[NodeRole]] = &[
    &[NodeRole::Claim, NodeRole::Qualifier],
    &[NodeRole::Ground, NodeRole::Rebuttal],
    &[NodeRole::Warrant],
    &[NodeRole::Backing],
];
/// The middle of a span, for a lone node.
const CENTRE: f32 = 0.5;

/// Node positions and how many rows they take.
#[derive(Debug, Clone, PartialEq)]
pub struct MapLayout {
    pub positions: Vec<(f32, f32)>,
    pub rows: usize,
}

/// The tier of `role`.
fn tier(role: NodeRole) -> usize {
    TIERS.iter().position(|roles| roles.contains(&role)).unwrap_or(0)
}

/// Spread `count` slots across `0..=1` (centred when alone).
fn spread(slot: usize, count: usize) -> f32 {
    if count <= 1 { CENTRE } else { slot as f32 / (count - 1) as f32 }
}

/// The layout of `map` with at most `columns` nodes per row.
pub fn layout(map: &ArgumentMap, columns: usize) -> MapLayout {
    let columns = columns.max(1);
    let per_tier: Vec<Vec<usize>> = (0..TIERS.len()).map(|t| (0..map.nodes.len()).filter(|n| tier(map.nodes[*n].role) == t).collect()).collect();
    let rows: Vec<Vec<usize>> = per_tier.iter().flat_map(|members| members.chunks(columns).map(<[usize]>::to_vec).collect::<Vec<_>>()).collect();
    let mut positions = vec![(CENTRE, CENTRE); map.nodes.len()];
    for (row, members) in rows.iter().enumerate() {
        for (slot, node) in members.iter().enumerate() {
            positions[*node] = (spread(slot, members.len()), spread(row, rows.len()));
        }
    }
    MapLayout { positions, rows: rows.len() }
}
