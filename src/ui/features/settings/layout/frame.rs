//! The window frame: a scrim over the app and the dialog, sized to fit inside the window
//! (never larger than the window minus an inset), holding the resizable rail | content split
//! (a fixed icon rail in a compact window). Escape closes it.

use super::super::callbacks::PlainAction;
use super::super::consts::{BACKDROP_ID, DIALOG_ID, KEY_ESCAPE, SPLIT_ID};
use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::components::{persisted_h_resizable, SplitPane};
use gpui_kit::*;

/// `(width, height)` of the dialog in a `viewport`-sized window (pure).
pub(in crate::ui::features::settings) fn dialog_size(viewport: Size<Pixels>, theme: &ThemeTokens) -> (Pixels, Pixels) {
    let inset = theme.settings_modal_inset + theme.settings_modal_inset;
    let fit = |available: Pixels, max: Pixels| (available - inset).max(Pixels::ZERO).min(max);
    (fit(viewport.width, theme.settings_modal_max_width), fit(viewport.height, theme.settings_modal_max_height))
}

/// The rail beside the content: a persisted resizable split, or a fixed icon rail.
fn split(theme: &ThemeTokens, compact: bool, rail: impl IntoElement, content: impl IntoElement) -> AnyElement {
    if compact {
        let rail = div().flex_none().h_full().w(theme.settings_rail_compact_width).child(rail);
        return div().size_full().flex().child(rail).child(div().flex_1().min_w_0().h_full().child(content)).into_any_element();
    }
    persisted_h_resizable(SPLIT_ID, theme)
        .pane(SplitPane::sized(theme.modal_rail_width, theme.modal_rail_min..theme.modal_rail_max).child(rail))
        .pane(SplitPane::flex(theme.split_flex_min).child(content))
        .into_any_element()
}

/// The scrim and dialog around `rail` and `content`.
pub(in crate::ui::features::settings) fn render_frame(
    theme: &ThemeTokens,
    window: &Window,
    (on_close, compact): (PlainAction, bool),
    rail: impl IntoElement,
    content: impl IntoElement,
) -> impl IntoElement {
    let (width, height) = dialog_size(window.viewport_size(), theme);
    let (scrim_close, key_close) = (on_close.clone(), on_close);
    div()
        .id(BACKDROP_ID)
        .absolute()
        .inset_0()
        .occlude()
        .bg(theme.bg_scrim)
        .flex()
        .items_center()
        .justify_center()
        .on_mouse_down(MouseButton::Left, move |_, window, cx| scrim_close(window, cx))
        .child(
            div()
                .id(DIALOG_ID)
                .w(width)
                .h(height)
                .max_w_full()
                .max_h_full()
                .rounded(theme.radius_lg)
                .border(theme.border_width_hairline)
                .border_color(theme.border)
                .bg(theme.bg_elevated)
                .elevation_shadow(theme)
                .flex()
                .overflow_hidden()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_key_down(move |event: &KeyDownEvent, window, cx| {
                    if event.keystroke.key == KEY_ESCAPE {
                        cx.stop_propagation();
                        key_close(window, cx);
                    }
                })
                .child(split(theme, compact, rail, content)),
        )
}
