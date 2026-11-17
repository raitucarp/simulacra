//! Measuring an element's width for layouts that need it at render time (cell sizes from the
//! available width). The width laid out in one frame is read back in the next; a change
//! re-renders the view that owns the measurement.

use gpui_kit::*;

/// The width the element keyed `id` had in the last frame (`None` before its first layout),
/// and the probe to place inside it (the element must be `relative`).
pub fn measured_width(id: impl Into<ElementId>, window: &mut Window, cx: &mut App) -> (Option<Pixels>, AnyElement) {
    let state = window.use_keyed_state(id, cx, |_, _| None::<Pixels>);
    let width = *state.read(cx);
    let probe = canvas(
        move |bounds, _, cx| {
            let measured = Some(bounds.size.width);
            state.update(cx, |current, cx| {
                if *current != measured {
                    *current = measured;
                    cx.notify();
                }
            });
        },
        |_, _, _, _| {},
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full();
    (width, probe.into_any_element())
}
