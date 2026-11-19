//! Stream pane header: clickable Threads-style "For you" / "Dialectic" tabs, and on "For you"
//! the switch between the newest-first and the recommended order.

use super::consts::{FEED_ORDER_ID, STREAM_TAB_ID};
use super::params::{FeedDiscovery, FeedOrder, RunCommandCallback, SelectFeedTabCallback};
use crate::ui::features::command_palette::{spec, CommandId};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use crate::state::FeedTab;
use crate::theme::ThemeTokens;
use gpui_kit::{
    div, Div, FontWeight, InteractiveElement, ParentElement, Stateful,
    StatefulInteractiveElement, Styled,
};
use rust_i18n::t;
use std::rc::Rc;

/// Header row of the stream pane: the stream tabs.
pub(crate) fn render_stream_header(
    theme: &ThemeTokens,
    active: FeedTab,
    on_select: Rc<SelectFeedTabCallback>,
    on_command: RunCommandCallback,
    discovery: &FeedDiscovery,
) -> Div {
    let order = (active == FeedTab::ForYou).then(|| render_order_switch(theme, discovery));
    let tabs = FeedTab::ALL
        .into_iter()
        .enumerate()
        .map(|(position, tab)| render_tab(theme, tab, position, tab == active, on_select.clone()));
    div()
        .w_full()
        .px(theme.panel_padding)
        .h(theme.feed_header_height)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .items_center()
        .justify_between()
        .child(div().flex().items_center().gap(theme.spacing_xl).h_full().children(tabs))
        .child(div().flex().items_center().gap(theme.inline_gap).children(order).child(render_header_commands(theme, on_command)))
}

/// "Latest" / "Recommended": shows the current "For you" order; clicking switches it.
fn render_order_switch(theme: &ThemeTokens, discovery: &FeedDiscovery) -> Button {
    let (icon, label, tooltip) = match discovery.order {
        FeedOrder::Latest => (LucideIcon::Clock, t!("feed.order.latest"), t!("feed.order.latest_hint")),
        FeedOrder::Recommended => (LucideIcon::Sparkles, t!("feed.order.recommended"), t!("feed.order.recommended_hint")),
    };
    let (next, on_select) = (discovery.order.toggled(), discovery.on_select_order.clone());
    Button::new(FEED_ORDER_ID)
        .ghost()
        .small()
        .icon(Icon::new(icon).size(theme.icon_size_md))
        .label(label.to_string())
        .tooltip(tooltip.to_string())
        .on_click(move |_, window, cx| on_select(next, window, cx))
}

/// The quick-capture and import buttons at the right of the header.
fn render_header_commands(theme: &ThemeTokens, on_command: RunCommandCallback) -> Div {
    let buttons = [(CommandId::NewNote, LucideIcon::Feather), (CommandId::ImportFile, LucideIcon::FileUp)].map(|(command, icon)| {
        let run = on_command.clone();
        Button::new(command.wire_key())
            .ghost()
            .icon(Icon::new(icon).size(theme.icon_size_md))
            .tooltip(spec(command).title())
            .on_click(move |_, window, cx| run(command, window, cx))
    });
    div().flex().items_center().gap(theme.inline_gap).children(buttons)
}

fn tab_label_text(tab: FeedTab) -> String {
    match tab {
        FeedTab::ForYou => t!("feed.for_you"),
        FeedTab::Dialectic => t!("feed.dialectic"),
        FeedTab::Inbox => t!("feed.inbox"),
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
                .w(theme.feed_tab_indicator_width)
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
