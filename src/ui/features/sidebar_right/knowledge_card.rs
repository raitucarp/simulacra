//! Grounding knowledge card molecule: document count and the "add document" file import.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::Button;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use crate::i18n::t_count;
use rust_i18n::t;

/// Element id of the card's import button.
const UPLOAD_DOC_BUTTON_ID: &str = "sidebar-upload-doc-btn";

/// Imports a text file into the active network's vault.
pub type ImportDocumentCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

fn render_title(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(div().w(theme.icon_size_2xl).h(theme.icon_size_2xl).text_color(theme.accent_alt).child(Icon::new(LucideIcon::Library)))
        .child(
            div()
                .text_size(theme.font_size_label)
                .font_bold()
                .text_color(theme.text_primary)
                .child(t!("sidebar.grounding_knowledge").to_string()),
        )
}

/// Real number of documents in the active network's vault.
fn render_badge(doc_count: usize, theme: &ThemeTokens) -> Div {
    div()
        .px(theme.spacing_xs_plus)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_sm)
        .bg(theme.hover_bg)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .text_size(theme.font_size_label)
        .font_medium()
        .text_color(theme.text_muted)
        .child(t_count("sidebar.doc_count", doc_count))
}

/// Full-width "add document" button opening the native file picker.
fn render_upload_button(on_import: ImportDocumentCallback) -> Button {
    Button::new(UPLOAD_DOC_BUTTON_ID)
        .outline()
        .small()
        .w_full()
        .icon(Icon::new(LucideIcon::Plus))
        .label(t!("sidebar.upload_doc").to_string())
        .on_click(move |_, window, cx| on_import(window, cx))
}

/// Atomic Grounding Knowledge Card with spacious, well-aligned Add Document placement.
pub fn render_knowledge_card(
    doc_count: usize,
    on_import: ImportDocumentCallback,
    theme: &ThemeTokens,
) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_sm_plus)
        // Header Row: Title & document count
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(render_title(theme))
                .child(render_badge(doc_count, theme)),
        )
        // Description
        .child(
            div()
                .text_size(theme.font_size_label)
                .text_color(theme.text_secondary)
                .child(t!("sidebar.grounding_desc").to_string()),
        )
        .child(render_upload_button(on_import))
}
