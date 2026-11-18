//! The calibration curve (reliability diagram): forecasts binned by stated probability, each
//! bin with its mean forecast, the observed frequency and that frequency's Wilson interval.
//! A calibrated forecaster's bins sit on the diagonal: of the claims given 70 %, 70 % held.

use super::scoring::{mean, Forecast};
use super::wilson::{wilson_interval, WilsonInterval, WILSON_Z};

/// Bins of the curve: tenths of probability.
pub const CALIBRATION_BINS: usize = 10;

/// One bin of the curve.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationBin {
    pub index: usize,
    /// Bounds of the bin's stated probabilities (`[low, high)`, the last bin closed).
    pub low: f64,
    pub high: f64,
    pub count: usize,
    /// Forecasts in the bin whose claim held.
    pub hits: usize,
    /// Mean stated probability (0 when empty).
    pub mean_forecast: f64,
    /// Share of the bin's claims that held (0 when empty).
    pub observed: f64,
    pub interval: Option<WilsonInterval>,
}

/// The bin (of `bins`) a probability falls in; 1.0 goes in the last bin.
pub fn bin_index(probability: f64, bins: usize) -> usize {
    let bins = bins.max(1);
    ((probability.clamp(0.0, 1.0) * bins as f64) as usize).min(bins - 1)
}

/// The forecasts of bin `index`.
fn members(forecasts: &[Forecast], index: usize, bins: usize) -> Vec<Forecast> {
    forecasts.iter().copied().filter(|f| bin_index(f.probability, bins) == index).collect()
}

fn bin(forecasts: &[Forecast], index: usize, bins: usize) -> CalibrationBin {
    let width = 1.0 / bins as f64;
    let inside = members(forecasts, index, bins);
    let hits = inside.iter().filter(|f| f.happened).count();
    CalibrationBin {
        index,
        low: index as f64 * width,
        high: (index + 1) as f64 * width,
        count: inside.len(),
        hits,
        mean_forecast: mean(inside.iter().map(|f| f.probability)).unwrap_or_default(),
        observed: mean(inside.iter().map(Forecast::observed)).unwrap_or_default(),
        interval: wilson_interval(hits, inside.len(), WILSON_Z),
    }
}

/// Every bin of the curve, empty ones included (`bins` of at least one).
pub fn calibration_curve(forecasts: &[Forecast], bins: usize) -> Vec<CalibrationBin> {
    let bins = bins.max(1);
    (0..bins).map(|index| bin(forecasts, index, bins)).collect()
}
