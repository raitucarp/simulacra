//! "Nice" axis ticks (steps of 1, 2 or 5 times a power of ten) and compact number labels.

/// Multipliers of a power of ten that make readable steps.
const NICE_STEPS: [f64; 4] = [1.0, 2.0, 5.0, 10.0];
const THOUSAND: f64 = 1_000.0;
const MILLION: f64 = 1_000_000.0;

/// Ticks from zero covering `max` with about `count` steps; the last tick is the axis top.
pub fn nice_ticks(max: f64, count: usize) -> Vec<f64> {
    if max <= 0.0 || !max.is_finite() {
        return vec![0.0, 1.0];
    }
    let raw = max / count.max(1) as f64;
    let power = 10f64.powf(raw.log10().floor());
    let step = NICE_STEPS.iter().map(|s| s * power).find(|s| *s >= raw).unwrap_or(raw);
    let steps = (max / step).ceil() as usize;
    (0..=steps).map(|i| i as f64 * step).collect()
}

/// [`nice_ticks`] for counts: steps of at least one, so no tick falls between two whole
/// numbers (a chart of 2 posts reads 0, 1, 2 rather than 0, 0.5, 1, 1.5, 2).
pub fn count_ticks(max: f64, count: usize) -> Vec<f64> {
    let ticks = nice_ticks(max, count);
    if ticks.get(1).is_some_and(|step| *step >= 1.0) {
        return ticks;
    }
    (0..=max.ceil().max(1.0) as usize).map(|i| i as f64).collect()
}

/// A short label: integers as is, thousands as `k`, millions as `M`, small values with one
/// or two decimals.
pub fn compact(value: f64) -> String {
    let magnitude = value.abs();
    if magnitude >= MILLION {
        format!("{:.1}M", value / MILLION).replace(".0M", "M")
    } else if magnitude >= THOUSAND * 10.0 {
        format!("{:.0}k", value / THOUSAND)
    } else if magnitude >= THOUSAND {
        format!("{:.1}k", value / THOUSAND).replace(".0k", "k")
    } else if (value - value.round()).abs() < 1e-9 {
        format!("{value:.0}")
    } else if magnitude >= 1.0 {
        format!("{value:.1}")
    } else {
        format!("{value:.2}")
    }
}

/// `value` (0–1) as a whole percentage.
pub fn percent(value: f64) -> String {
    format!("{:.0}%", value * 100.0)
}
