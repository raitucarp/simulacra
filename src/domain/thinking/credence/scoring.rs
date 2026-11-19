//! Proper scoring rules over resolved predictions.
//!
//! - Brier score: `1/N Σ (p − o)²`, with `o` 1 when the claim held and 0 when it failed.
//!   0 is perfect, 0.25 is what always saying 50 % earns, 1 is certainty in the wrong answer.
//! - Log score: `1/N Σ log₂ q`, with `q` the probability given to what actually happened
//!   (`p` when it held, `1 − p` when it failed). 0 is perfect, −1 is what 50 % earns; it
//!   punishes confident misses much harder than the Brier score does.

use super::model::{Credence, Outcome};

/// One resolved prediction: the stated probability and whether the claim held.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Forecast {
    pub probability: f64,
    pub happened: bool,
}

impl Forecast {
    /// 1 when the claim held, 0 when it failed.
    pub fn observed(&self) -> f64 {
        if self.happened {
            1.0
        } else {
            0.0
        }
    }

    /// The probability given to what actually happened.
    pub fn probability_of_outcome(&self) -> f64 {
        if self.happened {
            self.probability
        } else {
            1.0 - self.probability
        }
    }
}

/// The scorable credences (resolved true or false; open and ambiguous ones are left out).
pub fn forecasts(credences: &[Credence]) -> Vec<Forecast> {
    credences
        .iter()
        .filter_map(|credence| match credence.outcome {
            Some(Outcome::True) => Some(Forecast { probability: credence.probability(), happened: true }),
            Some(Outcome::False) => Some(Forecast { probability: credence.probability(), happened: false }),
            Some(Outcome::Ambiguous) | None => None,
        })
        .collect()
}

/// The mean of `values` (`None` when there are none).
pub(super) fn mean(values: impl Iterator<Item = f64>) -> Option<f64> {
    let (sum, count) = values.fold((0.0, 0usize), |(sum, count), value| (sum + value, count + 1));
    (count > 0).then(|| sum / count as f64)
}

/// Mean squared error of the forecasts (`None` when empty).
pub fn brier_score(forecasts: &[Forecast]) -> Option<f64> {
    mean(forecasts.iter().map(|f| (f.probability - f.observed()).powi(2)))
}

/// Mean base-2 logarithm of the probability given to what happened (`None` when empty).
pub fn log_score(forecasts: &[Forecast]) -> Option<f64> {
    mean(forecasts.iter().map(|f| f.probability_of_outcome().log2()))
}
