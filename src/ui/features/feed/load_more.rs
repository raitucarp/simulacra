//! Tokenized "Show more" trigger for progressive stream windowing.

use super::consts::{LOAD_MORE_BUTTON_ID, LOAD_MORE_CONTAINER_ID};
use super::params::LoadMoreCallback;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, AnyElement, Div, FontWeight, InteractiveElement, IntoElement, ParentElement, Stateful,
    StatefulInteractiveElement, Styled,
};
use rust_i18n::t;
use std::rc::Rc;

/// Load-more container: button plus "Showing n of total thoughts" summary.
pub(crate) fn render_load_more(
    shown: usize,
    total: usize,
    theme: &ThemeTokens,
    on_load_more: Option<Rc<LoadMoreCallback>>,
) -> AnyElement {
    div()
        .id(LOAD_MORE_CONTAINER_ID)
        .w_full()
        .py(theme.spacing_lg)
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(theme.spacing_xs)
        .child(render_load_more_button(total - shown, theme, on_load_more))
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_2xs)
                .text_color(theme.text_muted)
                .child(t!("feed.window_summary", shown = shown, total = total).to_string()),
        )
        .into_any_element()
}

fn render_load_more_button(
    remaining: usize,
    theme: &ThemeTokens,
    on_load_more: Option<Rc<LoadMoreCallback>>,
) -> Stateful<Div> {
    let (hover_bg, hover_border) = (theme.hover_bg, theme.border_strong);
    div()
        .id(LOAD_MORE_BUTTON_ID)
        .px(theme.spacing_lg)
        .py(theme.spacing_sm)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .hover(move |s| s.bg(hover_bg).border_color(hover_border))
        .cursor_pointer()
        .when_some(on_load_more, |this, cb| this.on_click(move |_, window, cx| cb(window, cx)))
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(
            div()
                .font_family(theme.font_family_ui.clone())
                .text_size(theme.font_size_sm)
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.text_secondary)
                .child(t!("feed.show_more", count = remaining).to_string()),
        )
        .child(
            div()
                .w(theme.font_size_sm)
                .h(theme.font_size_sm)
                .text_color(theme.text_muted)
                .child(Icon::new(LucideIcon::ChevronDown)),
        )
}
