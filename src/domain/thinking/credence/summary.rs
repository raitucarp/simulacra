//! Every calibration figure of one set of credences, as the Calibration view shows them.

use super::bias::{confidence_bias, ConfidenceBias};
use super::calibration::{calibration_curve, CalibrationBin, CALIBRATION_BINS};
use super::model::{Credence, Outcome};
use super::murphy::{murphy_decomposition, MurphyDecomposition};
use super::scoring::{brier_score, forecasts, log_score};

/// Counts, scores, decomposition, bias and the curve.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationSummary {
    /// Resolved true or false (scored).
    pub resolved: usize,
    pub open: usize,
    /// Resolved as ambiguous (closed, not scored).
    pub ambiguous: usize,
    pub brier: Option<f64>,
    pub log: Option<f64>,
    pub murphy: Option<MurphyDecomposition>,
    pub bias: Option<(ConfidenceBias, f64)>,
    pub bins: Vec<CalibrationBin>,
}

/// The summary of `credences` over [`CALIBRATION_BINS`] bins.
pub fn summarize(credences: &[Credence]) -> CalibrationSummary {
    let scored = forecasts(credences);
    CalibrationSummary {
        resolved: scored.len(),
        open: credences.iter().filter(|c| c.is_open()).count(),
        ambiguous: credences.iter().filter(|c| c.outcome == Some(Outcome::Ambiguous)).count(),
        brier: brier_score(&scored),
        log: log_score(&scored),
        murphy: murphy_decomposition(&scored, CALIBRATION_BINS),
        bias: confidence_bias(&scored),
        bins: calibration_curve(&scored, CALIBRATION_BINS),
    }
}
