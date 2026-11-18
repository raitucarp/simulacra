//! Set theory over posts: named sets (by concept, author, kind or vault subject), the
//! classic operations on sorted member lists in linear time, Jaccard similarity, and the
//! exclusive regions that a Venn diagram (2–3 sets) or an UpSet matrix (4+ sets) draws.

mod regions;
#[cfg(test)]
mod tests;

pub use regions::{exclusive_regions, upset_rows, Region};

/// What a family of sets groups posts by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetDimension {
    #[default]
    Concept,
    Author,
    Kind,
    Subject,
}

impl SetDimension {
    pub const ALL: [Self; 4] = [Self::Concept, Self::Author, Self::Kind, Self::Subject];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Concept => "concept",
            Self::Author => "author",
            Self::Kind => "kind",
            Self::Subject => "subject",
        }
    }
}

/// One set: a stable key, its label and its members (post indices, ascending, distinct).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedSet {
    pub key: String,
    pub label: String,
    pub members: Vec<u32>,
}

impl NamedSet {
    /// A set of `members` in any order, with duplicates.
    pub fn new(key: impl Into<String>, label: impl Into<String>, mut members: Vec<u32>) -> Self {
        members.sort_unstable();
        members.dedup();
        Self { key: key.into(), label: label.into(), members }
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

/// Merges two ascending lists, keeping elements for which `keep(in_a, in_b)` holds.
fn merge(a: &[u32], b: &[u32], keep: impl Fn(bool, bool) -> bool) -> Vec<u32> {
    let (mut i, mut j, mut out) = (0, 0, Vec::with_capacity(a.len().max(b.len())));
    while i < a.len() || j < b.len() {
        let (x, y) = (a.get(i).copied(), b.get(j).copied());
        let take = match (x, y) {
            (Some(x), Some(y)) if x == y => { (i, j) = (i + 1, j + 1); (x, true, true) }
            (Some(x), Some(y)) if x < y => { i += 1; (x, true, false) }
            (Some(x), None) => { i += 1; (x, true, false) }
            (_, Some(y)) => { j += 1; (y, false, true) }
            (None, None) => break,
        };
        if keep(take.1, take.2) {
            out.push(take.0);
        }
    }
    out
}

/// `a ∩ b`.
pub fn intersection(a: &[u32], b: &[u32]) -> Vec<u32> {
    merge(a, b, |x, y| x && y)
}

/// `a ∪ b`.
pub fn union(a: &[u32], b: &[u32]) -> Vec<u32> {
    merge(a, b, |x, y| x || y)
}

/// `a ∖ b`.
pub fn difference(a: &[u32], b: &[u32]) -> Vec<u32> {
    merge(a, b, |x, y| x && !y)
}

/// `|a ∩ b| / |a ∪ b|` (1 for two empty sets).
pub fn jaccard(a: &[u32], b: &[u32]) -> f64 {
    let both = union(a, b).len();
    if both == 0 {
        return 1.0;
    }
    intersection(a, b).len() as f64 / both as f64
}

/// Jaccard similarity of every pair of `sets`, row-major (`n × n`, 1 on the diagonal).
pub fn jaccard_matrix(sets: &[NamedSet]) -> Vec<Vec<f64>> {
    sets.iter().map(|a| sets.iter().map(|b| jaccard(&a.members, &b.members)).collect()).collect()
}
