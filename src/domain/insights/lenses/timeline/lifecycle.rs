//! Where a concept stands now, from its mentions per period:
//!
//! - **Quiet** (dormant): no mention for the last quarter of the span (at least three periods).
//! - **New**: it first appeared within the recent window (the last sixth, at least two periods).
//! - **Rising**: the recent window averages at least half again its earlier average.
//! - **Fading** (declining): the recent window averages at most half its earlier average.
//! - **Steady**: anything else.

/// A concept's phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    New,
    Rising,
    Steady,
    Declining,
    Dormant,
}

impl Phase {
    pub const ALL: [Self; 5] = [Self::New, Self::Rising, Self::Steady, Self::Declining, Self::Dormant];

    /// Stable key (locale keys `insight.phase.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Rising => "rising",
            Self::Steady => "steady",
            Self::Declining => "declining",
            Self::Dormant => "dormant",
        }
    }
}

const RECENT_SHARE: usize = 6;
const MIN_RECENT: usize = 2;
const DORMANT_SHARE: usize = 4;
const MIN_DORMANT: usize = 3;
const RISE: f64 = 1.5;
const DECLINE: f64 = 0.5;

fn mean(values: &[u32]) -> f64 {
    if values.is_empty() { 0.0 } else { values.iter().map(|&v| f64::from(v)).sum::<f64>() / values.len() as f64 }
}

/// The phase of `counts`, the last mention `quiet_periods` periods before today's.
pub fn phase(counts: &[u32], quiet_periods: usize) -> Phase {
    let n = counts.len();
    let recent = (n / RECENT_SHARE).max(MIN_RECENT).min(n);
    if quiet_periods >= (n / DORMANT_SHARE).max(MIN_DORMANT) {
        return Phase::Dormant;
    }
    let first = counts.iter().position(|&c| c > 0).unwrap_or(n);
    if first >= n - recent {
        return Phase::New;
    }
    let recent_mean = mean(&counts[n - recent..]);
    let earlier_mean = mean(&counts[first..n - recent]);
    if recent_mean > 0.0 && recent_mean >= RISE * earlier_mean {
        Phase::Rising
    } else if recent_mean <= DECLINE * earlier_mean {
        Phase::Declining
    } else {
        Phase::Steady
    }
}
