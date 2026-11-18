//! The map editor's edits as pure functions: connect or disconnect two points, cycle a
//! connection's direction, name it, drop a point (its connections go with it and the rest
//! are renumbered) and add a concept. Plus what keeps a map from being saved.

use super::{pair, same_label, ConnectionMap, MapEdge, MapPoint, MAX_POINTS, MIN_EDGES, MIN_POINTS};

/// Why a map cannot be saved yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapProblem {
    TooFewPoints,
    TooFewConnections,
    /// A point no connection reaches (it would be a free guess in the drill).
    LonelyPoint,
}

impl MapProblem {
    pub const fn message_key(self) -> &'static str {
        match self {
            Self::TooFewPoints => "review.map.problem_points",
            Self::TooFewConnections => "review.map.problem_edges",
            Self::LonelyPoint => "review.map.problem_lonely",
        }
    }
}

/// The first problem of `map`, if any.
pub fn map_problem(map: &ConnectionMap) -> Option<MapProblem> {
    let reached = |index: usize| map.edges.iter().any(|edge| edge.from == index || edge.to == index);
    if map.points.len() < MIN_POINTS {
        Some(MapProblem::TooFewPoints)
    } else if map.edges.len() < MIN_EDGES {
        Some(MapProblem::TooFewConnections)
    } else if !(0..map.points.len()).all(reached) {
        Some(MapProblem::LonelyPoint)
    } else {
        None
    }
}

/// `map` with the connection between `a` and `b` removed, or added (directed from `a` to
/// `b` when `directed`, carrying `label`). Joining a point to itself changes nothing.
pub fn toggled_edge(map: ConnectionMap, a: usize, b: usize, directed: bool, label: Option<String>) -> ConnectionMap {
    if a == b || a >= map.points.len() || b >= map.points.len() {
        return map;
    }
    let mut edges = map.edges;
    match edges.iter().position(|edge| edge.pair() == pair(a, b)) {
        Some(existing) => {
            edges.remove(existing);
        }
        None => edges.push(MapEdge { from: a, to: b, directed, label: label.map(|l| l.trim().to_string()).filter(|l| !l.is_empty()) }),
    }
    ConnectionMap { edges, ..map }
}

/// Connection `index` turned undirected → forwards (lower point first) → backwards →
/// undirected.
pub fn cycled_direction(map: ConnectionMap, index: usize) -> ConnectionMap {
    let edges = map
        .edges
        .into_iter()
        .enumerate()
        .map(|(i, edge)| {
            let (low, high) = edge.pair();
            match (i == index, edge.directed) {
                (false, _) => edge,
                (true, false) => MapEdge { directed: true, from: low, to: high, ..edge },
                (true, true) if edge.from < edge.to => MapEdge { from: high, to: low, ..edge },
                (true, true) => MapEdge { directed: false, from: low, to: high, ..edge },
            }
        })
        .collect();
    ConnectionMap { edges, ..map }
}

/// Connection `index` named `label` (blank clears it).
pub fn relabelled(map: ConnectionMap, index: usize, label: &str) -> ConnectionMap {
    let label = Some(label.trim().to_string()).filter(|l| !l.is_empty());
    let edges = map.edges.into_iter().enumerate().map(|(i, edge)| if i == index { MapEdge { label: label.clone(), ..edge } } else { edge }).collect();
    ConnectionMap { edges, ..map }
}

pub fn without_edge(map: ConnectionMap, index: usize) -> ConnectionMap {
    let edges = map.edges.into_iter().enumerate().filter(|(i, _)| *i != index).map(|(_, edge)| edge).collect();
    ConnectionMap { edges, ..map }
}

/// `map` without point `index`; its connections are dropped and later points renumbered.
pub fn without_point(map: ConnectionMap, index: usize) -> ConnectionMap {
    let shift = |i: usize| if i > index { i - 1 } else { i };
    let points = map.points.into_iter().enumerate().filter(|(i, _)| *i != index).map(|(_, p)| p).collect();
    let edges = map
        .edges
        .into_iter()
        .filter(|edge| edge.from != index && edge.to != index)
        .map(|edge| MapEdge { from: shift(edge.from), to: shift(edge.to), ..edge })
        .collect();
    ConnectionMap { points, edges, ..map }
}

/// `map` with a concept point named `label` (unchanged when blank, known or full).
pub fn with_concept(map: ConnectionMap, label: &str) -> ConnectionMap {
    let label = label.trim();
    let known = map.points.iter().any(|p| matches!(p, MapPoint::Concept { label: l } if same_label(l, label)));
    if label.is_empty() || known || map.points.len() >= MAX_POINTS {
        return map;
    }
    let mut points = map.points;
    points.push(MapPoint::Concept { label: label.to_string() });
    ConnectionMap { points, ..map }
}
