//! The grounding callout of the inspector, shown while the focused thought has no replies.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{div, Div, FontWeight, ParentElement, Styled};
use rust_i18n::t;

/// Grounding context insight shown when the focused thought has no replies yet.
pub(crate) fn render_grounding_callout(theme: &ThemeTokens) -> Div {
    div()
        .m(theme.panel_padding)
        .p(theme.card_padding)
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
                .child(Icon::new(LucideIcon::Sparkles).size(theme.font_size_sm).text_color(theme.accent_brand_text))
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
