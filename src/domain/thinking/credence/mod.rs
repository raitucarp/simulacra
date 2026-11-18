//! Credence and calibration (sb E1): a stated probability on a post's claim and how well
//! such statements match what later happened.
//!
//! - [`model`]: a [`Credence`] (1–99 % on the main claim or one sentence, an optional
//!   "resolves by" date, a resolution note) and its [`Outcome`] (true, false, ambiguous).
//! - [`due`]: open predictions by due date, the ones due today, and the due-date presets.
//! - [`scoring`]: resolved credences as [`Forecast`]s, the Brier score
//!   `1/N Σ (p − o)²` (0 best, 1 worst) and the log score `1/N Σ log₂ p(outcome)` (0 best).
//! - [`wilson`] / [`calibration`]: the calibration curve, forecasts binned by stated
//!   probability with the observed frequency of each bin and its Wilson score interval.
//! - [`murphy`]: Murphy's decomposition of the binned Brier score into reliability,
//!   resolution and uncertainty, `BS = REL − RES + UNC`.
//! - [`bias`]: over- or under-confidence, mean confidence minus the share of forecasts on
//!   the right side of one half.
//! - [`summary`]: everything above for one set of credences.
//!
//! Ambiguous outcomes are kept (they close a prediction) but never scored.

pub mod bias;
pub mod calibration;
pub mod due;
pub mod model;
pub mod murphy;
pub mod scoring;
pub mod summary;
pub mod wilson;

#[cfg(test)]
mod tests;

pub use bias::{confidence_bias, overconfidence, ConfidenceBias, CALIBRATED_MARGIN};
pub use calibration::{bin_index, calibration_curve, CalibrationBin, CALIBRATION_BINS};
pub use due::{due_on, open_by_due, DuePreset};
pub use model::{clamp_percent, Credence, Outcome, MAX_PERCENT, MIN_PERCENT};
pub use murphy::{murphy_decomposition, MurphyDecomposition};
pub use scoring::{brier_score, forecasts, log_score, Forecast};
pub use summary::{summarize, CalibrationSummary};
pub use wilson::{wilson_interval, WilsonInterval, WILSON_Z};
