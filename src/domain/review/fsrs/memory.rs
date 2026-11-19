//! The FSRS memory formulas (see the module documentation for the source of each).

use super::{Params, MAX_DIFFICULTY, MIN_DIFFICULTY, MIN_STABILITY};
use crate::domain::review::Grade;

/// Grade a new memory's difficulty reverts towards (`D0(Easy)`).
const MEAN_REVERSION_GRADE: Grade = Grade::Easy;
/// Grade whose difficulty change is zero.
const NEUTRAL_GRADE: f64 = 3.0;
/// Upper bound in the damping and recall terms (`10 − D`, `11 − D`).
const DAMPING_TOP: f64 = 10.0;
const RECALL_TOP: f64 = 11.0;
const DAMPING_SPAN: f64 = 9.0;

fn clamp_difficulty(difficulty: f64) -> f64 {
    difficulty.clamp(MIN_DIFFICULTY, MAX_DIFFICULTY)
}

/// Probability of recall `elapsed_days` after the last review.
pub fn retrievability(params: &Params, elapsed_days: f64, stability: f64) -> f64 {
    let stability = stability.max(MIN_STABILITY);
    (1.0 + params.factor() * elapsed_days.max(0.0) / stability).powf(-params.decay())
}

/// Days until recall probability falls to `retention` (unrounded).
pub fn interval_days(params: &Params, stability: f64, retention: f64) -> f64 {
    stability / params.factor() * (retention.powf(-1.0 / params.decay()) - 1.0)
}

/// Stability after the very first review.
pub fn initial_stability(params: &Params, grade: Grade) -> f64 {
    params.w(grade.index()).max(MIN_STABILITY)
}

/// Difficulty after the very first review (unclamped, as the mean-reversion target).
fn raw_initial_difficulty(params: &Params, grade: Grade) -> f64 {
    params.w(4) - (params.w(5) * (grade.value() - 1.0)).exp() + 1.0
}

/// Difficulty after the very first review.
pub fn initial_difficulty(params: &Params, grade: Grade) -> f64 {
    clamp_difficulty(raw_initial_difficulty(params, grade))
}

/// Difficulty after a later review: linear damping, then mean reversion.
pub fn next_difficulty(params: &Params, difficulty: f64, grade: Grade) -> f64 {
    let delta = -params.w(6) * (grade.value() - NEUTRAL_GRADE);
    let damped = difficulty + delta * (DAMPING_TOP - difficulty) / DAMPING_SPAN;
    let reverted = params.w(7) * raw_initial_difficulty(params, MEAN_REVERSION_GRADE) + (1.0 - params.w(7)) * damped;
    clamp_difficulty(reverted)
}

/// Stability after a successful recall (Hard, Good or Easy) at retrievability `r`.
pub fn recall_stability(params: &Params, memory_difficulty: f64, stability: f64, r: f64, grade: Grade) -> f64 {
    let hard_penalty = if grade == Grade::Hard { params.w(15) } else { 1.0 };
    let easy_bonus = if grade == Grade::Easy { params.w(16) } else { 1.0 };
    let growth = params.w(8).exp()
        * (RECALL_TOP - memory_difficulty)
        * stability.powf(-params.w(9))
        * ((params.w(10) * (1.0 - r)).exp() - 1.0)
        * hard_penalty
        * easy_bonus;
    (stability * (growth + 1.0)).max(MIN_STABILITY)
}

/// Stability after a lapse (Again) at retrievability `r`; never above the old stability.
pub fn lapse_stability(params: &Params, difficulty: f64, stability: f64, r: f64) -> f64 {
    let long_term = params.w(11)
        * difficulty.powf(-params.w(12))
        * ((stability + 1.0).powf(params.w(13)) - 1.0)
        * (params.w(14) * (1.0 - r)).exp();
    let short_term = stability / (params.w(17) * params.w(18)).exp();
    long_term.min(short_term).max(MIN_STABILITY)
}

/// Stability after a second review on the same day.
pub fn same_day_stability(params: &Params, stability: f64, grade: Grade) -> f64 {
    let growth = (params.w(17) * (grade.value() - NEUTRAL_GRADE + params.w(18))).exp() * stability.powf(-params.w(19));
    let growth = if grade >= Grade::Good { growth.max(1.0) } else { growth };
    (stability * growth).max(MIN_STABILITY)
}
