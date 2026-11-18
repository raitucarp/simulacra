//! The schema diagram: one object per kind (thought in the middle, the others around it),
//! sized by count, and one labelled arrow per relation type. Relations from an object to
//! itself are loops stacked above it; two opposite relations between a pair bend apart.

use crate::domain::insights::category::{ObjectKind, SchemaArrow};
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::paint::{dot, label, label_width, polygon, LabelStyle, StrokeBatch};
use crate::ui::features::insights::chart::palette::{faded, node_kind_colour};
use crate::ui::features::insights::chart::ticks::compact;
use crate::domain::graph::NodeKind;
use crate::ui::features::mind_graph::viewport::Frame;
use gpui_kit::*;
use std::cell::Cell;
use std::rc::Rc;

/// Object places as shares of the frame, in [`ObjectKind::ALL`] order.
const PLACES: [(f32, f32); 4] = [(0.5, 0.5), (0.86, 0.5), (0.5, 0.84), (0.14, 0.5)];
/// Sideways bend of an arrow, as a share of its length.
const BEND: f32 = 0.18;
/// Loop height steps above an object, in object radii.
const LOOP_STEP: f32 = 2.2;
const ARROW_HEAD: f32 = 7.0;
const HALF: f32 = 2.0;

fn kind_of(object: ObjectKind) -> NodeKind {
    match object {
        ObjectKind::Thought => NodeKind::Post,
        ObjectKind::Document => NodeKind::Document,
        ObjectKind::Concept => NodeKind::Hashtag,
        ObjectKind::Person => NodeKind::Profile,
    }
}

fn index(object: ObjectKind) -> usize {
    ObjectKind::ALL.iter().position(|o| *o == object).unwrap_or_default()
}

/// Centre of each object in `frame`.
pub fn places(frame: &Frame) -> Vec<(f32, f32)> {
    PLACES.iter().map(|(x, y)| (frame.x + frame.width * x, frame.y + frame.height * y)).collect()
}

/// The object under `at`, within `reach` pixels.
pub fn object_at(frame: &Frame, at: (f32, f32), reach: f32) -> Option<usize> {
    places(frame).iter().position(|p| (p.0 - at.0).hypot(p.1 - at.1) <= reach)
}

/// Point on the quadratic curve `a`–`b` with control `c` at parameter `t`.
fn on_curve(a: (f32, f32), b: (f32, f32), c: (f32, f32), t: f32) -> (f32, f32) {
    let u = 1.0 - t;
    (u * u * a.0 + 2.0 * u * t * c.0 + t * t * b.0, u * u * a.1 + 2.0 * u * t * c.1 + t * t * b.1)
}

fn arrow_head(window: &mut Window, tip: (f32, f32), from: (f32, f32), colour: Rgba) {
    let (dx, dy) = (tip.0 - from.0, tip.1 - from.1);
    let length = dx.hypot(dy).max(f32::EPSILON);
    let (ux, uy) = (dx / length, dy / length);
    let back = (tip.0 - ux * ARROW_HEAD, tip.1 - uy * ARROW_HEAD);
    polygon(window, &[tip, (back.0 - uy * ARROW_HEAD / HALF, back.1 + ux * ARROW_HEAD / HALF), (back.0 + uy * ARROW_HEAD / HALF, back.1 - ux * ARROW_HEAD / HALF)], colour);
}

/// Curve of an arrow between two distinct objects (start, end, control), shortened to the rims.
fn arrow_curve(from: (f32, f32), to: (f32, f32), rim: f32) -> ((f32, f32), (f32, f32), (f32, f32)) {
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
    let length = dx.hypot(dy).max(f32::EPSILON);
    let (ux, uy) = (dx / length, dy / length);
    let control = ((from.0 + to.0) / HALF - dy * BEND, (from.1 + to.1) / HALF + dx * BEND);
    ((from.0 + ux * rim, from.1 + uy * rim), (to.0 - ux * rim, to.1 - uy * rim), control)
}

/// The diagram canvas; `counts` are objects per kind, `focus` the hovered object.
pub fn diagram_canvas(arrows: Vec<SchemaArrow>, names: Vec<String>, counts: [usize; 4], focus: Option<usize>, slot: Rc<Cell<Option<Frame>>>, theme: ThemeTokens) -> AnyElement {
    canvas(move |bounds, _, _| slot.set(Some(Frame::from(bounds))), move |bounds, (), window, cx| {
        let frame = Frame::from(bounds);
        let at = places(&frame);
        let rim = theme.insights_node_radius_max.as_f32() * HALF;
        let style = LabelStyle { family: theme.font_family_ui.clone().into(), size: theme.font_size_2xs, line_height: theme.font_size_2xs * theme.line_height_body, colour: theme.text_secondary, max_width: theme.insights_label_max_width };
        let mut loops = [0usize; 4];
        for (k, arrow) in arrows.iter().enumerate() {
            let (s, t) = (index(arrow.source), index(arrow.target));
            let lit = focus.is_none_or(|f| f == s || f == t);
            let colour = if lit { theme.accent_brand } else { faded(theme.graph_edge, theme.graph_dim_opacity) };
            let (start, end, control) = if s == t {
                loops[s] += 1;
                let lift = rim * LOOP_STEP * loops[s] as f32;
                ((at[s].0 - rim, at[s].1 - rim / HALF), (at[s].0 + rim, at[s].1 - rim / HALF), (at[s].0, at[s].1 - rim - lift * HALF))
            } else {
                arrow_curve(at[s], at[t], rim)
            };
            let mut stroke = StrokeBatch::new(theme.insights_edge_width * HALF, colour);
            stroke.curve(start, end, control, window);
            stroke.finish(window);
            arrow_head(window, end, on_curve(start, end, control, 0.85), colour);
            let mid = on_curve(start, end, control, 0.5);
            let text = format!("{} · {}", names[k], compact(arrow.count as f64));
            let width = label_width(window, &text, &style);
            label(window, cx, &text, (mid.0 - width / HALF, mid.1 - style.line_height.as_f32()), &LabelStyle { colour: if lit { theme.text_primary } else { theme.text_muted }, ..style.clone() });
        }
        paint_objects(window, cx, &at, counts, focus, rim, &theme);
    })
    .absolute().top_0().left_0().size_full().into_any_element()
}

fn paint_objects(window: &mut Window, cx: &mut App, at: &[(f32, f32)], counts: [usize; 4], focus: Option<usize>, rim: f32, theme: &ThemeTokens) {
    let style = LabelStyle { family: theme.font_family_ui.clone().into(), size: theme.font_size_xs, line_height: theme.font_size_xs * theme.line_height_body, colour: theme.text_primary, max_width: theme.insights_label_max_width };
    for (i, object) in ObjectKind::ALL.iter().enumerate() {
        let colour = node_kind_colour(kind_of(*object), theme);
        let ring = (focus == Some(i)).then_some((theme.border_width_threadline, theme.text_primary));
        dot(window, at[i], rim / HALF * 1.4, colour, ring);
        let text = format!("{} · {}", super::super::common::tr(&format!("insights.object.{}", object.key())), counts[i]);
        let width = label_width(window, &text, &style);
        label(window, cx, &text, (at[i].0 - width / HALF, at[i].1 + rim), &style);
    }
}
