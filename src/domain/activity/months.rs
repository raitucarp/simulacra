//! Month labels above the heatmap's week columns.

use super::{ActivityHeatmap, HEATMAP_WEEKS};

/// Minimum number of week columns between two month labels so they never overlap.
const MIN_LABEL_GAP_WEEKS: usize = 3;

/// A month name anchored above the first week column whose Sunday falls in that month.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthLabel {
    /// Week column index.
    pub week: usize,
    /// Calendar month, `1..=12`.
    pub month: i8,
}

impl ActivityHeatmap {
    /// Month labels, left to right; the partial leading month is dropped when crowded.
    pub fn month_labels(&self) -> Vec<MonthLabel> {
        let mut labels: Vec<MonthLabel> = Vec::new();
        let mut previous_month = None;
        for week in 0..HEATMAP_WEEKS {
            let month = self.date_at(week, 0).month();
            if previous_month != Some(month) {
                labels.push(MonthLabel { week, month });
            }
            previous_month = Some(month);
        }
        if labels.len() > 1 && labels[1].week - labels[0].week < MIN_LABEL_GAP_WEEKS {
            labels.remove(0);
        }
        labels
    }
}
