//! A connection drill as immutable state: click a point, then another, to connect them
//! (clicking a connected pair again takes the connection back); pick the relation new
//! connections carry; check once to score against the reference.

use super::score::{score_drawing, DrawnEdge, MapScore};
use super::ConnectionMap;

/// The state of one drill.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Drill {
    pub drawn: Vec<DrawnEdge>,
    /// The first point of a connection being drawn.
    pub pending: Option<usize>,
    /// The relation new connections carry.
    pub label: Option<String>,
    /// Connections taken back so far.
    pub corrections: u32,
    /// The score, once checked (the drawing is then frozen).
    pub score: Option<MapScore>,
}

impl Drill {
    pub fn is_checked(&self) -> bool {
        self.score.is_some()
    }

    /// A click on point `point`.
    pub fn click(self, point: usize) -> Self {
        if self.is_checked() {
            return self;
        }
        match self.pending {
            None => Self { pending: Some(point), ..self },
            Some(first) if first == point => Self { pending: None, ..self },
            Some(first) => self.toggled(first, point),
        }
    }

    fn toggled(self, from: usize, to: usize) -> Self {
        let edge = DrawnEdge { from, to, label: self.label.clone() };
        let mut drawn = self.drawn;
        let corrections = match drawn.iter().position(|d| d.pair() == edge.pair()) {
            Some(existing) => {
                drawn.remove(existing);
                self.corrections + 1
            }
            None => {
                drawn.push(edge);
                self.corrections
            }
        };
        Self { drawn, pending: None, corrections, ..self }
    }

    /// New connections carry `label` (none for an unnamed connection).
    pub fn with_label(self, label: Option<String>) -> Self {
        Self { label, ..self }
    }

    /// Takes back every drawn connection.
    pub fn cleared(self) -> Self {
        if self.is_checked() || self.drawn.is_empty() {
            return self;
        }
        let corrections = self.corrections + self.drawn.len() as u32;
        Self { drawn: Vec::new(), pending: None, corrections, ..self }
    }

    /// Scores the drawing against `map`.
    pub fn checked(self, map: &ConnectionMap) -> Self {
        if self.is_checked() {
            return self;
        }
        let score = score_drawing(&map.edges, &self.drawn);
        Self { score: Some(score), pending: None, ..self }
    }
}
