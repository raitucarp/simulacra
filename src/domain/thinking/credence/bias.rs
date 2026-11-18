//! Over- and under-confidence: the mean confidence of the forecasts (`max(p, 1 − p)`, how
//! sure the forecaster was of the side they leaned to) minus the share of forecasts whose
//! side turned out right. Positive means more sure than right (overconfident), negative less
//! sure than right (underconfident). Forecasts of exactly 50 % lean nowhere and are left out.

use super::scoring::{mean, Forecast};
use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// Which way confidence and accuracy disagree; the key names its locale string.
    ConfidenceBias {
        Overconfident => "overconfident",
        Underconfident => "underconfident",
        Calibrated => "calibrated",
    }
}

/// A gap of at most this much (five points) counts as calibrated.
pub const CALIBRATED_MARGIN: f64 = 0.05;
/// The probability that leans to neither side.
const EVEN: f64 = 0.5;

fn leans(forecast: &Forecast) -> bool {
    (forecast.probability - EVEN).abs() > f64::EPSILON
}

/// Whether the side the forecast leaned to turned out right.
fn right_side(forecast: &Forecast) -> f64 {
    if (forecast.probability > EVEN) == forecast.happened {
        1.0
    } else {
        0.0
    }
}

/// Mean confidence minus the share on the right side (`None` when nothing leans).
pub fn overconfidence(forecasts: &[Forecast]) -> Option<f64> {
    let leaning: Vec<&Forecast> = forecasts.iter().filter(|f| leans(f)).collect();
    let confidence = mean(leaning.iter().map(|f| f.probability.max(1.0 - f.probability)))?;
    let right = mean(leaning.iter().map(|f| right_side(f)))?;
    Some(confidence - right)
}

/// The bias and its size (see [`overconfidence`]).
pub fn confidence_bias(forecasts: &[Forecast]) -> Option<(ConfidenceBias, f64)> {
    let gap = overconfidence(forecasts)?;
    let bias = match gap {
        g if g > CALIBRATED_MARGIN => ConfidenceBias::Overconfident,
        g if g < -CALIBRATED_MARGIN => ConfidenceBias::Underconfident,
        _ => ConfidenceBias::Calibrated,
    };
    Some((bias, gap))
}
