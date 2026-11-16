//! Knowledge Vault view for document grounding and semantic vector memory.

mod doc_card;
mod form;

use std::rc::Rc;

use crate::domain::GroundingDocument;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, App, Div, Entity, FontWeight, InteractiveElement, IntoElement, ParentElement, Styled, Window};
use rust_i18n::t;

pub type AddDocCallback = Box<dyn Fn(String, String, &mut Window, &mut App) + 'static>;
pub type DeleteDocCallback = Box<dyn Fn(String, &mut Window, &mut App) + 'static>;

const VAULT_SCROLL_ID: &str = "knowledge-vault-scroll-container";

fn render_header(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .text_size(theme.font_size_lg)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child(t!("vault.title").to_string()),
        )
        .child(
            div()
                .font_family(theme.font_family_ui.clone())
                .text_size(theme.font_size_xs)
                .text_color(theme.text_secondary)
                .child(t!("vault.subtitle").to_string()),
        )
}

fn render_documents_section<F>(documents: &[GroundingDocument], on_delete: Rc<F>, theme: &ThemeTokens) -> Div
where
    F: Fn(String, &mut Window, &mut App) + 'static,
{
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::BOLD)
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
        .children(
            documents
                .iter()
                .enumerate()
                .map(|(idx, doc)| doc_card::render_doc_card(idx, doc, on_delete.clone(), theme)),
        )
}

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

    div()
        .id(VAULT_SCROLL_ID)
        .flex_1()
        .h_full()
        .overflow_y_scrollbar()
        .p(theme.spacing_xl)
        .bg(theme.bg_app)
        .flex()
        .flex_col()
        .gap(theme.spacing_xl)
        .child(render_header(theme))
        .child(form::render_add_form(doc_title_input, doc_content_input, on_add, theme))
        .child(render_documents_section(documents, on_delete, theme))
}
