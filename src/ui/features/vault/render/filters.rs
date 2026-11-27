//! The active filters above the results: the count, one removable chip per chosen value,
//! and "clear filters".

use crate::domain::document::facets::Facet;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::text::facet_value_label;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

const CLEAR_FILTERS_ID: &str = "vault-clear-filters";

impl VaultPanel {
    /// "2 of 3 · Philosophy × · Book × · Clear filters" while anything narrows the results.
    pub(super) fn render_active_filters(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !self.query.is_filtering() || self.data.documents.is_empty() {
            return None;
        }
        let theme = &self.theme;
        let count = t!("vault.showing", shown = self.results.len(), total = self.data.documents.len()).to_string();
        let chips: Vec<AnyElement> = self
            .query
            .selection
            .pairs()
            .map(|(facet, key)| self.render_filter_chip(facet, key.to_string(), cx))
            .collect();
        let clear = (!self.query.selection.is_empty()).then(|| {
            div()
                .id(CLEAR_FILTERS_ID)
                .text_size(theme.font_size_xs)
                .text_color(theme.accent_brand_text)
                .cursor_pointer()
                .on_click(cx.listener(|this, _, _, cx| this.clear_filters(cx)))
                .child(t!("vault.clear_filters").to_string())
        });
        let row = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(theme.control_gap)
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(count))
            .children(chips)
            .children(clear);
        Some(row.into_any_element())
    }

    fn render_filter_chip(&self, facet: Facet, key: String, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let label = facet_value_label(facet, &key, &self.data.library);
        div()
            .id(ElementId::Name(format!("filter-{}-{key}", facet.key()).into()))
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .px(theme.spacing_sm)
            .py(theme.spacing_2xs)
            .rounded(theme.radius_full)
            .bg(theme.accent_brand_subtle)
            .border(theme.border_width_hairline)
            .border_color(theme.accent_brand_border)
            .text_size(theme.font_size_xs)
            .text_color(theme.text_primary)
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_facet_value(facet, &key, cx)))
            .child(label)
            .child(Icon::new(LucideIcon::X).size(theme.icon_size_xs).text_color(theme.text_muted))
            .into_any_element()
    }
}
