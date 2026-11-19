//! Painting of the lane chart on a canvas: the picked row's band, one cell per mentioned
//! period (the sequential ramp by its share of the row's busiest period, culled to the shown
//! range, positions cached per data, range and size), a ring on each row's peak and on the
//! hovered cell, the brush, the concept labels and the date axis. Cached cells are relative to
//! the frame, so scrolling the page reuses them.

use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::cull::Rect;
use crate::ui::features::insights::chart::lanes::{CellCache, CellKey, LaneCell, LaneLayout, PeriodRange};
use crate::ui::features::insights::chart::paint::{label, LabelStyle};
use crate::ui::features::insights::chart::palette::{faded, sequential};
use crate::ui::features::mind_graph::viewport::Frame;
use gpui_kit::*;
use std::cell::Cell;
use std::rc::Rc;

/// One shown concept.
#[derive(Clone)]
pub struct PaintedRow {
    pub label: String,
    pub counts: Vec<u32>,
    pub peak: usize,
    pub quiet: bool,
    pub selected: bool,
}

/// Everything one frame paints.
pub struct LanePaint {
    pub rows: Vec<PaintedRow>,
    pub lanes: Vec<usize>,
    pub range: PeriodRange,
    pub axis: Vec<(usize, String)>,
    pub hover: Option<(usize, usize)>,
    pub brush: Option<(f32, f32)>,
    pub generation: u64,
    pub slot: Rc<Cell<Option<Frame>>>,
    pub cache: Rc<CellCache>,
    pub theme: ThemeTokens,
}

fn bounds((x, y, w, h): Rect) -> Bounds<Pixels> {
    Bounds::new(point(px(x), px(y)), size(px(w), px(h)))
}

fn ring(window: &mut Window, rect: Rect, width: Pixels, colour: Rgba, theme: &ThemeTokens) {
    window.paint_quad(fill(bounds(rect), theme.bg_transparent).corner_radii(theme.radius_xs).border_widths(width).border_color(colour));
}

impl LanePaint {
    pub fn layout(&self, frame: Frame) -> LaneLayout {
        let theme = &self.theme;
        LaneLayout { frame, label_width: theme.insight_lane_label_width.as_f32(), lane_height: theme.insight_lane_height.as_f32(), gap: theme.insight_cell_gap.as_f32(), range: self.range, rows: self.rows.len() }
    }

    fn style(&self, colour: Rgba) -> LabelStyle {
        let theme = &self.theme;
        LabelStyle { family: theme.font_family_ui.clone().into(), size: theme.font_size_2xs, line_height: theme.insight_lane_height, colour, max_width: theme.insight_lane_label_width - theme.spacing_sm }
    }

    fn paint_rows(&self, layout: &LaneLayout, cells: &[LaneCell], window: &mut Window) {
        let theme = &self.theme;
        for row in (0..self.rows.len()).filter(|&row| self.rows[row].selected) {
            let band = (layout.frame.x, layout.frame.y + row as f32 * layout.lane_height, layout.frame.width, layout.lane_height);
            window.paint_quad(fill(bounds(band), theme.accent_brand_subtle).corner_radii(theme.radius_sm));
        }
        for cell in cells {
            let colour = if self.rows[cell.row].quiet { faded(sequential(cell.share, theme), theme.vault_muted_opacity) } else { sequential(cell.share, theme) };
            let (x, y, w, h) = cell.rect;
            window.paint_quad(fill(bounds((x + layout.frame.x, y + layout.frame.y, w, h)), colour).corner_radii(theme.radius_xs));
        }
        for (row, painted) in self.rows.iter().enumerate() {
            if let Some(rect) = layout.cell(row, painted.peak).filter(|_| painted.counts.get(painted.peak).is_some_and(|&n| n > 0)) {
                ring(window, rect, theme.insight_peak_ring_width, theme.text_secondary, theme);
            }
        }
        if let Some(rect) = self.hover.and_then(|(row, period)| layout.cell(row, period)) {
            ring(window, rect, theme.focus_ring_width, theme.text_primary, theme);
        }
        if let Some((a, b)) = self.brush {
            let (plot_left, plot_width) = layout.plot_x();
            let (left, right) = (a.min(b).max(plot_left), a.max(b).min(plot_left + plot_width));
            window.paint_quad(fill(bounds((left, layout.frame.y, (right - left).max(0.0), layout.rows_bottom() - layout.frame.y)), faded(theme.accent_brand, theme.cadence_alpha_2)));
        }
    }

    fn paint_text(&self, layout: &LaneLayout, window: &mut Window, cx: &mut App) {
        let theme = &self.theme;
        for (row, painted) in self.rows.iter().enumerate() {
            let colour = if painted.selected { theme.text_primary } else if painted.quiet { theme.text_muted } else { theme.text_secondary };
            label(window, cx, &painted.label, (layout.frame.x, layout.frame.y + row as f32 * layout.lane_height), &self.style(colour));
        }
        let axis = LabelStyle { max_width: theme.insight_lane_label_width, line_height: theme.insight_axis_height, ..self.style(theme.text_muted) };
        for (period, text) in &self.axis {
            let x = layout.plot_x().0 + (period - layout.range.0) as f32 * layout.period_width();
            label(window, cx, text, (x, layout.rows_bottom()), &axis);
        }
    }

    /// The canvas element.
    pub fn into_canvas(self) -> AnyElement {
        let slot = Rc::clone(&self.slot);
        canvas(move |bounds, _, _| slot.set(Some(Frame::from(bounds))), move |bounds, (), window, cx| {
            let frame = Frame::from(bounds);
            let layout = self.layout(frame);
            let key: CellKey = (self.generation, self.range, self.lanes.clone(), (frame.width as i32, frame.height as i32));
            let local = self.layout(Frame { x: 0.0, y: 0.0, ..frame });
            let cells = self.cache.get(&key, || local.cells(&self.rows.iter().map(|r| r.counts.as_slice()).collect::<Vec<_>>()));
            self.paint_rows(&layout, &cells, window);
            self.paint_text(&layout, window, cx);
        })
        .size_full()
        .into_any_element()
    }
}
