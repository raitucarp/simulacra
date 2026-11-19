//! Where an entry is filed (subjects with their broader terms, collections, classification)
//! and its contents (the outline of its headings).

use super::super::common::{tag, titled_section};
use crate::domain::document::facets::Facet;
use crate::domain::document::text::{blocks, outline, MIN_OUTLINE_ENTRIES};
use crate::domain::GroundingDocument;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::screen::Screen;
use crate::ui::features::vault::text::{discipline_label, form_label, period_label};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;

/// Separator of a subject's broader terms ("Philosophy › Metaphysics").
const PATH_SEPARATOR: &str = " › ";

impl VaultPanel {
    /// Tags of every filing of `doc` (each opens the catalogue filtered by it).
    fn filing_tags(&self, doc: &GroundingDocument) -> Vec<(Facet, String, String, LucideIcon)> {
        let class = &doc.record.classification;
        let subjects = doc.record.subjects.iter().map(|id| {
            let path: Vec<String> = self.data.tree.path(id).iter().map(|s| s.label.clone()).collect();
            let label = if path.is_empty() { id.as_str().to_string() } else { path.join(PATH_SEPARATOR) };
            (Facet::Subject, id.as_str().to_string(), label, LucideIcon::Tag)
        });
        let collections = doc.record.collections.iter().filter_map(|id| self.data.library.collection(id)).map(|c| {
            (Facet::Collection, c.id.as_str().to_string(), c.name.clone(), LucideIcon::Folder)
        });
        let classes = [
            class.discipline.map(|d| (Facet::Discipline, d.key().to_string(), discipline_label(d), LucideIcon::Library)),
            class.form.map(|f| (Facet::Form, f.key().to_string(), form_label(f), LucideIcon::FileText)),
            class.period.map(|p| (Facet::Period, p.key().to_string(), period_label(p), LucideIcon::Clock)),
            (!class.place.trim().is_empty()).then(|| (Facet::Place, class.place.trim().to_string(), class.place.trim().to_string(), LucideIcon::MapPin)),
        ];
        subjects.chain(collections).chain(classes.into_iter().flatten()).collect()
    }

    pub(super) fn render_filing(&self, doc: &GroundingDocument, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let tags: Vec<AnyElement> = self
            .filing_tags(doc)
            .into_iter()
            .map(|(facet, key, label, icon)| {
                let id = ElementId::Name(format!("entry-tag-{}-{key}", facet.key()).into());
                tag(id, Some(icon), label, theme)
                    .on_click(cx.listener(move |this, _, window, cx| this.browse_only(facet, &key, window, cx)))
                    .into_any_element()
            })
            .collect();
        let body = if tags.is_empty() {
            div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("vault.not_filed").to_string())
        } else {
            div().flex().flex_wrap().gap(theme.control_gap).children(tags)
        };
        div().flex().flex_col().gap(theme.stack_gap).child(titled_section(LucideIcon::Tag, t!("vault.filed_under").to_string(), theme)).child(body)
    }

    /// The outline of a text with headings; each heading opens the reader there.
    pub(super) fn render_contents(&self, doc: &GroundingDocument, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let theme = &self.theme;
        let entries = outline(&blocks(&doc.content));
        if entries.len() < MIN_OUTLINE_ENTRIES {
            return None;
        }
        let hover = theme.hover_bg;
        let rows = entries.into_iter().map(|entry| {
            let (id, block) = (doc.id.clone(), entry.block);
            let indent = theme.vault_tree_indent * f32::from(entry.level.saturating_sub(1));
            div()
                .id(ElementId::Name(format!("entry-toc-{}", entry.block).into()))
                .pl(indent)
                .py(theme.spacing_2xs)
                .rounded(theme.radius_sm)
                .text_size(theme.font_size_sm)
                .text_color(theme.text_secondary)
                .cursor_pointer()
                .hover(move |s| s.bg(hover))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.pending_block = Some(block);
                    this.go(Screen::Reader(id.clone()), cx);
                }))
                .child(entry.title)
        });
        let list = div().flex().flex_col().children(rows);
        Some(div().flex().flex_col().gap(theme.stack_gap).child(titled_section(LucideIcon::List, t!("vault.contents").to_string(), theme)).child(list))
    }
}
