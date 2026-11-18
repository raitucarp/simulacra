//! How the dialogue moves along Bloom's ladder.
//!
//! - **Climb** one level after a solid answer at the current level.
//! - **Stay** after a partial answer, probing with an example (or a definition at the
//!   bottom rung).
//! - **Probe, then step down**: a missed answer is probed once at the same level; after
//!   [`STEP_DOWN_AFTER`] misses in a row the tutor steps down a level and asks for an example.
//! - **Misconceptions** are met with a counterexample, then (if they persist) by putting the
//!   learner's own statements side by side.
//! - **Close** after [`MAX_EXCHANGES`] answers, or after a solid answer at the top rung.

use super::moves::SocraticMove;
use super::{Assessment, Exchange, Verdict};
use crate::domain::learning::bloom::BloomLevel;

/// Answers after which the dialogue closes with its summary.
pub const MAX_EXCHANGES: usize = 10;
/// Missed answers in a row at one level before the tutor steps down.
pub const STEP_DOWN_AFTER: usize = 2;

/// The level the last question aimed at (the bottom rung before any question).
pub fn current_level(exchanges: &[Exchange]) -> BloomLevel {
    exchanges.last().map_or(BloomLevel::LOWEST, |exchange| exchange.question.level)
}

/// Missed answers in a row, counted back from the last, at `level`.
pub fn misses_in_a_row(exchanges: &[Exchange], level: BloomLevel) -> usize {
    exchanges
        .iter()
        .rev()
        .take_while(|exchange| exchange.question.level == level)
        .take_while(|exchange| exchange.assessment.as_ref().is_some_and(|a| a.verdict == Verdict::Missing))
        .count()
}

/// The level of the next question after `assessment` of an answer at `current`, with
/// `misses` missed answers in a row at that level (this one included).
pub fn next_level(current: BloomLevel, assessment: &Assessment, misses: usize) -> BloomLevel {
    match assessment.verdict {
        Verdict::Solid => current.next(),
        Verdict::Partial => current,
        Verdict::Missing if misses >= STEP_DOWN_AFTER => current.previous(),
        Verdict::Missing => current,
    }
}

/// The move of the next question at `level`, after `previous` was assessed as `assessment`.
pub fn next_move(level: BloomLevel, previous: &Exchange, assessment: &Assessment) -> SocraticMove {
    let probed = previous.question.kind;
    if !assessment.misconceptions.is_empty() {
        return if probed == SocraticMove::Counterexample { SocraticMove::Contradiction } else { SocraticMove::Counterexample };
    }
    match assessment.verdict {
        _ if level < previous.question.level => SocraticMove::Example,
        Verdict::Partial | Verdict::Missing if level == BloomLevel::LOWEST => SocraticMove::Define,
        Verdict::Partial | Verdict::Missing => SocraticMove::Example,
        Verdict::Solid => SocraticMove::for_level(level),
    }
}

/// The highest level an answer showed: what a solid or partial answer reached (its
/// question's level when the assessment names none); a missed answer shows only what it
/// reached.
pub fn mastery(exchanges: &[Exchange]) -> Option<BloomLevel> {
    exchanges
        .iter()
        .filter_map(|exchange| {
            let assessment = exchange.assessment.as_ref()?;
            match assessment.verdict {
                Verdict::Solid => Some(assessment.reached.unwrap_or(exchange.question.level).max(exchange.question.level)),
                Verdict::Partial => Some(assessment.reached.unwrap_or(exchange.question.level.previous())),
                Verdict::Missing => assessment.reached,
            }
        })
        .max()
}

/// Whether the dialogue has done its work: enough answers, or the top rung met solidly.
pub fn should_close(exchanges: &[Exchange]) -> bool {
    let answered = exchanges.iter().filter(|exchange| exchange.assessment.is_some()).count();
    let top = exchanges.last().is_some_and(|exchange| {
        exchange.question.level == BloomLevel::HIGHEST && exchange.assessment.as_ref().is_some_and(|a| a.verdict == Verdict::Solid)
    });
    answered >= MAX_EXCHANGES || top
}
