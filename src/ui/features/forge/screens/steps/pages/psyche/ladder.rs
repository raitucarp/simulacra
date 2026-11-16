//! Pure progress along the quotient level ladder (Novice → Legendary) for the skill bars.

use crate::state::forge::QuotientLevel;

/// Where a score stands on the level ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LadderStep {
    pub level: QuotientLevel,
    /// The next level and the points still needed to reach it; `None` at the top.
    pub next: Option<(QuotientLevel, u8)>,
}

impl LadderStep {
    pub fn of(score: u8) -> Self {
        let level = QuotientLevel::from_score(score);
        let next = QuotientLevel::ALL
            .into_iter()
            .find(|candidate| *candidate > level)
            .map(|next| (next, next.threshold().saturating_sub(score)));
        Self { level, next }
    }

    /// Whether the ladder rung `rung` is lit (reached) at this step.
    pub fn lit(self, rung: QuotientLevel) -> bool {
        rung <= self.level
    }
}
