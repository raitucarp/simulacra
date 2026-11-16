//! Card for one grounded document in the Knowledge Vault corpus list.

use std::rc::Rc;

use crate::domain::GroundingDocument;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{
    div, App, Div, FontWeight, InteractiveElement, ParentElement, Stateful,
    StatefulInteractiveElement, Styled, Window,
};
use rust_i18n::t;

fn render_title_row(doc: &GroundingDocument, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .text_size(theme.font_size_sm)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child(doc.title.clone()),
        )
        .child(
            div()
                .px(theme.spacing_sm)
                .py(theme.spacing_2xs)
                .rounded(theme.radius_full)
                .bg(theme.hover_bg)
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_xs)
                .text_color(theme.accent_agent)
                .child(t!("vault.chunks_count", count = doc.chunk_count).to_string()),
        )
}

fn render_remove_button<F>(idx: usize, doc_id: String, on_delete: Rc<F>, theme: &ThemeTokens) -> Stateful<Div>
where
    F: Fn(String, &mut Window, &mut App) + 'static,
{
    let danger_color = theme.accent_danger;
    div()
        .id(format!("remove-doc-btn-{}", idx))
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .text_size(theme.font_size_xs)
        .text_color(theme.text_muted)
        .hover(move |s| s.text_color(danger_color))
        .on_click(move |_, window, cx| {
            on_delete(doc_id.clone(), window, cx);
        })
        .child(Icon::new(LucideIcon::Trash).size(theme.font_size_sm))
        .child(t!("vault.remove_doc").to_string())
}

fn render_status_row(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .pt(theme.spacing_xs)
        .child(
            div()
                .w(theme.spacing_xs)
                .h(theme.spacing_xs)
                .rounded(theme.radius_full)
                .bg(theme.accent_agent),
        )
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_xs)
                .text_color(theme.text_muted)
                .child(t!("vault.grounded_status").to_string()),
        )
}

/// Renders a grounded document card with chunk count, removal action and status.
pub(super) fn render_doc_card<F>(
    idx: usize,
    doc: &GroundingDocument,
    on_delete: Rc<F>,
    theme: &ThemeTokens,
) -> Stateful<Div>
where
    F: Fn(String, &mut Window, &mut App) + 'static,
{
    div()
        .id(format!("ground-doc-card-{}", idx))
        .w_full()
        .p(theme.spacing_lg)
        .rounded(theme.radius_lg)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .bg(theme.bg_card)
        .flex()
        .flex_col()
        .gap(theme.spacing_sm)
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(render_title_row(doc, theme))
                .child(render_remove_button(idx, doc.id.clone(), on_delete, theme)),
        )
        .child(
            div()
                .font_family(theme.font_family_ui.clone())
                .text_size(theme.font_size_xs)
                .text_color(theme.text_secondary)
                .child(doc.content.clone()),
        )
        .child(render_status_row(theme))
}
