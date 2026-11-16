//! Focus thread inspector chrome: header, grounding callout, and quick reply bar.

use super::consts::{QUICK_REPLY_BUTTON_ID, REPLY_TO_LABEL};
use super::params::ReplyPostCallback;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Div, FontWeight, ParentElement, Styled};
use rust_i18n::t;
use std::rc::Rc;

/// Inspector header with title and (when a thread is focused) the replies count pill.
pub(crate) fn render_inspector_header(theme: &ThemeTokens, replies_count: Option<usize>) -> Div {
    div()
        .w_full()
        .px(theme.spacing_lg)
        .h(theme.feed_header_height)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.spacing_sm)
                .child(Icon::new(LucideIcon::MessageSquare).size(theme.font_size_md).text_color(theme.accent_brand))
                .child(
                    div()
                        .font_family(theme.font_family_display.clone())
                        .text_size(theme.font_size_base)
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(t!("feed.inspector_title").to_string()),
                ),
        )
        .when_some(replies_count, |this, count| this.child(render_replies_pill(count, theme)))
}

fn render_replies_pill(count: usize, theme: &ThemeTokens) -> Div {
    div()
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
        .child(t!("feed.replies_count", count = count).to_string())
}

/// Grounding context insight shown when the focused thought has no replies yet.
pub(crate) fn render_grounding_callout(theme: &ThemeTokens) -> Div {
    div()
        .m(theme.spacing_lg)
        .p(theme.spacing_md)
        .rounded(theme.radius_md)
        .bg(theme.bg_callout)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.spacing_xs)
                .child(Icon::new(LucideIcon::Sparkles).size(theme.font_size_sm).text_color(theme.accent_brand))
                .child(
                    div()
                        .font_family(theme.font_family_display.clone())
                        .text_size(theme.font_size_xs)
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(t!("feed.grounding_context_title").to_string()),
                ),
        )
        .child(
            div()
                .font_family(theme.font_family_body.clone())
                .text_size(theme.font_size_xs)
                .text_color(theme.text_secondary)
                .child(t!("feed.grounding_context_desc").to_string()),
        )
}

/// Pinned bottom quick reply bar for the focused thought.
pub(crate) fn render_quick_reply_bar(root: &Post, theme: &ThemeTokens, on_reply: Rc<ReplyPostCallback>) -> Div {
    let root_id = root.id.clone();
    let author_handle = root.author.handle.clone();
    div()
        .w_full()
        .px(theme.spacing_md)
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
                .child(format!("{REPLY_TO_LABEL} {author_handle}")),
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
