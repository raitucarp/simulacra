//! Grounding knowledge card molecule with proper layout placement and Lucide icons.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

const VECTOR_RAG_BADGE: &str = "Vector RAG";

fn render_title(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(div().w_4().h_4().text_color(theme.accent_alt).child(Icon::new(LucideIcon::Library)))
        .child(
            div()
                .text_xs()
                .font_bold()
                .text_color(theme.text_primary)
                .child(t!("sidebar.grounding_knowledge").to_string()),
        )
}

fn render_badge(theme: &ThemeTokens) -> Div {
    div()
        .px_1p5()
        .py_0p5()
        .rounded_md()
        .bg(theme.hover_bg)
        .border_1()
        .border_color(theme.border)
        .text_xs()
        .font_medium()
        .text_color(theme.text_muted)
        .child(VECTOR_RAG_BADGE)
}

/// Clean, dedicated "upload document" action button.
fn render_upload_button(theme: &ThemeTokens) -> Div {
    let accent_user = theme.accent_user;
    div()
        .w_full()
        .py_1p5()
        .px_3()
        .rounded_lg()
        .bg(theme.hover_bg)
        .border_1()
        .border_color(theme.border)
        .cursor_pointer()
        .hover(move |s| s.border_color(accent_user))
        .flex()
        .items_center()
        .justify_center()
        .gap_1p5()
        .text_xs()
        .font_medium()
        .text_color(theme.accent_user)
        .child(div().w_3p5().h_3p5().child(Icon::new(LucideIcon::Plus)))
        .child(t!("sidebar.upload_doc").to_string())
}

/// Atomic Grounding Knowledge Card with spacious, well-aligned Add Document placement.
pub fn render_knowledge_card(theme: &ThemeTokens) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap_2p5()
        // Header Row: Title & Category Icon
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(render_title(theme))
                .child(render_badge(theme)),
        )
        // Description
        .child(
            div()
                .text_xs()
                .text_color(theme.text_secondary)
                .child(t!("sidebar.grounding_desc").to_string()),
        )
        .child(render_upload_button(theme))
}
