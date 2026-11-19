//! Equal-width histograms with a "nice" bin width (1, 2 or 5 times a power of ten), as many
//! bins as the square-root rule suggests within [`MIN_BINS`]..=[`MAX_BINS`].

pub const MIN_BINS: usize = 5;
pub const MAX_BINS: usize = 24;
const NICE_STEPS: [f64; 3] = [1.0, 2.0, 5.0];

/// Bins starting at `start`, each `width` wide.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Histogram {
    pub start: f64,
    pub width: f64,
    pub counts: Vec<usize>,
    /// Median of the values (0 when empty).
    pub median: f64,
}

impl Histogram {
    /// `[low, high)` of bin `i`.
    pub fn bin(&self, i: usize) -> (f64, f64) {
        let low = self.start + self.width * i as f64;
        (low, low + self.width)
    }

    pub fn total(&self) -> usize {
        self.counts.iter().sum()
    }
}

/// The smallest nice width at least `raw`.
pub fn nice_width(raw: f64) -> f64 {
    if raw <= 0.0 || !raw.is_finite() {
        return 1.0;
    }
    let power = 10f64.powf(raw.log10().floor());
    NICE_STEPS.iter().map(|s| s * power).find(|w| *w >= raw).unwrap_or(10.0 * power)
}

/// The histogram of `values` (empty for no values).
pub fn histogram(values: &[f64]) -> Histogram {
    if values.is_empty() {
        return Histogram::default();
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let (low, high) = (sorted[0], sorted[sorted.len() - 1]);
    let bins = ((values.len() as f64).sqrt().ceil() as usize).clamp(MIN_BINS, MAX_BINS);
    let width = nice_width((high - low) / bins as f64);
    let start = (low / width).floor() * width;
    let count = (((high - start) / width).floor() as usize + 1).max(1);
    let mut counts = vec![0usize; count];
    values.iter().for_each(|v| counts[(((v - start) / width).floor() as usize).min(count - 1)] += 1);
    let median = sorted[sorted.len() / 2];
    Histogram { start, width, counts, median }
}
