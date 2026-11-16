//! Dialog frame of the settings modal: scrim backdrop, tab header, scrollable body and action footer.

use super::callbacks::{ModalCloseCallback, SaveSettingsCallback};
use super::consts::{
    BACKDROP_ID, CANCEL_BUTTON_ID, CLOSE_BUTTON_ID, CONTENT_HEADER_HEIGHT, DIALOG_ID, ICON_LG,
    ICON_MD, SAVE_BUTTON_ID, TEXT_HEADING, TEXT_SMALL,
};
use super::tabs::SettingsTab;
use crate::ui::atoms::ElevationExt;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::{
    div, px, AnyElement, Div, FontWeight, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Styled,
};
use std::rc::Rc;

/// Assembles the right content pane (flex_1) from header, tab body and footer.
pub(super) fn render_content_pane(
    header: impl IntoElement,
    tab_content: AnyElement,
    footer: impl IntoElement,
) -> Div {
    div()
        .flex_1()
        .h_full()
        .flex()
        .flex_col()
        .child(header)
        // Scrollable Tab Body
        .child(div().flex_1().overflow_y_scrollbar().p_6().child(tab_content))
        .child(footer)
}

/// Header of the active tab: title, description and close button.
pub(super) fn render_content_header(
    theme: &ThemeTokens,
    active_tab: SettingsTab,
    on_close: Rc<ModalCloseCallback>,
) -> Div {
    div()
        .h(px(CONTENT_HEADER_HEIGHT))
        .px_6()
        .border_b_1()
        .border_color(theme.border)
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_0p5()
                .child(
                    div()
                        .text_size(px(TEXT_HEADING))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(active_tab.title()),
                )
                .child(div().text_size(px(TEXT_SMALL)).text_color(theme.text_muted).child(active_tab.description())),
        )
        .child(
            Button::new(CLOSE_BUTTON_ID)
                .ghost()
                .icon(Icon::new(LucideIcon::X).size(px(ICON_LG)))
                .on_click(move |_, window, cx| {
                    (on_close)(window, cx);
                }),
        )
}

/// Persistent footer action bar with Cancel and Save buttons.
pub(super) fn render_footer_bar(
    theme: &ThemeTokens,
    on_close: Rc<ModalCloseCallback>,
    on_save: Rc<SaveSettingsCallback>,
) -> Div {
    div()
        .h(theme.modal_footer_height)
        .px_6()
        .border_t_1()
        .border_color(theme.border)
        .bg(theme.bg_elevated)
        .flex()
        .items_center()
        .justify_between()
        .child(
            Button::new(CANCEL_BUTTON_ID).ghost().label("Cancel").on_click(move |_, window, cx| {
                (on_close)(window, cx);
            }),
        )
        .child(
            Button::new(SAVE_BUTTON_ID)
                .primary()
                .icon(Icon::new(LucideIcon::Check).size(px(ICON_MD)))
                .label("Save & Apply Settings")
                .on_click(move |_, window, cx| {
                    (on_save)(window, cx);
                }),
        )
}

/// Deep opaque scrim backdrop wrapping the tokenized dialog container.
pub(super) fn render_backdrop(
    theme: &ThemeTokens,
    on_close: Rc<ModalCloseCallback>,
    rail: impl IntoElement,
    content: impl IntoElement,
) -> impl IntoElement {
    div()
        .id(BACKDROP_ID)
        .absolute()
        .inset_0()
        .occlude()
        .bg(theme.bg_scrim) // Deep opaque scrim to eliminate background bleed
        .flex()
        .items_center()
        .justify_center()
        .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
            (on_close)(window, cx);
        })
        .child(
            div()
                .id(DIALOG_ID)
                .w(theme.modal_width)
                .h(theme.modal_height)
                .rounded_xl()
                .border_1()
                .border_color(theme.border)
                .bg(theme.bg_elevated) // solid OLED dark surface
                .elevation_shadow(theme)
                .flex()
                .overflow_hidden()
                .on_mouse_down(MouseButton::Left, |_ev, _window, cx| {
                    cx.stop_propagation();
                })
                .child(rail)
                .child(content),
        )
}
