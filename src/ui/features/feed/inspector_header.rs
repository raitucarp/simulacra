//! Focus thread inspector chrome: header and quick reply bar.

use super::consts::QUICK_REPLY_BUTTON_ID;
use super::inspector_mode_toggle::render_mode_toggle;
use super::inspector_outline::InspectorCallback;
use crate::domain::views::PostView;
use super::params::ReplyPostCallback;
use crate::domain::{Post, Profile};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Div, FontWeight, ParentElement, Styled};
use crate::i18n::t_count;
use rust_i18n::t;
use std::rc::Rc;

/// Inspector header with title, (when a thread is focused) the replies count pill, and the
/// cards / outline switch.
pub(crate) fn render_inspector_header(theme: &ThemeTokens, replies_count: Option<usize>, mode: PostView, on_command: &InspectorCallback) -> Div {
    div()
        .w_full()
        .px(theme.panel_padding)
        .h(theme.feed_header_height)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.control_gap)
        .child(
            div()
                .flex()
                .min_w_0()
                .items_center()
                .gap(theme.spacing_sm)
                .child(Icon::new(LucideIcon::MessageSquare).size(theme.font_size_md).text_color(theme.accent_brand))
                .child(
                    div()
                        .min_w_0()
                        .truncate()
                        .font_family(theme.font_family_display.clone())
                        .text_size(theme.font_size_base)
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(t!("feed.inspector_title").to_string()),
                ),
        )
        .child(
            div()
                .flex_none()
                .flex()
                .items_center()
                .gap(theme.control_gap)
                .when_some(replies_count, |this, count| this.child(render_replies_pill(count, theme)))
                .child(render_mode_toggle(mode, on_command, theme)),
        )
}

fn render_replies_pill(count: usize, theme: &ThemeTokens) -> Div {
    div()
        .flex_none()
        .whitespace_nowrap()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_sm)
        .bg(theme.hover_bg)
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_xs)
        .font_weight(FontWeight::MEDIUM)
        .text_color(theme.text_secondary)
        .child(t_count("feed.replies_count", count))
}

/// Pinned bottom quick reply bar for the focused thought.
pub(crate) fn render_quick_reply_bar(
    root: &Post,
    author: &Profile,
    theme: &ThemeTokens,
    on_reply: Rc<ReplyPostCallback>,
) -> Div {
    let root_id = root.id.clone();
    let author_handle = author.handle.clone();
    div()
        .w_full()
        .px(theme.panel_padding)
        .py(theme.spacing_sm)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.bg_elevated)
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_sm)
        .child(
            div()
                .font_family(theme.font_family_ui.clone())
                .flex()
                .items_center()
                .gap(theme.spacing_xs)
                .text_size(theme.font_size_xs)
                .text_color(theme.text_muted)
                .child(t!("feed.reply_to", handle = author_handle).to_string()),
        )
        .child(
            Button::new(QUICK_REPLY_BUTTON_ID)
                .ghost()
                .compact()
                .icon(Icon::new(LucideIcon::MessageSquare))
                .label(t!("feed.quote_or_reply").to_string())
                .on_click(move |_, window, cx| on_reply(root_id.clone(), &author_handle, window, cx)),
        )
}
