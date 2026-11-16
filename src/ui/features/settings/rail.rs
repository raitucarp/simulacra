//! Left navigation rail of the settings modal: brand header, tab list and status footer.

use super::callbacks::SelectTabCallback;
use super::consts::{
    BRAND_BADGE_SIZE, ICON_LG, ICON_SM, ICON_XL, TAB_BUTTON_ID_PREFIX, TAB_INDICATOR_HEIGHT,
    TAB_INDICATOR_WIDTH, TEXT_CAPTION, TEXT_LABEL, TEXT_TITLE,
};
use super::tabs::SettingsTab;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, px, Div, FontWeight, InteractiveElement, IntoElement, MouseButton, ParentElement,
    Stateful, Styled,
};
use std::rc::Rc;

/// Renders the full left rail (title, tab buttons, sovereignty footer).
pub(super) fn render_rail(
    theme: &ThemeTokens,
    active_tab: SettingsTab,
    on_tab: Rc<SelectTabCallback>,
) -> impl IntoElement {
    let tab_buttons: Vec<_> = SettingsTab::ALL
        .iter()
        .map(|&tab| render_tab_button(theme, tab, tab == active_tab, on_tab.clone()))
        .collect();

    div()
        .w(theme.modal_rail_width)
        .h_full()
        .bg(theme.bg_sidebar)
        .border_r_1()
        .border_color(theme.border)
        .p_4()
        .flex()
        .flex_col()
        .justify_between()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(render_rail_brand(theme))
                .child(div().flex().flex_col().gap_1().children(tab_buttons)),
        )
        .child(render_rail_footer(theme))
}

/// Rail top: settings icon badge with title and subtitle.
fn render_rail_brand(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap_2p5()
        .child(
            div()
                .w(px(BRAND_BADGE_SIZE))
                .h(px(BRAND_BADGE_SIZE))
                .rounded_lg()
                .bg(theme.accent_brand_subtle)
                .flex()
                .items_center()
                .justify_center()
                .child(Icon::new(LucideIcon::Sliders).size(px(ICON_LG)).text_color(theme.accent_brand)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .child(div().text_size(px(TEXT_TITLE)).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child("Settings"))
                .child(div().text_size(px(TEXT_CAPTION)).text_color(theme.text_muted).child("Simulacra System")),
        )
}

/// Rail bottom: epistemic status footer.
fn render_rail_footer(theme: &ThemeTokens) -> Div {
    div()
        .p_2p5()
        .rounded_lg()
        .bg(theme.hover_bg)
        .border_1()
        .border_color(theme.border)
        .flex()
        .items_center()
        .gap_2()
        .child(Icon::new(LucideIcon::Lock).size(px(ICON_SM)).text_color(theme.text_muted))
        .child(
            div()
                .text_size(px(TEXT_CAPTION))
                .text_color(theme.text_muted)
                .child("Local Epistemic Sovereignty"),
        )
}

/// One selectable tab row with icon, label and active indicator bar.
fn render_tab_button(
    theme: &ThemeTokens,
    tab: SettingsTab,
    is_active: bool,
    cb: Rc<SelectTabCallback>,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    let bg_color = if is_active { theme.bg_tab_active } else { theme.bg_transparent };
    div()
        .id(format!("{}{:?}", TAB_BUTTON_ID_PREFIX, tab))
        .w_full()
        .h(theme.titlebar_height)
        .px_3()
        .rounded_lg()
        .bg(bg_color)
        .hover(move |s| s.bg(hover_bg))
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_between()
        .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
            cx.stop_propagation();
            (cb)(tab, window, cx);
        })
        .child(render_tab_label(theme, tab, is_active))
        .when(is_active, |this| {
            this.child(
                div()
                    .w(px(TAB_INDICATOR_WIDTH))
                    .h(px(TAB_INDICATOR_HEIGHT))
                    .rounded_full()
                    .bg(theme.accent_brand),
            )
        })
}

/// Icon + label pair inside a tab button.
fn render_tab_label(theme: &ThemeTokens, tab: SettingsTab, is_active: bool) -> Div {
    let text_color = if is_active { theme.text_primary } else { theme.text_secondary };
    let icon_color = if is_active { theme.accent_brand } else { theme.text_muted };
    let weight = if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL };
    div()
        .flex()
        .items_center()
        .gap_2p5()
        .child(Icon::new(tab.icon()).size(px(ICON_XL)).text_color(icon_color))
        .child(div().text_size(px(TEXT_LABEL)).font_weight(weight).text_color(text_color).child(tab.label()))
}
