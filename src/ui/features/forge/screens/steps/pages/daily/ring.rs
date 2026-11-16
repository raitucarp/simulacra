//! The active-hours clock: a 24-segment ring drawn on a canvas, online hours lit in violet and
//! the chronotype's peak hours among them in gold, the window written in the middle. Clicking
//! a segment moves the start there; Shift-clicking moves the end (like the hour strip). It is a
//! tab stop: Left / Right move the start, Up / Down (or Shift + Left / Right) the end.

use super::clock::{hour_at, label_center, segment};
use crate::agent::schedule::{peak_window, HourWindow, HOURS_PER_DAY};
use crate::domain::agent_config::AgentConfig;
use crate::state::forge::HourField;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::widgets::{hour_keys, ValueCallback};
use gpui_kit::*;
use std::cell::Cell;
use std::rc::Rc;

const RING_ID: &str = "forge-clock-ring";
const ONLINE_KEY: &str = "forge.ui.daily.online";
const ALL_DAY_KEY: &str = "forge.ui.daily.all_day";
const RANGE_SEPARATOR: &str = " – ";
/// Hours written around the inside of the ring, each beside its own segment.
const QUARTERS: [u8; 4] = [0, 6, 12, 18];
const HALF: f32 = 2.0;

fn clock(hour: u8) -> String {
    format!("{hour:02}:00")
}

fn segment_color(hour: u8, active: HourWindow, peak: HourWindow, theme: &ThemeTokens) -> Rgba {
    match (active.contains(hour), peak.contains(hour)) {
        (true, true) => theme.accent_dialectic,
        (true, false) => theme.accent_brand,
        (false, _) => theme.bg_elevated,
    }
}

fn paint_ring(bounds: Bounds<Pixels>, (active, peak): (HourWindow, HourWindow), theme: &ThemeTokens, window: &mut Window) {
    let (width, height) = (bounds.size.width.as_f32(), bounds.size.height.as_f32());
    let center = (bounds.origin.x.as_f32() + width / HALF, bounds.origin.y.as_f32() + height / HALF);
    let outer = width.min(height) / HALF;
    let inner = outer - theme.forge_clock_ring_width.as_f32();
    for hour in 0..HOURS_PER_DAY {
        let points: Vec<Point<Pixels>> = segment(hour, center, (inner, outer)).into_iter().map(|(x, y)| point(px(x), px(y))).collect();
        let mut builder = PathBuilder::fill();
        builder.add_polygon(&points, true);
        if let Ok(path) = builder.build() {
            window.paint_path(path, segment_color(hour, active, peak, theme));
        }
    }
}

/// One hour number centered on the middle of its segment, just inside the ring.
fn hour_label(hour: u8, theme: &ThemeTokens) -> Div {
    let (size, label) = (theme.forge_clock_size.as_f32(), theme.forge_clock_label_size.as_f32());
    let radius = size / HALF - theme.forge_clock_ring_width.as_f32() - theme.spacing_2xs.as_f32() - label / HALF;
    let (x, y) = label_center(hour, (size / HALF, size / HALF), radius);
    div()
        .absolute()
        .left(px(x - label / HALF))
        .top(px(y - label / HALF))
        .size(theme.forge_clock_label_size)
        .flex()
        .items_center()
        .justify_center()
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_muted)
        .child(hour.to_string())
}

/// The hour labels and the window text inside the ring.
fn dial(start: u8, end: u8, theme: &ThemeTokens) -> Vec<Div> {
    let range = if start == end { tr(ALL_DAY_KEY) } else { format!("{}{RANGE_SEPARATOR}{}", clock(start), clock(end)) };
    let inset = theme.forge_clock_ring_width + theme.forge_clock_label_size + theme.spacing_2xs;
    let middle = div()
        .absolute()
        .top(inset)
        .bottom(inset)
        .left(inset)
        .right(inset)
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .text_center()
        .child(div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_xs).text_color(theme.text_primary).child(range))
        .child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(tr(ONLINE_KEY)));
    QUARTERS.into_iter().map(|hour| hour_label(hour, theme)).chain([middle]).collect()
}

/// The clock for `cfg`'s active hours; `on_change` gets `(field, hour)` per click.
pub(super) fn clock_ring(cfg: &AgentConfig, theme: &ThemeTokens, on_change: ValueCallback<(HourField, u8)>) -> Stateful<Div> {
    let windows = (HourWindow::new(cfg.active_hours_start, cfg.active_hours_end), peak_window(cfg.chronotype));
    let center: Rc<Cell<Option<(f32, f32)>>> = Rc::new(Cell::new(None));
    let (slot, paint_theme) = (center.clone(), theme.clone());
    let surface = canvas(
        move |bounds, _, _| slot.set(Some((bounds.center().x.as_f32(), bounds.center().y.as_f32()))),
        move |bounds, (), window, _| paint_ring(bounds, windows, &paint_theme, window),
    )
    .size_full();
    let keys = on_change.clone();
    hour_keys(RING_ID, (cfg.active_hours_start, cfg.active_hours_end), theme, keys)
        .relative()
        .flex_shrink_0()
        .size(theme.forge_clock_size)
        .rounded(theme.radius_full)
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, move |event, window, cx| {
            let Some((cx_, cy_)) = center.get() else { return };
            let hour = hour_at(event.position.x.as_f32() - cx_, event.position.y.as_f32() - cy_);
            let field = if event.modifiers.shift { HourField::ActiveHoursEnd } else { HourField::ActiveHoursStart };
            on_change((field, hour), window, cx);
        })
        .child(surface)
        .children(dial(cfg.active_hours_start, cfg.active_hours_end, theme))
}
