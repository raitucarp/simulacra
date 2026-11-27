//! The shelf: covers in reading order, each with title, creator and a reading mark.

use super::common::{meta_line, progress_bar};
use super::cover_view::cover;
use crate::domain::document::record::ReadingStatus;
use crate::domain::GroundingDocument;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::screen::Screen;
use crate::ui::features::vault::text::kind_label;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// The first creator's name ("Baruch Spinoza"), or nothing.
pub(super) fn first_creator(doc: &GroundingDocument) -> String {
    doc.record.authors().first().map(|c| c.name.clone()).unwrap_or_default()
}

/// "1677 · Book".
pub(super) fn year_and_kind(doc: &GroundingDocument) -> Vec<String> {
    vec![doc.record.published_year().map(|y| y.to_string()).unwrap_or_default(), kind_label(doc.record.kind)]
}

impl VaultPanel {
    pub(super) fn render_shelf(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let cards: Vec<AnyElement> = self.result_documents().into_iter().map(|doc| self.render_card(doc, cx)).collect();
        div().flex().flex_wrap().gap(theme.section_gap).children(cards)
    }

    fn render_card(&self, doc: &GroundingDocument, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let id = doc.id.clone();
        let reading = doc.record.reading;
        let hover = theme.hover_bg;
        let mark = self.reading_mark(reading);
        div()
            .id(ElementId::Name(format!("vault-card-{}", doc.id).into()))
            .w(theme.vault_cover_width)
            .flex_shrink_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_sm)
            .cursor_pointer()
            .rounded(theme.radius_md)
            .hover(move |s| s.bg(hover))
            .on_click(cx.listener(move |this, _, _, cx| this.go(Screen::Entry(id.clone()), cx)))
            .child(cover(doc, theme.vault_cover_width, theme))
            .child(
                div()
                    .text_size(theme.font_size_sm)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.text_primary)
                    .line_height(relative(theme.line_height_body))
                    .line_clamp(2).text_ellipsis()
                    .child(doc.title.clone()),
            )
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).truncate().child(first_creator(doc)))
            .child(meta_line(year_and_kind(doc), theme).text_size(theme.font_size_2xs))
            .when_some(mark, |card, mark| card.child(mark))
            .into_any_element()
    }

    /// A progress bar while reading, a check once read, nothing before.
    fn reading_mark(&self, reading: crate::domain::document::record::Reading) -> Option<AnyElement> {
        let theme = &self.theme;
        match reading.status {
            ReadingStatus::Unread => None,
            ReadingStatus::Reading => Some(progress_bar(reading.progress, theme).into_any_element()),
            ReadingStatus::Read => Some(
                div()
                    .flex()
                    .items_center()
                    .gap(theme.inline_gap)
                    .text_size(theme.font_size_2xs)
                    .text_color(theme.accent_success)
                    .child(Icon::new(LucideIcon::Check).size(theme.icon_size_xs))
                    .child(crate::ui::features::vault::text::status_label(ReadingStatus::Read))
                    .into_any_element(),
            ),
        }
    }
}
