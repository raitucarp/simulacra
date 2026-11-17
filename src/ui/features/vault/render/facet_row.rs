//! One facet value row: an optional tree toggle, the label and its count.

use super::facets::FacetRow;
use crate::domain::document::facets::Facet;
use crate::ui::features::vault::panel::VaultPanel;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

impl VaultPanel {
    /// A selectable value row; values with no documents are dimmed.
    pub(super) fn render_value_row(&self, facet: Facet, row: FacetRow, lead: Option<AnyElement>, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let hover = theme.hover_bg;
        let key = row.key.clone();
        let indent = theme.vault_tree_indent * row.depth as f32;
        div()
            .id(row.id)
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .pl(indent)
            .pr(theme.spacing_xs)
            .py(theme.spacing_2xs)
            .rounded(theme.radius_sm)
            .cursor_pointer()
            .text_size(theme.font_size_xs)
            .when(row.selected, |r| r.bg(theme.accent_brand_subtle).text_color(theme.text_primary).font_weight(FontWeight::SEMIBOLD))
            .when(!row.selected, |r| r.text_color(theme.text_secondary).hover(move |s| s.bg(hover)))
            .when(row.count == 0 && !row.selected, |r| r.opacity(theme.vault_muted_opacity))
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_facet_value(facet, &key, cx)))
            .children(lead)
            .child(div().flex_1().min_w_0().truncate().child(row.label))
            .child(
                div()
                    .font_family(theme.font_family_mono.clone())
                    .text_size(theme.font_size_2xs)
                    .text_color(if row.selected { theme.accent_brand } else { theme.text_muted })
                    .child(row.count.to_string()),
            )
            .into_any_element()
    }
}
