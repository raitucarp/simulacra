//! Stream pane header: clickable Threads-style "For you" / "Dialectic" tabs and the loop
//! status pill.

use super::consts::{ACTIVE_TAB_INDICATOR_WIDTH, STREAM_TAB_ID};
use super::params::SelectFeedTabCallback;
use crate::state::FeedTab;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{
    div, px, Div, FontWeight, InteractiveElement, ParentElement, Stateful,
    StatefulInteractiveElement, Styled,
};
use rust_i18n::t;
use std::rc::Rc;

/// Header row of the stream pane: the stream tabs and the loop status.
pub(crate) fn render_stream_header(
    theme: &ThemeTokens,
    active: FeedTab,
    on_select: Rc<SelectFeedTabCallback>,
) -> Div {
    let tabs = FeedTab::ALL
        .into_iter()
        .enumerate()
        .map(|(position, tab)| render_tab(theme, tab, position, tab == active, on_select.clone()));
    div()
        .w_full()
        .px(theme.spacing_lg)
        .h(theme.feed_header_height)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .items_center()
        .justify_between()
        .child(div().flex().items_center().gap(theme.spacing_xl).h_full().children(tabs))
        .child(render_loop_status(theme))
}

fn tab_label_text(tab: FeedTab) -> String {
    match tab {
        FeedTab::ForYou => t!("feed.for_you"),
        FeedTab::Dialectic => t!("feed.dialectic"),
    }
    .to_string()
}

/// A clickable tab; the active one is bold with the brand underline indicator.
fn render_tab(
    theme: &ThemeTokens,
    tab: FeedTab,
    position: usize,
    is_active: bool,
    on_select: Rc<SelectFeedTabCallback>,
) -> Stateful<Div> {
    let indicator = if is_active { theme.accent_brand } else { theme.bg_transparent };
    div()
        .id((STREAM_TAB_ID, position))
        .flex()
        .flex_col()
        .items_center()
        .h_full()
        .justify_between()
        .pt(theme.spacing_sm)
        .cursor_pointer()
        .on_click(move |_, window, cx| on_select(tab, window, cx))
        .child(render_tab_label(theme, tab, is_active))
        .child(
            div()
                .w(px(ACTIVE_TAB_INDICATOR_WIDTH))
                .h(theme.border_width_threadline)
                .rounded(theme.radius_full)
                .bg(indicator),
        )
}

/// Tab label in the display typeface: bold and primary when active, muted otherwise.
fn render_tab_label(theme: &ThemeTokens, tab: FeedTab, is_active: bool) -> Div {
    let (weight, color) = if is_active {
        (FontWeight::BOLD, theme.text_primary)
    } else {
        (FontWeight::MEDIUM, theme.text_muted)
    };
    let hover_color = theme.text_primary;
    div()
        .font_family(theme.font_family_display.clone())
        .text_size(theme.font_size_base)
        .font_weight(weight)
        .text_color(color)
        .hover(move |s| s.text_color(hover_color))
        .child(tab_label_text(tab))
}

/// "Loop active" status pill.
fn render_loop_status(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_full)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(
            div()
                .w(theme.font_size_xs)
                .h(theme.font_size_xs)
                .text_color(theme.accent_success)
                .child(Icon::new(LucideIcon::Activity)),
        )
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.accent_success)
                .child(t!("feed.loop_active").to_string()),
        )
}
