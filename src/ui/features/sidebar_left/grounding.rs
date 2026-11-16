//! Grounding Sources channel list (first few documents of the active network).

use super::section::{section_container, section_header};
use super::NavHandle;
use crate::domain::GroundingDocument;
use crate::state::NavigationTab;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{
    div, Div, InteractiveElement, ParentElement, Stateful, StatefulInteractiveElement, Styled,
};
use rust_i18n::t;

/// Maximum number of grounding documents previewed in the sidebar.
const GROUNDING_PREVIEW_LIMIT: usize = 3;
/// Element id prefix of each document row (suffixed with the row index).
const DOC_ROW_ID_PREFIX: &str = "ground-doc-sidebar-";
/// Unit suffix appended to a document's chunk count.
const CHUNK_SUFFIX: &str = "c";

pub(super) fn render_grounding_section(
    docs: Vec<&GroundingDocument>,
    on_select_nav: &NavHandle,
    theme: &ThemeTokens,
) -> Div {
    let header = section_header(
        t!("sidebar.grounding_sources").to_string(),
        docs.len(),
        theme.accent_agent,
        theme,
    );
    let rows = docs
        .into_iter()
        .take(GROUNDING_PREVIEW_LIMIT)
        .enumerate()
        .map(|(idx, doc)| render_doc_row(idx, doc, on_select_nav.clone(), theme));
    section_container(theme).child(header).children(rows)
}

/// One document row; clicking opens the Knowledge tab.
fn render_doc_row(
    idx: usize,
    doc: &GroundingDocument,
    nav: NavHandle,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    div()
        .id(format!("{DOC_ROW_ID_PREFIX}{idx}"))
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_between()
        .hover(move |s| s.bg(hover_bg))
        .on_click(move |_, window, cx| {
            nav(NavigationTab::Knowledge, window, cx);
        })
        .child(render_doc_title(doc.title.clone(), theme))
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_2xs)
                .text_color(theme.text_muted)
                .child(format!("{}{CHUNK_SUFFIX}", doc.chunk_count)),
        )
}

fn render_doc_title(title: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .min_w_0()
        .child(
            div()
                .text_color(theme.text_muted)
                .child(Icon::new(LucideIcon::FileText).size(theme.font_size_xs)),
        )
        .child(
            div()
                .text_size(theme.font_size_xs)
                .text_color(theme.text_primary)
                .min_w_0()
                .truncate()
                .child(title),
        )
}
