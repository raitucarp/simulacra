//! The reliability diagram: stated probability across, observed frequency up. The diagonal
//! is perfect calibration; each non-empty bin is a dot (larger for more forecasts) at its
//! mean forecast and observed frequency, with its Wilson interval as a vertical whisker. A
//! square canvas sized to its card, painted with the chart kit's primitives.

use crate::domain::thinking::credence::CalibrationBin;
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::paint::{dot, label, LabelStyle, StrokeBatch};
use crate::ui::features::insights::chart::scale::LinearScale;
use gpui_kit::*;
use std::rc::Rc;

/// Gridlines and their labels (shares of one).
const GRID: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];
/// Square aspect of the diagram.
const SQUARE: f32 = 1.0;
const PERCENT: f64 = 100.0;

/// The plot area inside `bounds` (room for the labels on the left and below).
fn plot(bounds: Bounds<Pixels>, theme: &ThemeTokens) -> (LinearScale, LinearScale) {
    let (x, y) = (bounds.origin.x.as_f32(), bounds.origin.y.as_f32());
    let (w, h) = (bounds.size.width.as_f32(), bounds.size.height.as_f32());
    let (left, bottom, pad) = (theme.insights_axis_width.as_f32(), theme.font_size_2xs.as_f32() * 2.0, theme.spacing_xs.as_f32());
    (LinearScale::new((0.0, 1.0), (x + left, x + w - pad)), LinearScale::new((0.0, 1.0), (y + h - bottom, y + pad)))
}

/// Dot radius of a bin of `count` forecasts, given the largest bin.
fn radius(count: usize, largest: usize, theme: &ThemeTokens) -> f32 {
    let (min, max) = (theme.calibration_dot_min.as_f32(), theme.calibration_dot_max.as_f32());
    min + (max - min) * (count as f32 / largest.max(1) as f32).sqrt()
}

fn paint_grid(across: LinearScale, up: LinearScale, theme: &ThemeTokens, window: &mut Window, cx: &mut App) {
    let mut grid = StrokeBatch::new(theme.border_width_hairline, theme.border_subtle);
    let style = LabelStyle { family: theme.font_family_ui.clone().into(), size: theme.font_size_2xs, line_height: theme.font_size_2xs, colour: theme.text_muted, max_width: theme.insights_axis_width };
    for share in GRID {
        grid.line((across.share(share), up.range.0), (across.share(share), up.range.1), window);
        grid.line((across.range.0, up.share(share)), (across.range.1, up.share(share)), window);
        let text = format!("{:.0}%", share * PERCENT);
        label(window, cx, &text, (across.range.0 - theme.insights_axis_width.as_f32(), up.share(share) - theme.font_size_2xs.as_f32() / 2.0), &style);
        label(window, cx, &text, (across.share(share) - theme.spacing_sm.as_f32(), up.range.0 + theme.spacing_xs.as_f32()), &style);
    }
    grid.finish(window);
    let mut diagonal = StrokeBatch::new(theme.border_width_threadline, theme.border_strong);
    diagonal.line((across.share(0.0), up.share(0.0)), (across.share(1.0), up.share(1.0)), window);
    diagonal.finish(window);
}

fn paint_bins(bins: &[CalibrationBin], across: LinearScale, up: LinearScale, theme: &ThemeTokens, window: &mut Window) {
    let largest = bins.iter().map(|bin| bin.count).max().unwrap_or(1);
    let filled: Vec<&CalibrationBin> = bins.iter().filter(|bin| bin.count > 0).collect();
    let mut whiskers = StrokeBatch::new(theme.border_width_threadline, theme.accent_brand_border);
    for bin in &filled {
        if let Some(interval) = bin.interval {
            let x = across.share(bin.mean_forecast);
            whiskers.line((x, up.share(interval.low)), (x, up.share(interval.high)), window);
        }
    }
    whiskers.finish(window);
    for bin in filled {
        let at = (across.share(bin.mean_forecast), up.share(bin.observed));
        dot(window, at, radius(bin.count, largest, theme), theme.accent_brand, Some((theme.border_width_hairline, theme.bg_card)));
    }
}

/// The diagram of `bins`, a square that fills its container up to the token size.
pub fn reliability_diagram(bins: Vec<CalibrationBin>, theme: &ThemeTokens) -> Div {
    let (bins, paint_theme) = (Rc::new(bins), theme.clone());
    let surface = canvas(
        |_, _, _| {},
        move |bounds, (), window, cx| {
            let (across, up) = plot(bounds, &paint_theme);
            paint_grid(across, up, &paint_theme, window, cx);
            paint_bins(&bins, across, up, &paint_theme, window);
        },
    )
    .size_full();
    div().w_full().max_w(theme.calibration_chart_size).aspect_ratio(SQUARE).child(surface)
}
