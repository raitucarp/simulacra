//! "Add grounding document" form card of the Knowledge Vault.

use std::rc::Rc;

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::component::Icon;
use gpui_kit::{div, App, Div, Entity, FontWeight, IntoElement, ParentElement, Styled, Window};
use rust_i18n::t;

const SUBMIT_DOC_BUTTON_ID: &str = "submit-grounding-doc-btn";
const VECTOR_INDEX_LABEL: &str = "HNSW Vector Index";

fn render_add_button<F>(
    doc_title_input: &Entity<InputState>,
    doc_content_input: &Entity<TextareaState>,
    on_add: Rc<F>,
) -> Button
where
    F: Fn(String, String, &mut Window, &mut App) + 'static,
{
    let doc_title_ent = doc_title_input.clone();
    let doc_content_ent = doc_content_input.clone();
    Button::new(SUBMIT_DOC_BUTTON_ID)
        .primary()
        .icon(Icon::new(LucideIcon::Plus))
        .label(t!("vault.ground_button").to_string())
        .on_click(move |_, window, cx| {
            let title = doc_title_ent.read(cx).value().to_string();
            let content = doc_content_ent.read(cx).value().to_string();
            if !title.trim().is_empty() && !content.trim().is_empty() {
                on_add(title, content, window, cx);
            }
        })
}

fn render_form_header(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_muted)
                .child(t!("vault.add_doc").to_string()),
        )
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.accent_agent)
                .child(VECTOR_INDEX_LABEL),
        )
}

/// Inset field frame wrapping an input or textarea.
fn field_frame(field: impl IntoElement, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .bg(theme.bg_app)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .child(field)
}

/// Renders the add-document form card.
pub(super) fn render_add_form<F>(
    doc_title_input: &Entity<InputState>,
    doc_content_input: &Entity<TextareaState>,
    on_add: Rc<F>,
    theme: &ThemeTokens,
) -> Div
where
    F: Fn(String, String, &mut Window, &mut App) + 'static,
{
    let add_btn = render_add_button(doc_title_input, doc_content_input, on_add);
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
        .child(render_form_header(theme))
        .child(field_frame(Input::new(doc_title_input).bordered(false), theme))
        .child(field_frame(Textarea::new(doc_content_input).bordered(false), theme))
        .child(div().flex().justify_end().child(add_btn))
}
