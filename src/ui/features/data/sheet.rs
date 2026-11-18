//! The frame every data sheet shares: a scrim, an elevated card with a header (icon, title,
//! subtitle), a scrolling body and a footer of buttons. Clicking the scrim closes it.

use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use std::rc::Rc;

/// A no-argument sheet callback.
pub(super) type SheetCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// What one sheet puts in the frame.
pub(super) struct SheetParts {
    /// Element id stem (`<id>-backdrop`, `<id>-card`, `<id>-scroll`).
    pub id: &'static str,
    pub icon: LucideIcon,
    pub title: String,
    pub subtitle: Option<String>,
    pub body: Div,
    pub footer: Div,
    pub on_close: SheetCallback,
}

/// A small upper-case heading inside a sheet.
pub(super) fn heading(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(text)
}

/// A muted explanatory line.
pub(super) fn hint(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_xs).line_height(relative(theme.line_height_body)).text_color(theme.text_muted).child(text)
}

/// A tinted notice box (warnings and errors).
pub(super) fn notice(icon: LucideIcon, text: String, colour: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .bg(theme.bg_callout)
        .border_l(theme.border_width_threadline)
        .border_color(colour)
        .child(Icon::new(icon).size(theme.icon_size_sm).text_color(colour))
        .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(text))
}

/// A clickable row with a checkbox and its label.
pub(super) fn tick_row(id: &'static str, label: String, checked: bool, on_click: SheetCallback, theme: &ThemeTokens) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .cursor_pointer()
        .on_click(move |_, window, cx| on_click(window, cx))
        .child(gpui_kit::component::checkbox::Checkbox::new(SharedString::from(format!("{id}-box"))).checked(checked))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(label))
}

/// The footer row (buttons right-aligned).
pub(super) fn footer(theme: &ThemeTokens) -> Div {
    div().flex().flex_wrap().items_center().justify_end().gap(theme.control_gap).px(theme.card_padding).py(theme.spacing_sm).border_t(theme.border_width_hairline).border_color(theme.border_subtle)
}

fn header(parts: &SheetParts, theme: &ThemeTokens) -> Div {
    let titles = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(parts.title.clone()))
        .children(parts.subtitle.clone().map(|s| div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(s)));
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.card_padding)
        .py(theme.spacing_sm)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(Icon::new(parts.icon).size(theme.icon_size_sm).text_color(theme.accent_brand))
        .child(titles)
}

/// The sheet over its scrim.
pub(super) fn sheet_frame(parts: SheetParts, theme: &ThemeTokens) -> Stateful<Div> {
    let id = parts.id;
    let dismiss = parts.on_close.clone();
    let head = header(&parts, theme);
    let body = div().id(SharedString::from(format!("{id}-scroll"))).flex_1().min_h_0().overflow_y_scroll().child(parts.body.flex().flex_col().gap(theme.section_gap).p(theme.card_padding));
    let card = div()
        .id(SharedString::from(format!("{id}-card")))
        .w(theme.data_sheet_width)
        .max_w_full()
        .max_h(theme.data_sheet_max_height)
        .flex()
        .flex_col()
        .overflow_hidden()
        .rounded(theme.radius_lg)
        .bg(theme.bg_elevated)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .elevation_shadow(theme)
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .child(head)
        .child(body)
        .child(parts.footer);
    div()
        .id(SharedString::from(format!("{id}-backdrop")))
        .absolute()
        .inset_0()
        .occlude()
        .bg(theme.overlay)
        .flex()
        .flex_col()
        .items_center()
        .px(theme.panel_padding_compact)
        .pt(theme.palette_top_offset)
        .on_mouse_down(MouseButton::Left, move |_, window, cx| dismiss(window, cx))
        .child(card)
}
