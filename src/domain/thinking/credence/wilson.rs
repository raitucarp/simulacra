//! The Wilson score interval: a confidence interval for an observed frequency that stays
//! inside [0, 1] and behaves well for small samples (unlike the normal approximation).
//!
//! For `s` successes in `n` trials, `p̂ = s / n` and the normal quantile `z`: centre
//! `(p̂ + z²/2n) / (1 + z²/n)`, half-width `z / (1 + z²/n) · √(p̂(1 − p̂)/n + z²/4n²)`.

/// Normal quantile of a 95 % interval.
pub const WILSON_Z: f64 = 1.96;
const HALF: f64 = 0.5;
const QUARTER: f64 = 0.25;

/// A confidence interval for a frequency.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WilsonInterval {
    pub low: f64,
    pub high: f64,
}

/// The Wilson interval of `successes` in `trials` at quantile `z` (`None` without trials).
pub fn wilson_interval(successes: usize, trials: usize, z: f64) -> Option<WilsonInterval> {
    if trials == 0 {
        return None;
    }
    let n = trials as f64;
    let share = successes.min(trials) as f64 / n;
    let z2 = z * z;
    let denominator = 1.0 + z2 / n;
    let centre = (share + z2 * HALF / n) / denominator;
    let half_width = z / denominator * (share * (1.0 - share) / n + z2 * QUARTER / (n * n)).sqrt();
    Some(WilsonInterval { low: (centre - half_width).max(0.0), high: (centre + half_width).min(1.0) })
}
