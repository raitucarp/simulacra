//! Painting of the rhythm dial and the trait radar.

use super::geometry::{axes, dial, hours, radar, radar_point, sector, TRAIT_MAX};
use super::HOUR_MARKS;
use crate::domain::insights::activity::{HOURS, WEEKDAYS};
use crate::domain::insights::facts::TRAIT_AXES;
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::paint::{label, label_width, polygon, LabelStyle, StrokeBatch};
use crate::ui::features::insights::chart::palette::{faded, sequential, series};
use crate::ui::features::insights::chart::ticks::compact;
use crate::ui::features::mind_graph::viewport::Frame;
use gpui_kit::*;
use std::cell::Cell;
use std::rc::Rc;

/// Grid rings of the radar, as trait values.
const RADAR_RINGS: [f32; 4] = [25.0, 50.0, 75.0, 100.0];
const HALF: f32 = 2.0;

fn style(theme: &ThemeTokens) -> LabelStyle {
    LabelStyle { family: theme.font_family_ui.clone().into(), size: theme.font_size_2xs, line_height: theme.font_size_2xs * theme.line_height_body, colour: theme.text_muted, max_width: theme.insights_label_max_width }
}

/// Paints `text` centred on `at`.
fn centred(window: &mut Window, cx: &mut App, text: &str, at: (f32, f32), style: &LabelStyle) {
    let width = label_width(window, text, style);
    label(window, cx, text, (at.0 - width / HALF, at.1 - style.line_height.as_f32() / HALF), style);
}

fn surface(prepaint: impl Fn(Bounds<Pixels>) + 'static, paint: impl Fn(Frame, &mut Window, &mut App) + 'static) -> AnyElement {
    canvas(move |bounds, _, _| prepaint(bounds), move |bounds, (), window, cx| paint(Frame::from(bounds), window, cx)).absolute().top_0().left_0().size_full().into_any_element()
}

/// The weekly rhythm dial.
pub fn rhythm_canvas(rhythm: [[usize; HOURS]; WEEKDAYS], hover: Option<(usize, usize)>, slot: Rc<Cell<Option<Frame>>>, theme: ThemeTokens) -> AnyElement {
    let max = rhythm.iter().flatten().copied().max().unwrap_or(0).max(1) as f32;
    let total: usize = rhythm.iter().flatten().sum();
    surface(move |bounds| slot.set(Some(Frame::from(bounds))), move |frame, window, cx| {
        let d = dial(&frame);
        let mut seams = StrokeBatch::new(theme.border_width_hairline, theme.bg_card);
        for (day, row) in rhythm.iter().enumerate() {
            for (hour, &count) in row.iter().enumerate() {
                let cell = sector(&d, day, hour);
                polygon(window, &cell, sequential(count as f32 / max, &theme));
                cell.iter().zip(cell.iter().cycle().skip(1)).for_each(|(a, b)| seams.line(*a, *b, window));
            }
        }
        seams.finish(window);
        if let Some((day, hour)) = hover {
            let mut ring = StrokeBatch::new(theme.focus_ring_width, theme.text_primary);
            let outline = sector(&d, day, hour);
            outline.iter().zip(outline.iter().cycle().skip(1)).for_each(|(a, b)| ring.line(*a, *b, window));
            ring.finish(window);
        }
        let text = style(&theme);
        for hour in HOUR_MARKS {
            let at = hours(&d).point(hour as f32, d.outer + (frame.width.min(frame.height) / HALF - d.outer) / HALF);
            centred(window, cx, &format!("{hour:02}"), at, &text);
        }
        let middle = hover.map_or(total, |(day, hour)| rhythm[day][hour]);
        centred(window, cx, &compact(middle as f64), d.centre, &LabelStyle { colour: theme.text_secondary, size: theme.font_size_sm, ..style(&theme) });
    })
}

/// The trait radar: grid rings, axes with labels, one translucent polygon per shown agent.
pub fn radar_canvas(agents: Vec<(usize, [u8; TRAIT_AXES])>, labels: Vec<String>, axis: Option<usize>, slot: Rc<Cell<Option<Frame>>>, theme: ThemeTokens) -> AnyElement {
    surface(move |bounds| slot.set(Some(Frame::from(bounds))), move |frame, window, cx| {
        let (centre, radius) = radar(&frame);
        let mut grid = StrokeBatch::new(theme.border_width_hairline, theme.border);
        for value in RADAR_RINGS {
            let ring: Vec<(f32, f32)> = (0..=TRAIT_AXES).map(|a| radar_point(&frame, a % TRAIT_AXES, value)).collect();
            ring.windows(2).for_each(|w| grid.line(w[0], w[1], window));
        }
        (0..TRAIT_AXES).for_each(|a| grid.line(centre, radar_point(&frame, a, TRAIT_MAX), window));
        grid.finish(window);
        for (series_index, values) in &agents {
            let colour = series(*series_index, &theme);
            let shape: Vec<(f32, f32)> = values.iter().enumerate().map(|(a, v)| radar_point(&frame, a, f32::from(*v))).collect();
            polygon(window, &shape, faded(colour, theme.cadence_alpha_1));
            let mut edge = StrokeBatch::new(theme.insights_edge_width * HALF, colour);
            shape.iter().zip(shape.iter().cycle().skip(1)).for_each(|(a, b)| edge.line(*a, *b, window));
            edge.finish(window);
        }
        for (a, text) in labels.iter().enumerate() {
            let colour = if axis == Some(a) { theme.text_primary } else { theme.text_muted };
            let at = axes(centre).point(a as f32, radius + (frame.width.min(frame.height) / HALF - radius) / HALF);
            centred(window, cx, text, at, &LabelStyle { colour: colour, ..style(&theme) });
        }
    })
}
