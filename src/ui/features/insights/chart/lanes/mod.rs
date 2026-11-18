//! sb F3: geometry of a lane chart (one row per series, one column per period of the shown
//! range, a label column on the left and a date axis under the rows). Pure, so hit testing,
//! brushing and culling are unit-tested; cells outside the range are never produced, and the
//! cells are cached per data, range and frame size ([`CellCache`]).

#[cfg(test)]
mod tests;

use super::cache::GeometryCache;
use super::cull::Rect;
use crate::ui::features::mind_graph::viewport::Frame;

/// An inclusive range of period indices.
pub type PeriodRange = (usize, usize);
/// What cached cells depend on: data generation, range, shown series and frame size (cache
/// cells laid out in a frame at the origin and offset them when painting).
pub type CellKey = (u64, PeriodRange, Vec<usize>, (i32, i32));
pub type CellCache = GeometryCache<CellKey, Vec<LaneCell>>;

const HALF: f32 = 2.0;

/// The chart laid out in a frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LaneLayout {
    pub frame: Frame,
    pub label_width: f32,
    pub lane_height: f32,
    pub gap: f32,
    pub range: PeriodRange,
    pub rows: usize,
}

/// One drawn cell: its row, period, rectangle and share of the row's busiest period.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LaneCell {
    pub row: usize,
    pub period: usize,
    pub rect: Rect,
    pub share: f32,
}

impl LaneLayout {
    /// Periods shown.
    pub fn periods(&self) -> usize {
        self.range.1.saturating_sub(self.range.0) + 1
    }

    /// Left edge and width of the plot.
    pub fn plot_x(&self) -> (f32, f32) {
        (self.frame.x + self.label_width, (self.frame.width - self.label_width).max(0.0))
    }

    /// Width of one period column.
    pub fn period_width(&self) -> f32 {
        self.plot_x().1 / self.periods() as f32
    }

    /// Bottom of the last row (where the axis starts).
    pub fn rows_bottom(&self) -> f32 {
        self.frame.y + self.rows as f32 * self.lane_height
    }

    /// The cell of `row` in `period`, or `None` when the period is outside the range.
    pub fn cell(&self, row: usize, period: usize) -> Option<Rect> {
        let (from, to) = self.range;
        if period < from || period > to || row >= self.rows {
            return None;
        }
        let (left, _) = self.plot_x();
        let width = self.period_width();
        let x = left + (period - from) as f32 * width;
        let y = self.frame.y + row as f32 * self.lane_height;
        Some((x + self.gap / HALF, y + self.gap, (width - self.gap).max(self.gap), self.lane_height - self.gap * HALF))
    }

    /// The row under window y, if any.
    pub fn row_at(&self, y: f32) -> Option<usize> {
        let offset = y - self.frame.y;
        (offset >= 0.0 && y < self.rows_bottom()).then(|| (offset / self.lane_height) as usize)
    }

    /// The period under window x, clamped into the range.
    pub fn period_at(&self, x: f32) -> usize {
        let (left, _) = self.plot_x();
        let column = ((x - left) / self.period_width().max(f32::EPSILON)).floor().max(0.0) as usize;
        (self.range.0 + column).min(self.range.1)
    }

    /// Whether window x lies over the plot.
    pub fn over_plot(&self, x: f32) -> bool {
        let (left, width) = self.plot_x();
        x >= left && x <= left + width
    }

    /// The periods a drag from `a` to `b` covers.
    pub fn brush_range(&self, a: f32, b: f32) -> PeriodRange {
        let (p, q) = (self.period_at(a), self.period_at(b));
        (p.min(q), p.max(q))
    }

    /// The cells of `rows` (each row's counts over every period), culled to the range; empty
    /// periods are left out.
    pub fn cells(&self, rows: &[&[u32]]) -> Vec<LaneCell> {
        rows.iter().enumerate().flat_map(|(row, counts)| {
            let (from, to) = self.range;
            let busiest = counts.iter().skip(from).take(to + 1 - from).copied().max().unwrap_or(0).max(1) as f32;
            (from..=to.min(counts.len().saturating_sub(1))).filter(move |&p| counts[p] > 0).filter_map(move |p| self.cell(row, p).map(|rect| LaneCell { row, period: p, rect, share: counts[p] as f32 / busiest }))
        }).collect()
    }

    /// Periods to label on the axis: every `stride`-th, so labels `label_width` wide never overlap.
    pub fn axis_periods(&self, label_width: f32) -> Vec<usize> {
        let stride = (label_width / self.period_width().max(f32::EPSILON)).ceil().max(1.0) as usize;
        (self.range.0..=self.range.1).step_by(stride).collect()
    }
}
