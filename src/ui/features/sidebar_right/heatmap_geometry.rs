//! Pure sizing of the cadence grid: square cells sized from the available width (clamped to
//! the token range), as many trailing weeks as fit, and month labels that never overlap.

use crate::domain::activity::{MonthLabel, HEATMAP_WEEKS};
use gpui_kit::Pixels;

/// Week columns shown and the side of one square cell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridGeometry {
    /// Trailing weeks shown (the most recent ones), `1..=HEATMAP_WEEKS`.
    pub weeks: usize,
    pub cell: Pixels,
    pub gap: Pixels,
}

/// A month label placed over the shown weeks: its column and the room it may use.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacedLabel {
    pub month: u32,
    /// Column among the shown weeks.
    pub column: usize,
    /// Width up to the next label (or the grid's end).
    pub width: Pixels,
}

impl GridGeometry {
    /// Fits the year into `width`: every week when cells of at least `min` fit (cells grow up
    /// to `max`), otherwise only the most recent weeks that fit at `min`.
    pub fn fit(width: Pixels, gap: Pixels, (min, max): (Pixels, Pixels)) -> Self {
        let width = width.max(Pixels::ZERO);
        let cell_for = |weeks: usize| (width - gap * (weeks as f32 - 1.0)) / weeks as f32;
        let all = cell_for(HEATMAP_WEEKS);
        if all >= min {
            return Self { weeks: HEATMAP_WEEKS, cell: all.min(max), gap };
        }
        let fitting = ((width + gap) / (min + gap)).floor() as usize;
        let weeks = fitting.clamp(1, HEATMAP_WEEKS);
        Self { weeks, cell: cell_for(weeks).clamp(min, max), gap }
    }

    /// Index of the first shown week in the full year.
    pub fn first_week(&self) -> usize {
        HEATMAP_WEEKS - self.weeks
    }

    /// Distance from one column to the next.
    pub fn step(&self) -> Pixels {
        self.cell + self.gap
    }

    /// Labels of the shown weeks that fit `label_width` without overlapping; when crowded the
    /// later month wins (the leading partial month is dropped first).
    pub fn place_labels(&self, labels: &[MonthLabel], label_width: Pixels) -> Vec<PlacedLabel> {
        let span = (label_width / self.step()).ceil().max(1.0) as usize;
        let first = self.first_week();
        let mut next_start = self.weeks;
        let mut placed: Vec<PlacedLabel> = labels
            .iter()
            .rev()
            .filter(|label| label.week >= first)
            .filter_map(|label| {
                let column = label.week - first;
                (column + span <= next_start).then(|| {
                    let width = self.step() * (next_start - column) as f32 - self.gap;
                    next_start = column;
                    PlacedLabel { month: label.month, column, width }
                })
            })
            .collect();
        placed.reverse();
        placed
    }
}
