//! The facet column: the subject tree, collections and the classification facets, each
//! value with the number of documents it would leave.

use super::common::section_heading;
use crate::domain::document::facets::{Facet, FacetCount};
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::text::{facet_label, facet_value_label};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::collections::HashMap;

const FACETS_SCROLL_ID: &str = "vault-facets";
/// Facets listed under the subject tree, in order.
const FLAT_FACETS: [Facet; 8] =
    [Facet::Collection, Facet::Kind, Facet::Status, Facet::Discipline, Facet::Form, Facet::Period, Facet::Place, Facet::Language];

/// A facet value row: label, count, selection.
pub(super) struct FacetRow {
    pub id: ElementId,
    pub key: String,
    pub label: String,
    pub count: usize,
    pub selected: bool,
    pub depth: usize,
}

impl VaultPanel {
    pub(super) fn render_facets(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let groups: Vec<AnyElement> = std::iter::once(Facet::Subject)
            .chain(FLAT_FACETS)
            .filter_map(|facet| self.render_facet_group(facet, cx))
            .collect();
        div()
            .id(FACETS_SCROLL_ID)
            .size_full()
            .overflow_y_scrollbar()
            .p(theme.panel_padding_compact)
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(div().pt(theme.spacing_sm).child(section_heading(t!("vault.browse_by").to_string(), theme)))
            .children(groups)
    }

    /// The rows of a flat facet: values with documents, plus chosen ones (collections list
    /// every shelf of the network).
    fn flat_rows(&self, facet: Facet) -> Vec<FacetRow> {
        let mut counts = self.facet_values(facet);
        if facet == Facet::Collection {
            for collection in self.data.library.collections_of(&self.data.network_id) {
                if !counts.iter().any(|c| c.key == collection.id.as_str()) {
                    counts.push(FacetCount { key: collection.id.as_str().to_string(), count: 0, selected: false });
                }
            }
        }
        counts
            .into_iter()
            .map(|c| FacetRow {
                id: ElementId::Name(format!("facet-{}-{}", facet.key(), c.key).into()),
                label: facet_value_label(facet, &c.key, &self.data.library),
                key: c.key,
                count: c.count,
                selected: c.selected,
                depth: 0,
            })
            .collect()
    }

    /// The subject tree's visible rows with their counts.
    fn subject_rows(&self) -> Vec<(FacetRow, Option<crate::domain::document::SubjectId>, bool)> {
        let counts: HashMap<String, FacetCount> = self.facet_values(Facet::Subject).into_iter().map(|c| (c.key.clone(), c)).collect();
        self.data
            .tree
            .visible_rows(&self.expanded_subjects)
            .into_iter()
            .map(|row| {
                let key = row.subject.id.as_str();
                let found = counts.get(key);
                let facet_row = FacetRow {
                    id: ElementId::Name(format!("facet-subject-{key}").into()),
                    key: key.to_string(),
                    label: row.subject.label.clone(),
                    count: found.map_or(0, |c| c.count),
                    selected: found.is_some_and(|c| c.selected),
                    depth: row.depth,
                };
                let expanded = self.expanded_subjects.contains(&row.subject.id);
                (facet_row, row.has_children.then(|| row.subject.id.clone()), expanded)
            })
            .collect()
    }

    fn render_facet_group(&self, facet: Facet, cx: &mut Context<Self>) -> Option<AnyElement> {
        let collapsed = self.collapsed_facets.contains(&facet);
        let rows: Vec<AnyElement> = if facet == Facet::Subject {
            self.subject_rows().into_iter().map(|(row, toggle, open)| self.render_subject_row(facet, row, toggle, open, cx)).collect()
        } else {
            self.flat_rows(facet).into_iter().map(|row| self.render_value_row(facet, row, None, cx)).collect()
        };
        if rows.is_empty() {
            return None;
        }
        let theme = &self.theme;
        let chevron = if collapsed { LucideIcon::ChevronRight } else { LucideIcon::ChevronDown };
        let header = div()
            .id(ElementId::Name(format!("facet-group-{}", facet.key()).into()))
            .flex()
            .items_center()
            .justify_between()
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_facet_group(facet, cx)))
            .child(section_heading(facet_label(facet), theme))
            .child(Icon::new(chevron).size(theme.icon_size_xs).text_color(theme.text_muted));
        let group = div().flex().flex_col().gap(theme.spacing_2xs).child(header).when(!collapsed, |g| g.children(rows));
        Some(group.into_any_element())
    }

    fn render_subject_row(
        &self,
        facet: Facet,
        row: FacetRow,
        toggle: Option<crate::domain::document::SubjectId>,
        open: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let theme = &self.theme;
        let id = ElementId::Name(format!("{}-toggle", row.id).into());
        let chevron = toggle.map(|subject| {
            let icon = if open { LucideIcon::ChevronDown } else { LucideIcon::ChevronRight };
            div()
                .id(id)
                .cursor_pointer()
                .text_color(theme.text_muted)
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.toggle_subject(&subject, cx);
                }))
                .child(Icon::new(icon).size(theme.icon_size_xs))
                .into_any_element()
        });
        let spacer = || div().w(theme.icon_size_xs).into_any_element();
        self.render_value_row(facet, row, Some(chevron.unwrap_or_else(spacer)), cx)
    }
}
