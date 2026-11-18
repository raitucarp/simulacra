//! Building blocks of the Journal page: cards, titles, captions, tiles, a progress track,
//! framed text fields and button helpers.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

/// A surface card holding one block of the page.
pub fn surface(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
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

/// Secondary explanatory text.
pub fn caption(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_xs).line_height(relative(theme.line_height_body)).text_color(theme.text_muted).child(text)
}

/// Body prose.
pub fn prose(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_sm).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(text)
}

/// A big number over its label.
pub fn count_tile(value: String, label: String, color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .flex_1()
        .min_w(theme.journal_row_label_width)
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .p(theme.card_padding)
        .rounded(theme.radius_md)
        .bg(theme.bg_elevated)
        .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(color).child(value))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(label))
}

/// A thin track filled to `share` (0–1) in `color`.
pub fn progress_bar(share: f32, color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .h(theme.journal_progress_height)
        .rounded(theme.radius_full)
        .bg(theme.border_subtle)
        .overflow_hidden()
        .child(div().h_full().w(relative(share.clamp(0.0, 1.0))).rounded(theme.radius_full).bg(color))
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

/// A small button with an icon and a label.
pub fn small_button(id: impl Into<ElementId>, icon: LucideIcon, label: String) -> Button {
    Button::new(id).small().icon(Icon::new(icon)).label(label)
}

/// A small primary button.
pub fn primary_button(id: impl Into<ElementId>, icon: LucideIcon, label: String) -> Button {
    small_button(id, icon, label).primary()
}

/// A small ghost button.
pub fn ghost_button(id: impl Into<ElementId>, icon: LucideIcon, label: String) -> Button {
    small_button(id, icon, label).ghost()
}

/// A label column beside a control (the check-in and focus forms).
pub fn labelled(label: String, control: impl IntoElement, theme: &ThemeTokens) -> Div {
    let label = div().w(theme.journal_row_label_width).flex_none().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(label);
    div().w_full().flex().flex_wrap().items_center().gap(theme.control_gap).child(label).child(div().flex_1().min_w_0().child(control))
}

/// `element` fading in over the calm motion duration (at once with reduced motion).
pub fn entrance(element: Div, id: impl Into<ElementId>, reduced_motion: bool, theme: &ThemeTokens) -> AnyElement {
    if reduced_motion {
        return element.into_any_element();
    }
    let duration = std::time::Duration::from_millis(theme.vault_motion_duration_ms as u64);
    element.with_animation(id, Animation::new(duration).with_easing(ease_in_out), |el, delta| el.opacity(delta)).into_any_element()
}
