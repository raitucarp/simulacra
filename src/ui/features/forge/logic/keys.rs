//! Keyboard control of the forge's sliders and active-hours pickers.

use crate::state::forge::fields::MAX_HOUR;
use crate::state::forge::HourField;

/// Notches one Page Up / Page Down moves a slider.
pub const PAGE_NOTCHES: u16 = 10;
const LEFT: &str = "left";
const RIGHT: &str = "right";
const UP: &str = "up";
const DOWN: &str = "down";
const PAGE_UP: &str = "pageup";
const PAGE_DOWN: &str = "pagedown";
const HOME: &str = "home";
const END: &str = "end";

/// The value of a slider over `min..=max` moving by `step` after pressing `key`, when the
/// key moves it: arrows one notch, Page Up / Down [`PAGE_NOTCHES`], Home / End to an end.
pub fn slider_key_value(value: u16, key: &str, (min, max, step): (u16, u16, u16)) -> Option<u16> {
    let page = step.saturating_mul(PAGE_NOTCHES);
    let next = match key {
        LEFT | DOWN => value.saturating_sub(step),
        RIGHT | UP => value.saturating_add(step),
        PAGE_DOWN => value.saturating_sub(page),
        PAGE_UP => value.saturating_add(page),
        HOME => min,
        END => max,
        _ => return None,
    }
    .clamp(min, max.max(min));
    (next != value).then_some(next)
}

/// The hour after `hour`, wrapping past midnight.
fn later(hour: u8) -> u8 {
    if hour >= MAX_HOUR { 0 } else { hour + 1 }
}

/// The hour before `hour`, wrapping past midnight.
fn earlier(hour: u8) -> u8 {
    if hour == 0 { MAX_HOUR } else { hour - 1 }
}

/// The active-hours edit for `key`: Left / Right move the start an hour (the end with
/// Shift held), Up / Down move the end.
pub fn hour_key_change(key: &str, shift: bool, (start, end): (u8, u8)) -> Option<(HourField, u8)> {
    let start_field = if shift { HourField::ActiveHoursEnd } else { HourField::ActiveHoursStart };
    let start_value = if shift { end } else { start };
    match key {
        LEFT => Some((start_field, earlier(start_value))),
        RIGHT => Some((start_field, later(start_value))),
        DOWN => Some((HourField::ActiveHoursEnd, earlier(end))),
        UP => Some((HourField::ActiveHoursEnd, later(end))),
        _ => None,
    }
}
