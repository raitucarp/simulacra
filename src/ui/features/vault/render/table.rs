//! The table: one row per document, columns sortable where a sort exists.

use super::cover_view::cover;
use super::shelf::first_creator;
use crate::domain::document::query::SortKey;
use crate::domain::GroundingDocument;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::screen::Screen;
use crate::ui::features::vault::text::kind_label;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use crate::ui::features::vault::text::tr;

/// Date format of the "Added" column.
const ADDED_FORMAT: &str = "%Y-%m-%d";

/// A column: its header key, width (none = fills), sort, and whether compact windows show it.
struct Column {
    key: &'static str,
    width: Option<Pixels>,
    sort: Option<SortKey>,
    compact: bool,
}

impl VaultPanel {
    fn columns(&self) -> [Column; 6] {
        let t = &self.theme;
        [
            Column { key: "title", width: None, sort: Some(SortKey::Title), compact: true },
            Column { key: "creator", width: Some(t.vault_column_creator_width), sort: None, compact: false },
            Column { key: "kind", width: Some(t.vault_column_kind_width), sort: None, compact: false },
            Column { key: "published", width: Some(t.vault_column_year_width), sort: Some(SortKey::DatePublished), compact: true },
            Column { key: "added", width: Some(t.vault_column_added_width), sort: Some(SortKey::DateAdded), compact: false },
            Column { key: "links", width: Some(t.vault_column_links_width), sort: Some(SortKey::MostLinked), compact: true },
        ]
    }

    pub(super) fn render_table(&self, compact: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let header: Vec<AnyElement> =
            self.columns().into_iter().filter(|c| c.compact || !compact).map(|c| self.render_header_cell(c, cx)).collect();
        let rows: Vec<AnyElement> = self.result_documents().into_iter().map(|doc| self.render_row(doc, compact, cx)).collect();
        div()
            .flex()
            .flex_col()
            .rounded(theme.radius_lg)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .bg(theme.bg_card)
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(theme.control_gap)
                    .px(theme.card_padding)
                    .py(theme.spacing_sm)
                    .bg(theme.bg_elevated)
                    .border_b(theme.border_width_hairline)
                    .border_color(theme.border)
                    .children(header),
            )
            .children(rows)
    }

    fn render_header_cell(&self, column: Column, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let (active, descending) = self.query.effective_sort();
        let arrow = column.sort.filter(|key| *key == active).map(|_| if descending { LucideIcon::ArrowDown } else { LucideIcon::ArrowUp });
        let cell = div()
            .id(ElementId::Name(format!("vault-col-{}", column.key).into()))
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .text_size(theme.font_size_2xs)
            .font_weight(FontWeight::BOLD)
            .text_color(if arrow.is_some() { theme.text_primary } else { theme.text_muted })
            .child(tr(format!("vault.column.{}", column.key)).to_uppercase())
            .when_some(arrow, |c, icon| c.child(Icon::new(icon).size(theme.icon_size_xs)));
        let cell = match column.sort {
            Some(key) => cell.cursor_pointer().on_click(cx.listener(move |this, _, _, cx| this.sort_by(key, cx))),
            None => cell,
        };
        sized(cell, column.width).into_any_element()
    }

    fn render_row(&self, doc: &GroundingDocument, compact: bool, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let id = doc.id.clone();
        let hover = theme.hover_bg;
        let links = self.data.link_counts.get(&doc.id).copied().unwrap_or_default();
        let muted = |text: String| div().text_size(theme.font_size_xs).text_color(theme.text_secondary).truncate().child(text);
        let cells: Vec<(AnyElement, Option<Pixels>, bool)> = vec![
            (self.render_title_cell(doc).into_any_element(), None, true),
            (muted(first_creator(doc)).into_any_element(), Some(theme.vault_column_creator_width), false),
            (muted(kind_label(doc.record.kind)).into_any_element(), Some(theme.vault_column_kind_width), false),
            (muted(doc.record.published_year().map(|y| y.to_string()).unwrap_or_default()).into_any_element(), Some(theme.vault_column_year_width), true),
            (muted(doc.created_at.strftime(ADDED_FORMAT).to_string()).into_any_element(), Some(theme.vault_column_added_width), false),
            (muted(links.to_string()).into_any_element(), Some(theme.vault_column_links_width), true),
        ];
        div()
            .id(ElementId::Name(format!("vault-row-{}", doc.id).into()))
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .px(theme.card_padding)
            .py(theme.spacing_sm)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .cursor_pointer()
            .hover(move |s| s.bg(hover))
            .on_click(cx.listener(move |this, _, _, cx| this.go(Screen::Entry(id.clone()), cx)))
            .children(cells.into_iter().filter(|(_, _, keep)| *keep || !compact).map(|(cell, width, _)| sized(div().child(cell), width)))
            .into_any_element()
    }

    fn render_title_cell(&self, doc: &GroundingDocument) -> impl IntoElement {
        let theme = &self.theme;
        let subtitle = doc.record.subtitle.clone();
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .min_w_0()
            .child(cover(doc, theme.vault_cover_thumb_width, theme))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .min_w_0()
                    .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).truncate().child(doc.title.clone()))
                    .when(!subtitle.is_empty(), |d| d.child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).truncate().child(subtitle))),
            )
    }
}

/// `cell` at a fixed `width`, or filling the remaining room.
fn sized<E: Styled>(cell: E, width: Option<Pixels>) -> E {
    match width {
        Some(width) => cell.w(width).flex_shrink_0(),
        None => cell.flex_1().min_w_0(),
    }
}
