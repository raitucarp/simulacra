//! The FSRS grade a checked drawing earns. Only an exact structure counts as recalled:
//! any missing or wrong connection is Again. An exact drawing is Good, Easy when it came
//! quickly and without second thoughts, and Hard when it took a long time.

use super::score::MapScore;
use crate::domain::review::Grade;

/// Seconds per reference connection under which an exact drawing counts as fluent.
pub const FAST_SECONDS_PER_EDGE: u64 = 12;
/// Seconds per reference connection above which an exact drawing counts as laboured.
pub const SLOW_SECONDS_PER_EDGE: u64 = 45;

/// How the drawing went.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DrillEffort {
    /// Seconds from showing the points to checking.
    pub seconds: u64,
    /// Connections drawn and then taken back.
    pub corrections: u32,
}

/// The grade of a drawing scored `score` after `effort`.
pub fn drill_grade(score: &MapScore, effort: DrillEffort) -> Grade {
    if !score.is_exact() || score.reference() == 0 {
        return Grade::Again;
    }
    let per_edge = effort.seconds / score.reference() as u64;
    match per_edge {
        fast if fast <= FAST_SECONDS_PER_EDGE && effort.corrections == 0 => Grade::Easy,
        slow if slow > SLOW_SECONDS_PER_EDGE => Grade::Hard,
        _ => Grade::Good,
    }
}
