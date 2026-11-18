//! Building blocks of the Arena page and the profile's Beliefs tab: cards, titles, captions,
//! framed text fields, buttons, side badges and value bars.

use crate::domain::arena::debate::Side;
use crate::theme::ThemeTokens;
use crate::ui::components::{meta_badge, BadgeColors};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

/// A surface card holding one block of the page.
pub fn surface(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .p(theme.card_padding)
        .rounded(theme.radius_lg)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
}

/// An icon and a title.
pub fn block_title(icon: LucideIcon, color: Rgba, title: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .child(Icon::new(icon).size(theme.icon_size_sm).text_color(color))
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title))
}

/// A small upper-case section label.
pub fn section_label(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(text.to_uppercase())
}

/// Secondary explanatory text.
pub fn caption(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_xs).line_height(relative(theme.line_height_body)).text_color(theme.text_muted).child(text)
}

/// Body prose.
pub fn prose(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_sm).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(text)
}

/// A row of controls that wraps on narrow windows.
pub fn control_row(theme: &ThemeTokens) -> Div {
    div().flex().flex_wrap().items_center().gap(theme.control_gap)
}

fn field_frame(theme: &ThemeTokens) -> Div {
    div().w_full().rounded(theme.radius_md).border(theme.border_width_hairline).border_color(theme.border).bg(theme.input_bg).px(theme.spacing_sm)
}

/// A one-line text field in the page's frame.
pub fn line_field(input: &Entity<InputState>, theme: &ThemeTokens) -> Div {
    field_frame(theme).child(Input::new(input).bordered(false))
}

/// A multi-line text field in the page's frame.
pub fn area_field(input: &Entity<TextareaState>, theme: &ThemeTokens) -> Div {
    field_frame(theme).py(theme.spacing_xs).child(Textarea::new(input).bordered(false))
}

/// A label column beside a control.
pub fn labelled(label: String, control: impl IntoElement, theme: &ThemeTokens) -> Div {
    let label = div().w(theme.arena_row_label_width).flex_none().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(label);
    div().w_full().flex().flex_wrap().items_center().gap(theme.control_gap).child(label).child(div().flex_1().min_w_0().child(control))
}

/// A small button with an icon and a label.
pub fn small_button(id: impl Into<ElementId>, icon: LucideIcon, label: String) -> Button {
    Button::new(id).small().icon(Icon::new(icon)).label(label)
}

pub fn primary_button(id: impl Into<ElementId>, icon: LucideIcon, label: String) -> Button {
    small_button(id, icon, label).primary()
}

pub fn ghost_button(id: impl Into<ElementId>, icon: LucideIcon, label: String) -> Button {
    small_button(id, icon, label).ghost()
}

/// An icon-only ghost button with a tooltip.
pub fn icon_button(id: impl Into<ElementId>, icon: LucideIcon, tooltip: String) -> Button {
    Button::new(id).small().ghost().icon(Icon::new(icon)).tooltip(tooltip)
}

/// The accent of a side: brand for, dialectic gold against, muted when open.
pub fn side_color(side: Side, theme: &ThemeTokens) -> BadgeColors {
    match side {
        Side::For => BadgeColors::new(theme.accent_brand, theme.accent_brand_subtle, theme.accent_brand_border),
        Side::Against => BadgeColors::new(theme.accent_dialectic, theme.accent_dialectic_subtle, theme.accent_dialectic_border),
        Side::Open => BadgeColors::new(theme.text_secondary, theme.bg_elevated, theme.border_subtle),
    }
}

pub fn side_badge(side: Side, label: String, theme: &ThemeTokens) -> Div {
    meta_badge(&label, side_color(side, theme), theme)
}

/// A thin track `width` wide, filled to `share` (0–1) in `color`.
pub fn value_bar(share: f32, width: Pixels, color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .w(width)
        .flex_none()
        .h(theme.arena_bar_height)
        .rounded(theme.radius_full)
        .bg(theme.border_subtle)
        .overflow_hidden()
        .child(div().h_full().w(relative(share.clamp(0.0, 1.0))).rounded(theme.radius_full).bg(color))
}
