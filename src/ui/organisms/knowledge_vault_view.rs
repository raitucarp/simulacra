//! Knowledge Vault view organism for document grounding and semantic vector memory.

use crate::domain::GroundingDocument;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, App, Entity, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

pub type AddDocCallback = Box<dyn Fn(String, String, &mut Window, &mut App) + 'static>;
pub type DeleteDocCallback = Box<dyn Fn(String, &mut Window, &mut App) + 'static>;

/// Renders the Knowledge Vault view with document indexing and management.
pub fn render_knowledge_vault_view(
    documents: &[GroundingDocument],
    theme: &ThemeTokens,
    doc_title_input: &Entity<InputState>,
    doc_content_input: &Entity<TextareaState>,
    on_add_document: impl Fn(String, String, &mut Window, &mut App) + 'static,
    on_delete_document: impl Fn(String, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let on_add = Rc::new(on_add_document);
    let on_delete = Rc::new(on_delete_document);

    let doc_title_ent = doc_title_input.clone();
    let doc_content_ent = doc_content_input.clone();

    let add_btn = Button::new("submit-grounding-doc-btn")
        .primary()
        .icon(Icon::new(LucideIcon::Plus))
        .label(t!("vault.ground_button").to_string())
        .on_click({
            let on_add = on_add.clone();
            move |_, window, cx| {
                let title = doc_title_ent.read(cx).value().to_string();
                let content = doc_content_ent.read(cx).value().to_string();
                if !title.trim().is_empty() && !content.trim().is_empty() {
                    on_add(title, content, window, cx);
                }
            }
        });

    div()
        .id("knowledge-vault-scroll-container")
        .flex_1()
        .h_full()
        .overflow_y_scrollbar()
        .p(theme.spacing_xl)
        .bg(theme.bg_app)
        .flex()
        .flex_col()
        .gap(theme.spacing_xl)
        // Header
        .child(
            div()
                .flex()
                .flex_col()
                .gap(theme.spacing_xs)
                .child(
                    div()
                        .font_family(theme.font_family_display.clone())
                        .text_size(theme.font_size_lg)
                        .font_weight(gpui_kit::FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(t!("vault.title").to_string()),
                )
                .child(
                    div()
                        .font_family(theme.font_family_ui.clone())
                        .text_size(theme.font_size_xs)
                        .text_color(theme.text_secondary)
                        .child(t!("vault.subtitle").to_string()),
                ),
        )
        // Add Document Form Card
        .child(
            div()
                .w_full()
                .p(theme.spacing_lg)
                .rounded(theme.radius_lg)
                .border(theme.border_width_hairline)
                .border_color(theme.border)
                .bg(theme.bg_card)
                .flex()
                .flex_col()
                .gap(theme.spacing_md)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .font_family(theme.font_family_display.clone())
                                .text_size(theme.font_size_xs)
                                .font_weight(gpui_kit::FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child(t!("vault.add_doc").to_string()),
                        )
                        .child(
                            div()
                                .font_family(theme.font_family_mono.clone())
                                .text_size(theme.font_size_xs)
                                .font_weight(gpui_kit::FontWeight::MEDIUM)
                                .text_color(theme.accent_agent)
                                .child("HNSW Vector Index"),
                        ),
                )
                .child(
                    div()
                        .w_full()
                        .rounded(theme.radius_md)
                        .border(theme.border_width_hairline)
                        .border_color(theme.border)
                        .bg(theme.bg_app)
                        .px(theme.spacing_sm)
                        .py(theme.spacing_xs)
                        .child(Input::new(doc_title_input).bordered(false)),
                )
                .child(
                    div()
                        .w_full()
                        .rounded(theme.radius_md)
                        .border(theme.border_width_hairline)
                        .border_color(theme.border)
                        .bg(theme.bg_app)
                        .px(theme.spacing_sm)
                        .py(theme.spacing_xs)
                        .child(Textarea::new(doc_content_input).bordered(false)),
                )
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .child(add_btn),
                ),
        )
        // Documents List Section
        .child(
            div()
                .flex()
                .flex_col()
                .gap(theme.spacing_md)
                .child(
                    div()
                        .font_family(theme.font_family_display.clone())
                        .text_size(theme.font_size_xs)
                        .font_weight(gpui_kit::FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child(t!("vault.corpus_header", count = documents.len()).to_string()),
                )
                .when(documents.is_empty(), |this| {
                    this.child(
                        div()
                            .p(theme.spacing_xl)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(theme.font_size_xs)
                            .text_color(theme.text_muted)
                            .child(t!("vault.no_docs").to_string()),
                    )
                })
                .children(documents.iter().enumerate().map(|(idx, doc)| {
                    let doc_id = doc.id.clone();
                    let del_cb = on_delete.clone();
                    let danger_color = theme.accent_danger;

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
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(theme.spacing_sm)
                                        .child(
                                            div()
                                                .font_family(theme.font_family_display.clone())
                                                .text_size(theme.font_size_sm)
                                                .font_weight(gpui_kit::FontWeight::BOLD)
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
                                        ),
                                )
                                .child(
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
                                            del_cb(doc_id.clone(), window, cx);
                                        })
                                        .child(
                                            Icon::new(LucideIcon::Trash)
                                                .size(theme.font_size_sm),
                                        )
                                        .child(t!("vault.remove_doc").to_string()),
                                ),
                        )
                        .child(
                            div()
                                .font_family(theme.font_family_ui.clone())
                                .text_size(theme.font_size_xs)
                                .text_color(theme.text_secondary)
                                .child(doc.content.clone()),
                        )
                        .child(
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
                                ),
                        )
                })),
        )
}
