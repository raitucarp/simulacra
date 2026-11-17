//! Dialog frame of the settings modal: scrim backdrop, tab header, scrollable body and action footer.

use super::callbacks::{ModalCloseCallback, SaveSettingsCallback};
use super::consts::{BACKDROP_ID, CANCEL_BUTTON_ID, CLOSE_BUTTON_ID, DIALOG_ID, SAVE_BUTTON_ID, SPLIT_ID};
use super::tabs::SettingsTab;
use crate::ui::atoms::ElevationExt;
use crate::theme::ThemeTokens;
use crate::ui::components::{persisted_h_resizable, SplitPane};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::{
    div, AnyElement, Div, FontWeight, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Styled,
};
use std::rc::Rc;
use rust_i18n::t;

/// Assembles the right content pane (flex_1) from header, tab body and footer.
pub(super) fn render_content_pane(
    header: impl IntoElement,
    tab_content: AnyElement,
    footer: impl IntoElement,
    theme: &ThemeTokens,
) -> Div {
    div()
        .flex_1()
        .h_full()
        .flex()
        .flex_col()
        .child(header)
        // Scrollable Tab Body
        .child(div().flex_1().overflow_y_scrollbar().p(theme.panel_padding).child(tab_content))
        .child(footer)
}

/// Header of the active tab: title, description and close button.
pub(super) fn render_content_header(
    theme: &ThemeTokens,
    active_tab: SettingsTab,
    on_close: Rc<ModalCloseCallback>,
) -> Div {
    div()
        .h(theme.settings_header_height)
        .px(theme.panel_padding)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border)
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .flex_col()
                .gap(theme.spacing_2xs)
                .child(
                    div()
                        .text_size(theme.font_size_md)
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(active_tab.title()),
                )
                .child(div().text_size(theme.font_size_hint).text_color(theme.text_muted).child(active_tab.description())),
        )
        .child(
            Button::new(CLOSE_BUTTON_ID)
                .ghost()
                .icon(Icon::new(LucideIcon::X).size(theme.icon_size_lg))
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
        .px(theme.panel_padding)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border)
        .bg(theme.bg_elevated)
        .flex()
        .items_center()
        .justify_between()
        .child(
            Button::new(CANCEL_BUTTON_ID).ghost().label(t!("settings.cancel").to_string()).on_click(move |_, window, cx| {
                (on_close)(window, cx);
            }),
        )
        .child(
            Button::new(SAVE_BUTTON_ID)
                .primary()
                .icon(Icon::new(LucideIcon::Check).size(theme.icon_size_md))
                .label(t!("settings.save_apply").to_string())
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
                .rounded(theme.radius_lg)
                .border(theme.border_width_hairline)
                .border_color(theme.border)
                .bg(theme.bg_elevated) // solid OLED dark surface
                .elevation_shadow(theme)
                .flex()
                .overflow_hidden()
                .on_mouse_down(MouseButton::Left, |_ev, _window, cx| {
                    cx.stop_propagation();
                })
                .child(
                    persisted_h_resizable(SPLIT_ID, theme)
                        .pane(SplitPane::sized(theme.modal_rail_width, theme.modal_rail_min..theme.modal_rail_max).child(rail))
                        .pane(SplitPane::flex(theme.split_flex_min).child(content)),
                ),
        )
}
