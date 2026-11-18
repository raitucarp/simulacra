//! Connection recall: spaced repetition of how ideas connect rather than of isolated facts.
//!
//! The method follows Justin Sung's teaching on learning as building a connected web of
//! knowledge: what is retrieved is the structure, the relations between points, and the
//! retrieval only succeeds when that structure is reproduced exactly.
//!
//! - [`ConnectionMap`]: points (posts, vault passages, concepts) and the reference
//!   connections between them, each optionally directed and labelled.
//! - [`edit`]: the pure edits of the map editor; [`seed`]: maps seeded from threads,
//!   selections of posts and a vault document's related entries.
//! - [`drill`]: drawing connections between scattered points ([`layout`]), [`score`]:
//!   comparing them with the reference, and [`grade`]: the FSRS grade the result earns.

pub mod drill;
pub mod edit;
pub mod grade;
pub mod layout;
pub mod score;
pub mod seed;

pub use drill::Drill;
pub use grade::{drill_grade, DrillEffort};
pub use score::{score_drawing, DrawnEdge, EdgeMistake, MapScore};

use super::item::Anchor;
use crate::domain::post::SourceLocator;
use serde::{Deserialize, Serialize};

/// Fewest points and connections a map needs, and the most points it may hold.
pub const MIN_POINTS: usize = 3;
pub const MIN_EDGES: usize = 2;
pub const MAX_POINTS: usize = 12;

/// One point of a map.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MapPoint {
    Post { id: String },
    /// A vault document passage (no text: the whole document, shown by its title).
    Passage {
        document_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        locator: Option<SourceLocator>,
        #[serde(default)]
        text: String,
    },
    /// A concept the learner named.
    Concept { label: String },
}

impl MapPoint {
    /// The source this point is, if it is a post or a passage.
    pub fn anchor(&self) -> Option<Anchor> {
        match self {
            Self::Post { id } => Some(Anchor::Post { id: id.clone() }),
            Self::Passage { document_id, locator, text } => Some(Anchor::passage(document_id.clone(), locator.clone(), text.clone())),
            Self::Concept { .. } => None,
        }
    }
}

/// A reference connection between points `from` and `to` (indices into the points).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MapEdge {
    pub from: usize,
    pub to: usize,
    /// Whether the connection only holds from `from` to `to`.
    #[serde(default)]
    pub directed: bool,
    /// The relation's name; when set, a drawn connection must carry the same one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl MapEdge {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to, directed: false, label: None }
    }

    /// The unordered pair of points it joins.
    pub fn pair(&self) -> (usize, usize) {
        pair(self.from, self.to)
    }
}

/// `(a, b)` as an unordered pair, smaller index first.
pub fn pair(a: usize, b: usize) -> (usize, usize) {
    (a.min(b), a.max(b))
}

/// A relation label compared without case or surrounding space.
pub fn same_label(a: &str, b: &str) -> bool {
    a.trim().to_lowercase() == b.trim().to_lowercase()
}

/// A map of points and their reference connections.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConnectionMap {
    pub title: String,
    pub points: Vec<MapPoint>,
    pub edges: Vec<MapEdge>,
}

impl ConnectionMap {
    /// The distinct relation labels of the reference, sorted (the drill offers these).
    pub fn labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = Vec::new();
        for label in self.edges.iter().filter_map(|edge| edge.label.as_deref()).map(str::trim).filter(|l| !l.is_empty()) {
            if !labels.iter().any(|known| same_label(known, label)) {
                labels.push(label.to_string());
            }
        }
        labels.sort_by_key(|label| label.to_lowercase());
        labels
    }

    /// The reference edge joining `a` and `b`, if any.
    pub fn edge_between(&self, a: usize, b: usize) -> Option<usize> {
        self.edges.iter().position(|edge| edge.pair() == pair(a, b))
    }

    /// Whether the map refers to post `id`.
    pub fn has_post(&self, id: &str) -> bool {
        self.points.iter().any(|point| matches!(point, MapPoint::Post { id: post } if post == id))
    }
}
