//! Splitter appearance: gpui-kit's own divider, plus a hit band over the splitter that
//! restores the split's default sizes on double-click.

use super::store::reset_panel_layout;
use gpui_kit::component::resizable::resize_handle_appearance;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use gpui_kit::base::ResizeHandleRenderer;
use std::rc::Rc;

/// Clicks in a row that count as a double-click.
const DOUBLE_CLICK_COUNT: usize = 2;

/// The kit divider of layout `id`, double-clickable across the splitter's whole band
/// (`hit_padding` on each side of the hairline).
pub(super) fn reset_on_double_click(id: SharedString, hit_padding: Pixels) -> ResizeHandleRenderer {
    let kit = resize_handle_appearance();
    Rc::new(move |handle, window, cx| {
        let line = kit(handle, window, cx)?;
        let horizontal = handle.axis() == Axis::Horizontal;
        let id = id.clone();
        let band = div()
            .absolute()
            .map(|band| match horizontal {
                true => band.top_0().bottom_0().left(-hit_padding).right(-hit_padding),
                false => band.left_0().right_0().top(-hit_padding).bottom(-hit_padding),
            })
            .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                if event.click_count >= DOUBLE_CLICK_COUNT {
                    reset_panel_layout(&id, cx);
                    window.refresh();
                }
            });
        let frame = div().relative().flex_none().flex().map(|frame| if horizontal { frame.h_full() } else { frame.w_full() });
        Some(frame.child(line).child(band).into_any_element())
    })
}
