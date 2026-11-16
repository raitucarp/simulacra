//! Parent post context shown above the editor in reply mode.

use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use gpui_kit::*;

pub(super) fn render_reply_context(parent: &Post, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .px(theme.spacing_lg)
        .pt(theme.spacing_md)
        .flex()
        .gap(theme.spacing_md)
        .child(parent_avatar_column(parent, theme))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(theme.spacing_2xs)
                .child(
                    div()
                        .font_family(theme.font_family_display.clone())
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_size(theme.font_size_xs)
                        .text_color(theme.text_secondary)
                        .child(parent.author.name.clone()),
                )
                .child(
                    div()
                        .text_size(theme.font_size_sm)
                        .text_color(theme.text_muted)
                        .truncate()
                        .child(parent.content.clone()),
                ),
        )
}

/// Parent author's avatar with a threadline leading down to the reply editor.
fn parent_avatar_column(parent: &Post, theme: &ThemeTokens) -> Div {
    div()
        .w(theme.post_avatar_col_width)
        .flex()
        .flex_col()
        .items_center()
        .child(render_profile_avatar(&parent.author, theme.avatar_size_sm, theme))
        .child(
            div()
                .w(theme.border_width_threadline)
                .flex_1()
                .min_h(theme.threadline_min_height)
                .bg(theme.border_subtle)
                .rounded(theme.radius_full)
                .mt(theme.spacing_2xs),
        )
}
