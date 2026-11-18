//! Murphy's decomposition of the Brier score (Murphy 1973), over `K` probability bins:
//!
//! - reliability `REL = 1/N Σₖ nₖ (fₖ − oₖ)²`: how far each bin's mean forecast `fₖ` is from
//!   its observed frequency `oₖ` (lower is better; 0 is perfect calibration);
//! - resolution `RES = 1/N Σₖ nₖ (oₖ − ō)²`: how much the bins' frequencies differ from the
//!   overall base rate `ō` (higher is better: the forecasts tell cases apart);
//! - uncertainty `UNC = ō (1 − ō)`: how hard the questions were, beyond the forecaster.
//!
//! `BS = REL − RES + UNC` holds exactly for the binned score, where each forecast is replaced
//! by its bin's mean; it equals the plain Brier score when every forecast in a bin is equal.

use super::calibration::{calibration_curve, CalibrationBin};
use super::scoring::{mean, Forecast};

/// The three parts of the binned Brier score.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MurphyDecomposition {
    pub reliability: f64,
    pub resolution: f64,
    pub uncertainty: f64,
    /// `reliability − resolution + uncertainty`.
    pub brier: f64,
}

/// `1/N Σₖ nₖ · term(fₖ, oₖ)` over the non-empty bins.
fn weighted(curve: &[CalibrationBin], total: f64, term: impl Fn(f64, f64) -> f64) -> f64 {
    curve.iter().filter(|b| b.count > 0).map(|b| b.count as f64 * term(b.mean_forecast, b.observed)).sum::<f64>() / total
}

/// The decomposition over `bins` bins (`None` without forecasts).
pub fn murphy_decomposition(forecasts: &[Forecast], bins: usize) -> Option<MurphyDecomposition> {
    let base_rate = mean(forecasts.iter().map(Forecast::observed))?;
    let total = forecasts.len() as f64;
    let curve = calibration_curve(forecasts, bins);
    let reliability = weighted(&curve, total, |forecast, observed| (forecast - observed).powi(2));
    let resolution = weighted(&curve, total, |_, observed| (observed - base_rate).powi(2));
    let uncertainty = base_rate * (1.0 - base_rate);
    Some(MurphyDecomposition { reliability, resolution, uncertainty, brier: reliability - resolution + uncertainty })
}
