//! A belief's sparkline: its credence over time on a fixed 0–1 scale (so lines compare
//! across beliefs), painted with the chart kit's strokes and an end dot, over a faint
//! midline at 0.5.

use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::paint::{dot, StrokeBatch};
use gpui_kit::*;

/// The midline's height on the 0–1 scale, and where a lone point sits across.
const MIDLINE: f32 = 0.5;
/// The inset applies at both ends of each axis.
const BOTH_ENDS: f32 = 2.0;

/// Points of `series` (each 0–1) spread over `width` × `height`, from the top-left corner;
/// one point sits in the middle.
pub fn spark_points(series: &[f32], width: f32, height: f32, inset: f32) -> Vec<(f32, f32)> {
    let span = (width - BOTH_ENDS * inset).max(0.0);
    let rise = (height - BOTH_ENDS * inset).max(0.0);
    let step = if series.len() > 1 { span / (series.len() - 1) as f32 } else { 0.0 };
    let start = if series.len() > 1 { inset } else { width * MIDLINE };
    series.iter().enumerate().map(|(index, value)| (start + step * index as f32, inset + rise * (1.0 - value.clamp(0.0, 1.0)))).collect()
}

fn paint_spark(bounds: Bounds<Pixels>, series: &[f32], color: Rgba, theme: &ThemeTokens, window: &mut Window) {
    let (width, height) = (bounds.size.width.as_f32(), bounds.size.height.as_f32());
    let radius = theme.belief_spark_dot.as_f32();
    let (x, y) = (bounds.origin.x.as_f32(), bounds.origin.y.as_f32());
    let shift = |(dx, dy): (f32, f32)| (x + dx, y + dy);
    let mut mid = StrokeBatch::new(theme.border_width_hairline, theme.border_subtle);
    let mid_y = y + radius + (height - BOTH_ENDS * radius) * (1.0 - MIDLINE);
    mid.line((x, mid_y), (x + width, mid_y), window);
    mid.finish(window);
    let points: Vec<(f32, f32)> = spark_points(series, width, height, radius).into_iter().map(shift).collect();
    let mut line = StrokeBatch::new(theme.belief_spark_stroke, color);
    for pair in points.windows(2) {
        line.line(pair[0], pair[1], window);
    }
    line.finish(window);
    if let Some(last) = points.last() {
        dot(window, *last, radius, color, None);
    }
}

/// The sparkline element of `series` in `color`.
pub fn sparkline(series: Vec<f32>, color: Rgba, theme: &ThemeTokens) -> Div {
    let painter = theme.clone();
    let figure = canvas(|_, _, _| {}, move |bounds, (), window, _| paint_spark(bounds, &series, color, &painter, window)).size_full();
    div().w(theme.belief_spark_width).h(theme.belief_spark_height).flex_none().child(figure)
}
