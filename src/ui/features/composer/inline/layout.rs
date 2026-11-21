//! Layout pieces shared by the inline composer view and its standalone fallback renderer.

use crate::theme::ThemeTokens;
use crate::ui::atoms::render_char_meter;
use gpui_kit::*;

/// Full-width composer row: avatar column on the left, editor column on the right.
pub(super) fn composer_row(
    avatar_column: impl IntoElement,
    editor_column: impl IntoElement,
    theme: &ThemeTokens,
) -> Div {
    div()
        .w_full()
        .px(theme.panel_padding)
        .py(theme.spacing_md)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.bg_app)
        .flex()
        .gap(theme.spacing_md)
        // Left Column: Avatar + Threads vertical threadline
        .child(avatar_column)
        // Right Column: Textarea & Minimal Actions
        .child(editor_column)
}

/// Avatar with the Threads vertical threadline flowing beneath it.
pub(super) fn avatar_column(avatar: impl IntoElement, theme: &ThemeTokens) -> Div {
    div()
        .flex_shrink_0()
        .w(theme.post_avatar_col_width)
        .h_full()
        .flex()
        .flex_col()
        .items_center()
        .pt(theme.spacing_2xs)
        .child(avatar)
        .child(
            div()
                .w(theme.border_width_threadline)
                .flex_1()
                .min_h(theme.threadline_min_height)
                .bg(theme.border_subtle)
                .rounded(theme.radius_full)
                .mt(theme.spacing_xs)
                .mb(theme.spacing_2xs),
        )
}

/// Textarea area, then the error / limit notice (if any), stacked above the bottom toolbar.
pub(super) fn editor_column(
    editor: impl IntoElement,
    notice: Option<Div>,
    toolbar: impl IntoElement,
    theme: &ThemeTokens,
) -> Div {
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(editor)
        .children(notice)
        // Bottom Toolbar
        .child(toolbar)
}

/// Bottom toolbar: insert and format buttons on the left, character meter and submit on the right.
pub(super) fn toolbar(
    link_button: impl IntoElement,
    tag_button: impl IntoElement,
    format_buttons: impl IntoElement,
    submit_button: impl IntoElement,
    draft_length: usize,
    theme: &ThemeTokens,
) -> Div {
    div()
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .pt(theme.spacing_xs)
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.spacing_xs)
                .child(link_button)
                .child(tag_button)
                .child(format_buttons),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.spacing_md)
                .child(render_char_meter(draft_length, theme))
                .child(submit_button),
        )
}
