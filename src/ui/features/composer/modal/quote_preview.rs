//! Card previewing the post being quoted, shown under the editor in quote mode.

use crate::domain::Post;
use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Card previewing the post being quoted.
pub(super) fn render_quote_preview(target: &Post, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .bg(theme.bg_card)
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .font_weight(FontWeight::SEMIBOLD)
                .text_size(theme.font_size_xs)
                .text_color(theme.text_primary)
                .child(format!("{} {}", target.author.name, target.author.display_handle())),
        )
        .child(
            div()
                .text_size(theme.font_size_sm)
                .text_color(theme.text_secondary)
                .child(target.content.clone()),
        )
}
