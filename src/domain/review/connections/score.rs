//! Scoring a drawing against the reference map. A drawn connection is right when it joins
//! the same two points as a reference connection, runs the same way when that one is
//! directed, and carries the same relation when that one is labelled. The drawing succeeds
//! only when every reference connection is drawn and nothing else is.

use super::{pair, same_label, MapEdge};

/// A connection the learner drew, from the first point clicked to the second.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DrawnEdge {
    pub from: usize,
    pub to: usize,
    pub label: Option<String>,
}

impl DrawnEdge {
    pub fn pair(&self) -> (usize, usize) {
        pair(self.from, self.to)
    }
}

/// What is wrong with a drawn connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeMistake {
    /// No such connection in the reference.
    Unexpected,
    /// The right points, the wrong way round.
    WrongDirection,
    /// The right points, another relation.
    WrongLabel,
}

impl EdgeMistake {
    pub const fn key(self) -> &'static str {
        match self {
            Self::Unexpected => "unexpected",
            Self::WrongDirection => "direction",
            Self::WrongLabel => "label",
        }
    }
}

/// How a drawing compares with the reference.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MapScore {
    pub correct: Vec<DrawnEdge>,
    pub wrong: Vec<(DrawnEdge, EdgeMistake)>,
    /// Reference connections not drawn correctly.
    pub missing: Vec<MapEdge>,
}

impl MapScore {
    pub fn drawn(&self) -> usize {
        self.correct.len() + self.wrong.len()
    }

    pub fn reference(&self) -> usize {
        self.correct.len() + self.missing.len()
    }

    /// Share of drawn connections that are right (0 when nothing was drawn).
    pub fn precision(&self) -> f64 {
        share(self.correct.len(), self.drawn())
    }

    /// Share of reference connections drawn right.
    pub fn recall(&self) -> f64 {
        share(self.correct.len(), self.reference())
    }

    /// The drawn structure is the reference structure, exactly.
    pub fn is_exact(&self) -> bool {
        self.wrong.is_empty() && self.missing.is_empty()
    }
}

fn share(part: usize, whole: usize) -> f64 {
    if whole == 0 { 0.0 } else { part as f64 / whole as f64 }
}

/// Whether `drawn` reproduces `reference`, or what is wrong with it.
fn verdict(reference: &MapEdge, drawn: &DrawnEdge) -> Option<EdgeMistake> {
    if reference.directed && (drawn.from, drawn.to) != (reference.from, reference.to) {
        return Some(EdgeMistake::WrongDirection);
    }
    match (&reference.label, &drawn.label) {
        (Some(expected), Some(given)) if same_label(expected, given) => None,
        (Some(_), _) => Some(EdgeMistake::WrongLabel),
        (None, _) => None,
    }
}

/// Compares `drawn` with `reference` (each reference connection matches one drawing).
pub fn score_drawing(reference: &[MapEdge], drawn: &[DrawnEdge]) -> MapScore {
    let mut used = vec![false; reference.len()];
    let mut score = MapScore::default();
    for edge in drawn {
        let matching = reference.iter().enumerate().find(|(i, r)| !used[*i] && r.pair() == edge.pair());
        match matching.map(|(i, r)| (i, verdict(r, edge))) {
            Some((i, None)) => {
                used[i] = true;
                score.correct.push(edge.clone());
            }
            Some((_, Some(mistake))) => score.wrong.push((edge.clone(), mistake)),
            None => score.wrong.push((edge.clone(), EdgeMistake::Unexpected)),
        }
    }
    score.missing = reference.iter().zip(used).filter(|(_, used)| !used).map(|(edge, _)| edge.clone()).collect();
    score
}
