//! A kit slider entity kept in keyed element state: values are whole numbers, a soft
//! `ceiling` below the maximum snaps the thumb back (budgets), and the value is reported on
//! release so one drag is one undo step. While dragging, the live value is exposed so the
//! label can follow the thumb. [`keyboard_slider`] makes it reachable and movable by keyboard.

use super::focus_ring::KeyboardStop;
use super::ValueCallback;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::logic::slider_key_value;
use gpui_kit::component::slider::{Slider, SliderEvent, SliderState};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// Keyed state of one slider.
struct SliderSlot {
    slider: Entity<SliderState>,
    dragging: Option<u16>,
    ceiling: u16,
    on_release: Option<ValueCallback<u16>>,
    _subscription: Subscription,
}

impl SliderSlot {
    fn new(slider: Entity<SliderState>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let subscription = cx.subscribe_in(&slider, window, Self::on_event);
        Self { slider, dragging: None, ceiling: u16::MAX, on_release: None, _subscription: subscription }
    }

    fn on_event(&mut self, slider: &Entity<SliderState>, event: &SliderEvent, window: &mut Window, cx: &mut Context<Self>) {
        let (raw, released) = match event {
            SliderEvent::Change(value) => (value.start(), false),
            SliderEvent::Release(value) => (value.start(), true),
        };
        let value = (raw.round().max(0.0) as u16).min(self.ceiling);
        if raw > f32::from(self.ceiling) {
            slider.update(cx, |state, cx| state.set_value(f32::from(value), window, cx));
        }
        self.dragging = (!released).then_some(value);
        cx.notify();
        if released {
            if let Some(on_release) = self.on_release.clone() {
                on_release(value, window, cx);
            }
        }
    }
}

/// Range, step and soft ceiling of a keyed slider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderRange {
    pub min: u16,
    pub max: u16,
    pub step: u16,
    /// Highest value the thumb may stay at (`max` when unrestricted).
    pub ceiling: u16,
}

/// A kit slider over `range` at `value`. The maximum is set before the minimum: the kit
/// clamps its thumb on every bound change, and a minimum above its default maximum (100, e.g.
/// the photo zoom's 1000) would otherwise make that clamp panic.
pub(super) fn fresh_slider(range: SliderRange, value: u16) -> SliderState {
    SliderState::new()
        .max(f32::from(range.max))
        .min(f32::from(range.min))
        .step(f32::from(range.step))
        .default_value(f32::from(value))
}

/// The slider entity for `id` (created on first use) and the live value while dragging.
pub struct SliderBinding {
    pub slider: Entity<SliderState>,
    pub dragging: Option<u16>,
}

/// Binds the slider keyed by `id` to `value`, installing this frame's ceiling and callback.
pub fn keyed_slider(
    id: impl Into<ElementId>,
    value: u16,
    range: SliderRange,
    on_release: ValueCallback<u16>,
    window: &mut Window,
    cx: &mut App,
) -> SliderBinding {
    let slot = window.use_keyed_state(id, cx, |window, cx| {
        let slider = cx.new(|_| fresh_slider(range, value));
        SliderSlot::new(slider, window, cx)
    });
    let (slider, dragging) = slot.update(cx, |slot, _| {
        slot.ceiling = range.ceiling;
        slot.on_release = Some(on_release);
        (slot.slider.clone(), slot.dragging)
    });
    if dragging.is_none() && slider.read(cx).value().start() != f32::from(value) {
        slider.update(cx, |state, cx| state.set_value(f32::from(value), window, cx));
    }
    SliderBinding { slider, dragging }
}

/// The kit slider of `binding` as a tab stop with the theme's focus ring: arrows, Page Up /
/// Down and Home / End move it (up to the soft ceiling), each press one released value.
pub fn keyboard_slider(
    id: impl Into<ElementId>,
    binding: &SliderBinding,
    (value, range): (u16, SliderRange),
    disabled: bool,
    theme: &ThemeTokens,
    on_release: ValueCallback<u16>,
) -> Stateful<Div> {
    let limits = (range.min, range.ceiling.min(range.max), range.step);
    div()
        .id(id)
        .rounded(theme.radius_sm)
        .when(!disabled, |slot| {
            slot.tab_stop_ring(theme).on_key_down(move |event: &KeyDownEvent, window, cx| {
                let next = slider_key_value(value, event.keystroke.key.as_str(), limits);
                if let Some(next) = next.filter(|_| !event.keystroke.modifiers.modified()) {
                    cx.stop_propagation();
                    on_release(next, window, cx);
                }
            })
        })
        .child(Slider::new(&binding.slider).disabled(disabled))
}

#[cfg(test)]
mod tests {
    use super::{fresh_slider, SliderRange};
    use crate::domain::portrait::{ZOOM_MAX, ZOOM_MIN, ZOOM_STEP};

    #[test]
    fn a_range_starting_above_the_kit_default_maximum_builds() {
        let range = SliderRange { min: ZOOM_MIN, max: ZOOM_MAX, step: ZOOM_STEP, ceiling: ZOOM_MAX };
        let slider = fresh_slider(range, ZOOM_MIN);
        assert_eq!((slider.min_value(), slider.max_value()), (f32::from(ZOOM_MIN), f32::from(ZOOM_MAX)));
    }
}
