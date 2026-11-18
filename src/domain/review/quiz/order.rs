//! Put it back in order: the posts of a thread or the sentences of a passage, shuffled; the
//! learner rebuilds the line of thought (reconstructing structure is retrieval of how ideas
//! connect, not just of facts).

use super::shuffle::derangement_order;

/// Fewest and most pieces in one puzzle.
pub const MIN_PIECES: usize = 3;
pub const MAX_PIECES: usize = 7;

/// A shuffled sequence being put back in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderPuzzle {
    pub title: String,
    /// The pieces in their right order.
    pub pieces: Vec<String>,
    /// The current arrangement: indices into `pieces`, top to bottom.
    pub arrangement: Vec<usize>,
    pub checked: bool,
}

/// How a checked arrangement scored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderScore {
    /// Pieces in their right position.
    pub in_place: usize,
    /// Neighbouring pairs in the right order (credit for runs that are only shifted).
    pub pairs_in_order: usize,
    pub total: usize,
}

impl OrderScore {
    pub fn is_perfect(&self) -> bool {
        self.in_place == self.total
    }
}

impl OrderPuzzle {
    /// A puzzle of `pieces` (at most [`MAX_PIECES`]); `None` with fewer than [`MIN_PIECES`].
    pub fn new(title: String, pieces: Vec<String>, seed: u64) -> Option<Self> {
        let pieces: Vec<String> = pieces.into_iter().filter(|p| !p.trim().is_empty()).take(MAX_PIECES).collect();
        (pieces.len() >= MIN_PIECES).then(|| Self { arrangement: derangement_order(pieces.len(), seed), title, pieces, checked: false })
    }

    /// The piece at `slot` moved one place up (towards the top), if it can be.
    pub fn move_up(self, slot: usize) -> Self {
        if slot == 0 || slot >= self.arrangement.len() || self.checked {
            return self;
        }
        let mut arrangement = self.arrangement;
        arrangement.swap(slot - 1, slot);
        Self { arrangement, ..self }
    }

    /// The piece at `slot` moved one place down.
    pub fn move_down(self, slot: usize) -> Self {
        let slot_below = slot + 1;
        if slot_below >= self.arrangement.len() {
            return self;
        }
        self.move_up(slot_below)
    }

    pub fn check(self) -> Self {
        Self { checked: true, ..self }
    }

    /// Whether the piece at `slot` is where it belongs.
    pub fn is_in_place(&self, slot: usize) -> bool {
        self.arrangement.get(slot) == Some(&slot)
    }

    pub fn score(&self) -> OrderScore {
        let in_place = (0..self.arrangement.len()).filter(|slot| self.is_in_place(*slot)).count();
        let pairs_in_order = self.arrangement.windows(2).filter(|pair| pair[1] == pair[0] + 1).count();
        OrderScore { in_place, pairs_in_order, total: self.pieces.len() }
    }
}
