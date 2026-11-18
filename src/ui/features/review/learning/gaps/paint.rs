//! Painting of the gap radar with the chart kit's primitives: grid rings and axes, one
//! translucent polygon per series (the average, the selected concept), and axis labels.

use super::geometry::{label_anchor, outline, point, radar, ring, RINGS};
use crate::domain::learning::gaps::{DimensionScores, DIMENSIONS};
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::paint::{label, label_width, polygon, LabelStyle, StrokeBatch};
use crate::ui::features::insights::chart::palette::faded;
use crate::ui::features::mind_graph::viewport::Frame;
use gpui_kit::*;

const HALF: f32 = 2.0;
/// Edge width of a series, in hairlines.
const EDGE_HAIRLINES: f32 = 2.0;

/// One drawn series.
#[derive(Debug, Clone, PartialEq)]
pub struct RadarSeries {
    pub colour: Rgba,
    pub scores: DimensionScores,
}

fn style(theme: &ThemeTokens) -> LabelStyle {
    LabelStyle { family: theme.font_family_ui.clone().into(), size: theme.font_size_2xs, line_height: theme.font_size_2xs * theme.line_height_body, colour: theme.text_secondary, max_width: theme.insights_label_max_width }
}

fn closed(batch: &mut StrokeBatch, points: &[(f32, f32)], window: &mut Window) {
    points.iter().zip(points.iter().cycle().skip(1)).for_each(|(a, b)| batch.line(*a, *b, window));
}

fn paint_grid(frame: &Frame, theme: &ThemeTokens, window: &mut Window) {
    let (centre, _) = radar(frame);
    let mut grid = StrokeBatch::new(theme.border_width_hairline, theme.border);
    RINGS.iter().for_each(|value| closed(&mut grid, &ring(frame, *value), window));
    (0..DIMENSIONS).for_each(|axis| grid.line(centre, point(frame, axis, 1.0), window));
    grid.finish(window);
}

/// The radar canvas, filling its (square) container.
pub fn radar_canvas(series: Vec<RadarSeries>, labels: Vec<String>, theme: ThemeTokens) -> AnyElement {
    canvas(|_, _, _| (), move |bounds, (), window, cx| {
        let frame = Frame::from(bounds);
        paint_grid(&frame, &theme, window);
        for one in &series {
            let shape = outline(&frame, &one.scores);
            polygon(window, &shape, faded(one.colour, theme.cadence_alpha_2));
            let mut edge = StrokeBatch::new(theme.border_width_hairline * EDGE_HAIRLINES, one.colour);
            closed(&mut edge, &shape, window);
            edge.finish(window);
        }
        let text = style(&theme);
        for (axis, name) in labels.iter().enumerate() {
            let at = label_anchor(&frame, axis);
            let width = label_width(window, name, &text);
            label(window, cx, name, (at.0 - width / HALF, at.1 - text.line_height.as_f32() / HALF), &text);
        }
    })
    .absolute()
    .top_0()
    .left_0()
    .size_full()
    .into_any_element()
}
