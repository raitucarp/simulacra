//! A two- or three-set Venn diagram: circles in a fixed symmetric arrangement (the regions are
//! labelled with counts rather than drawn to scale), the region under the pointer by circle
//! membership, and the painting.

use crate::domain::insights::sets::Region;
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::paint::{dot, label, label_width, LabelStyle};
use crate::ui::features::insights::chart::palette::{faded, series};
use crate::ui::features::insights::chart::ticks::compact;
use crate::ui::features::mind_graph::viewport::Frame;
use gpui_kit::*;
use std::cell::Cell;
use std::f32::consts::{FRAC_PI_2, TAU};
use std::rc::Rc;

/// Circle radius as a share of the frame's half height.
const RADIUS_SHARE: f32 = 0.58;
/// Distance of the circle centres from the middle, as a share of the radius.
const SPREAD: f32 = 0.62;
/// How far out from the middle a shared region's count sits, as a share of the spread.
const LABEL_REACH: f32 = 0.95;
/// How far beyond its circle's centre a single set's count sits, in radii.
const OUTWARD: f32 = 0.45;
const HALF: f32 = 2.0;
/// Width the arrangement needs, in radii (two circles side by side plus a margin).
const WIDTH_IN_RADII: f32 = 3.6;

/// Circles `(centre, radius)` of `count` (2 or 3) sets in `frame`.
pub fn circles(frame: &Frame, count: usize) -> Vec<((f32, f32), f32)> {
    let radius = (frame.height / HALF * RADIUS_SHARE).min(frame.width / WIDTH_IN_RADII);
    let middle = (frame.x + frame.width / HALF, frame.y + frame.height / HALF);
    (0..count)
        .map(|i| {
            let angle = if count == 2 { i as f32 * TAU / 2.0 } else { i as f32 * TAU / count as f32 - FRAC_PI_2 };
            ((middle.0 + radius * SPREAD * angle.cos(), middle.1 + radius * SPREAD * angle.sin()), radius)
        })
        .collect()
}

/// The region (bit mask) under `at`, if inside any circle.
pub fn region_at(frame: &Frame, count: usize, at: (f32, f32)) -> Option<u32> {
    let mask = circles(frame, count).iter().enumerate().fold(0u32, |mask, (i, ((x, y), r))| {
        let inside = (at.0 - x).powi(2) + (at.1 - y).powi(2) <= r * r;
        if inside { mask | 1 << i } else { mask }
    });
    (mask != 0).then_some(mask)
}

/// Where the count of region `mask` is written: towards the mean direction of its circles,
/// away from the circles it excludes.
pub fn region_anchor(frame: &Frame, count: usize, mask: u32) -> (f32, f32) {
    let all = circles(frame, count);
    let middle = (frame.x + frame.width / HALF, frame.y + frame.height / HALF);
    let (mut dx, mut dy) = (0.0, 0.0);
    for (i, ((x, y), _)) in all.iter().enumerate() {
        let sign = if mask & (1 << i) != 0 { 1.0 } else { -0.5 };
        (dx, dy) = (dx + (x - middle.0) * sign, dy + (y - middle.1) * sign);
    }
    let length = (dx * dx + dy * dy).sqrt();
    let degree = mask.count_ones() as usize;
    let reach = all[0].1 * match degree {
        d if d == count => 0.0,
        1 => SPREAD + OUTWARD,
        _ => SPREAD * LABEL_REACH,
    };
    if length < f32::EPSILON { middle } else { (middle.0 + dx / length * reach, middle.1 + dy / length * reach) }
}

/// The Venn canvas.
pub fn venn_canvas(count: usize, regions: Vec<Region>, hover: Option<u32>, slot: Rc<Cell<Option<Frame>>>, theme: ThemeTokens) -> AnyElement {
    canvas(move |bounds, _, _| slot.set(Some(Frame::from(bounds))), move |bounds, (), window, cx| {
        let frame = Frame::from(bounds);
        for (i, (centre, radius)) in circles(&frame, count).into_iter().enumerate() {
            let colour = series(i, &theme);
            dot(window, centre, radius, faded(colour, theme.cadence_alpha_1), Some((theme.border_width_threadline, colour)));
        }
        let style = |colour: Rgba, size: Pixels| LabelStyle { family: theme.font_family_ui.clone().into(), size, line_height: size * theme.line_height_body, colour, max_width: theme.insights_label_max_width };
        for region in &regions {
            let colour = if hover == Some(region.mask) { theme.accent_brand } else { theme.text_primary };
            let text = compact(region.members.len() as f64);
            let at = region_anchor(&frame, count, region.mask);
            let s = style(colour, theme.font_size_md);
            let width = label_width(window, &text, &s);
            label(window, cx, &text, (at.0 - width / HALF, at.1 - s.line_height.as_f32() / HALF), &s);
        }
    })
    .absolute().top_0().left_0().size_full().into_any_element()
}
