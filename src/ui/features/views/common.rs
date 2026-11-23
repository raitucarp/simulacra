//! Pieces every list view shares: the locked-post note, the empty note and the scrolling
//! column.

use super::input::ViewInput;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// What a locked post shows in every view: its placeholder, never its text (X4).
pub fn locked_note(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .text_size(theme.font_size_xs)
        .text_color(theme.text_muted)
        .child(Icon::new(LucideIcon::LockKeyhole).xsmall().text_color(theme.text_muted))
        .child(t!("posts.lock.locked").to_string())
}

/// A muted line saying why a view is empty.
pub fn empty_note(key: &str, theme: &ThemeTokens) -> AnyElement {
    div().p(theme.panel_padding).text_size(theme.font_size_sm).text_color(theme.text_muted).child(t!(key).to_string()).into_any_element()
}

/// A muted section heading (a day, a month, a state).
pub fn section_heading(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(text)
}

/// The vertically scrolling column a view's sections sit in.
pub fn scroll_column(input: &ViewInput<'_>, part: &str) -> Stateful<Div> {
    let theme = input.theme;
    div().id(input.id(part)).flex_1().min_h_0().w_full().overflow_y_scroll().px(theme.panel_padding_compact).py(theme.spacing_xs).flex().flex_col().gap(theme.section_gap)
}
