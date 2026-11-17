//! Panel layout row: how to resize panels, and "Reset layout" (every split back to its
//! default sizes, saved sizes forgotten).

use super::super::common::{field_title, muted_note, spaced_row};
use crate::theme::ThemeTokens;
use crate::ui::components::persisted_split::reset_panel_layouts;
use gpui_kit::component::button::Button;
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;

const RESET_LAYOUT_ID: &str = "appearance-reset-layout";

/// Title, hint and the reset button.
pub(super) fn render(theme: &ThemeTokens) -> Div {
    let reset = Button::new(RESET_LAYOUT_ID)
        .small()
        .outline()
        .label(t!("layout.reset").to_string())
        .on_click(|_, window, cx| {
            reset_panel_layouts(cx);
            window.refresh();
        });
    let text = div()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .min_w_0()
        .child(field_title(theme, t!("layout.title").to_string()))
        .child(muted_note(theme, t!("layout.hint").to_string()));
    spaced_row().gap(theme.spacing_md).child(text).child(reset)
}
