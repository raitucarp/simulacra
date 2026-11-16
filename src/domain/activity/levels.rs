//! Quantile-based intensity levels for per-day counts.

use super::MAX_ACTIVITY_LEVEL;

/// Quantiles of the active (non-zero) days separating levels 1, 2, 3 and 4.
const LEVEL_QUANTILES: [f64; 3] = [0.25, 0.5, 0.75];

/// Nearest-rank quantile `q` of an ascending, non-empty slice.
fn nearest_rank(sorted: &[u32], q: f64) -> u32 {
    let rank = (q * sorted.len() as f64).ceil() as usize;
    sorted[rank.clamp(1, sorted.len()) - 1]
}

/// Level of one day: 0 when empty, the top level for the busiest day(s), otherwise
/// one plus the number of quantile thresholds the count exceeds.
pub(super) fn level_for(count: u32, max: u32, thresholds: &[u32]) -> u8 {
    if count == 0 {
        0
    } else if count >= max {
        MAX_ACTIVITY_LEVEL
    } else {
        let above = thresholds.iter().filter(|&&t| count > t).count() as u8;
        (1 + above).min(MAX_ACTIVITY_LEVEL)
    }
}

/// Levels (`0..=MAX_ACTIVITY_LEVEL`) for every count, from quartiles of the active days.
pub(super) fn quantile_levels(counts: &[u32]) -> Vec<u8> {
    let mut active: Vec<u32> = counts.iter().copied().filter(|&c| c > 0).collect();
    active.sort_unstable();
    let Some(&max) = active.last() else {
        return vec![0; counts.len()];
    };
    let thresholds = LEVEL_QUANTILES.map(|q| nearest_rank(&active, q));
    counts.iter().map(|&c| level_for(c, max, &thresholds)).collect()
}
