//! Related entries (shared subjects, discipline or meaning) and the posts linking here.

use super::super::common::{card, meta_line, titled_section};
use super::super::cover_view::cover;
use crate::domain::document::links::{related_entries, RelatedEntry, RelatedReason};
use crate::domain::GroundingDocument;
use crate::ui::features::vault::data::BacklinkPreview;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::screen::Screen;
use crate::ui::features::vault::VaultEvent;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;

/// Most related entries shown.
const RELATED_LIMIT: usize = 4;
/// Date format of a backlink.
const BACKLINK_DATE_FORMAT: &str = "%Y-%m-%d";

fn reason_label(reason: RelatedReason) -> String {
    match reason {
        RelatedReason::SharedSubjects(count) => crate::i18n::t_count("vault.related.shared_subjects", count),
        RelatedReason::SameDiscipline => t!("vault.related.same_discipline").to_string(),
        RelatedReason::Semantic => t!("vault.related.semantic").to_string(),
    }
}

impl VaultPanel {
    fn quiet_note(&self, text: String) -> Div {
        div().text_size(self.theme.font_size_xs).text_color(self.theme.text_muted).child(text)
    }

    pub(super) fn render_related(&self, doc: &GroundingDocument, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let semantic = self.data.semantic.get(&doc.id).map(Vec::as_slice).unwrap_or_default();
        let entries = related_entries(doc, &self.data.documents, &self.data.tree, semantic, RELATED_LIMIT);
        let body = if entries.is_empty() {
            self.quiet_note(t!("vault.related.none").to_string())
        } else {
            div().flex().flex_col().gap(theme.spacing_xs).children(entries.into_iter().filter_map(|entry| self.render_related_row(entry, cx)))
        };
        div().flex().flex_col().gap(theme.stack_gap).child(titled_section(LucideIcon::Compass, t!("vault.related.title").to_string(), theme)).child(body)
    }

    fn render_related_row(&self, entry: RelatedEntry, cx: &mut Context<Self>) -> Option<AnyElement> {
        let theme = &self.theme;
        let doc = self.data.document(entry.id.as_str())?;
        let (id, hover) = (doc.id.clone(), theme.hover_bg);
        let creator = doc.record.authors().first().map(|c| c.name.clone()).unwrap_or_default();
        let row = div()
            .id(ElementId::Name(format!("related-{}", doc.id).into()))
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .p(theme.spacing_xs)
            .rounded(theme.radius_md)
            .cursor_pointer()
            .hover(move |s| s.bg(hover))
            .on_click(cx.listener(move |this, _, _, cx| this.go(Screen::Entry(id.clone()), cx)))
            .child(cover(doc, theme.vault_cover_thumb_width, theme))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).truncate().child(doc.title.clone()))
                    .child(meta_line(vec![creator, reason_label(entry.reason)], theme)),
            );
        Some(row.into_any_element())
    }

    pub(super) fn render_backlinks(&self, doc: &GroundingDocument, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let links = self.data.backlinks.get(&doc.id).cloned().unwrap_or_default();
        let title = crate::i18n::t_count("vault.backlinks", links.len());
        let body = if links.is_empty() {
            self.quiet_note(t!("vault.no_backlinks", title = doc.title.clone()).to_string())
        } else {
            div().flex().flex_col().gap(theme.stack_gap).children(links.into_iter().map(|link| self.render_backlink(link, cx)))
        };
        div().flex().flex_col().gap(theme.stack_gap).child(titled_section(LucideIcon::Link2, title, theme)).child(body)
    }

    fn render_backlink(&self, link: BacklinkPreview, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let post = link.post_id.clone();
        let border = theme.accent_brand_border;
        card(theme)
            .id(ElementId::Name(format!("backlink-{}", link.post_id).into()))
            .cursor_pointer()
            .hover(move |s| s.border_color(border))
            .on_click(cx.listener(move |_, _, _, cx| cx.emit(VaultEvent::OpenPost(post.clone()))))
            .child(meta_line(vec![link.author.clone(), link.created_at.strftime(BACKLINK_DATE_FORMAT).to_string()], theme))
            .child(div().text_size(theme.font_size_sm).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(link.excerpt))
            .into_any_element()
    }
}
