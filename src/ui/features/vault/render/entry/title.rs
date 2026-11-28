//! Title block of an entry (kind, title, byline, imprint) and its actions.

use super::super::common::{badge, meta_line, progress_bar};
use crate::domain::document::record::{CreatorRole, ReadingStatus};
use crate::domain::GroundingDocument;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::screen::Screen;
use crate::ui::features::vault::text::{kind_label, status_label};
use crate::ui::features::vault::VaultEvent;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const READ_BUTTON_ID: &str = "vault-entry-read";
const EDIT_BUTTON_ID: &str = "vault-entry-edit";
const MARK_BUTTON_ID: &str = "vault-entry-mark";
const DELETE_BUTTON_ID: &str = "vault-entry-delete";
const PASSAGES_BUTTON_ID: &str = "vault-entry-learn-passages";
const CONNECTIONS_BUTTON_ID: &str = "vault-entry-learn-connections";
const HISTORY_BUTTON_ID: &str = "vault-entry-history";
const QUOTE_BUTTON_ID: &str = "vault-entry-quote";

/// "Baruch Spinoza · translated by R. H. M. Elwes".
fn byline(doc: &GroundingDocument) -> Vec<String> {
    let names = |role| doc.record.creators.iter().filter(|c| c.role == role).map(|c| c.name.clone()).collect::<Vec<_>>().join(", ");
    let (translators, editors) = (names(CreatorRole::Translator), names(CreatorRole::Editor));
    vec![
        names(CreatorRole::Author),
        if translators.is_empty() { String::new() } else { t!("vault.translated_by", names = translators).to_string() },
        if editors.is_empty() { String::new() } else { t!("vault.edited_by", names = editors).to_string() },
    ]
}

/// Whether any item has text.
fn has_any(items: &[String]) -> bool {
    items.iter().any(|item| !item.trim().is_empty())
}

impl VaultPanel {
    pub(super) fn render_title_block(&self, doc: &GroundingDocument, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let record = &doc.record;
        let reading = record.reading;
        let imprint = vec![record.date.clone(), record.publisher.clone(), record.classification.place.clone()];
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(
                div()
                    .flex()
                    .gap(theme.control_gap)
                    .child(badge(LucideIcon::BookOpen, kind_label(record.kind), theme))
                    .child(badge(LucideIcon::Bookmark, status_label(reading.status), theme)),
            )
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(doc.title.clone()))
            .when(!record.subtitle.is_empty(), |b| b.child(div().text_size(theme.font_size_md).text_color(theme.text_secondary).child(record.subtitle.clone())))
            .when(has_any(&byline(doc)), |b| b.child(meta_line(byline(doc), theme).text_size(theme.font_size_sm).text_color(theme.text_secondary)))
            .when(has_any(&imprint), |b| b.child(meta_line(imprint.clone(), theme)))
            .when(reading.status == ReadingStatus::Reading, |b| b.child(div().max_w(theme.vault_search_width).child(progress_bar(reading.progress, theme))))
            .child(self.render_entry_actions(doc, cx))
    }

    fn render_entry_actions(&self, doc: &GroundingDocument, cx: &mut Context<Self>) -> impl IntoElement {
        let (id, edit_id, mark_id) = (doc.id.clone(), doc.id.clone(), doc.id.clone());
        let reading = doc.record.reading;
        let read_label = match reading.status {
            ReadingStatus::Reading => t!("vault.continue_reading", progress = reading.progress).to_string(),
            _ => t!("vault.read").to_string(),
        };
        let is_read = reading.status == ReadingStatus::Read;
        let read = Button::new(READ_BUTTON_ID).primary().small().icon(Icon::new(LucideIcon::BookOpen)).label(read_label)
            .on_click(cx.listener(move |this, _, _, cx| this.go(Screen::Reader(id.clone()), cx)));
        let edit = Button::new(EDIT_BUTTON_ID).ghost().small().icon(Icon::new(LucideIcon::Edit)).label(t!("vault.edit").to_string())
            .on_click(cx.listener(move |this, _, window, cx| this.open_editor(Some(edit_id.clone()), window, cx)));
        let mark_label = if is_read { t!("vault.mark_unread") } else { t!("vault.mark_read") }.to_string();
        let mark = Button::new(MARK_BUTTON_ID).ghost().small().icon(Icon::new(LucideIcon::Check)).label(mark_label)
            .on_click(cx.listener(move |this, _, _, cx| this.mark_read(mark_id.clone(), !is_read, cx)));
        let history_id = doc.id.clone(); // sb F2
        let history = Button::new(HISTORY_BUTTON_ID).ghost().small().icon(Icon::new(LucideIcon::RotateCcwClock)).label(t!("data.history.button").to_string()).on_click(cx.listener(move |_, _, _, cx| cx.emit(VaultEvent::History(history_id.clone()))));
        let quote_id = doc.id.clone(); // N7
        let quote = Button::new(QUOTE_BUTTON_ID).ghost().small().icon(Icon::new(LucideIcon::Quote)).label(t!("posts.pinned.button").to_string()).on_click(cx.listener(move |_, _, _, cx| cx.emit(VaultEvent::Quote(quote_id.clone()))));
        div().flex().flex_wrap().gap(self.theme.control_gap).child(read).child(edit).child(mark).children(self.learn_buttons(doc, cx)).child(history).child(quote).child(self.render_delete_button(doc, cx))
    }

    /// "Learn passages" and "Learn connections" (sb C1b).
    fn learn_buttons(&self, doc: &GroundingDocument, cx: &mut Context<Self>) -> [Button; 2] {
        let (passages_id, connections_id) = (doc.id.clone(), doc.id.clone());
        let passages = Button::new(PASSAGES_BUTTON_ID).ghost().small().icon(Icon::new(LucideIcon::GraduationCap)).label(t!("vault.learn_passages").to_string())
            .on_click(cx.listener(move |_, _, _, cx| cx.emit(VaultEvent::LearnPassages(passages_id.clone()))));
        let connections = Button::new(CONNECTIONS_BUTTON_ID).ghost().small().icon(Icon::new(LucideIcon::Share2)).label(t!("vault.learn_connections").to_string())
            .on_click(cx.listener(move |_, _, _, cx| cx.emit(VaultEvent::LearnConnections(connections_id.clone()))));
        [passages, connections]
    }

    /// Delete, after a confirmation naming the document.
    fn render_delete_button(&self, doc: &GroundingDocument, cx: &mut Context<Self>) -> impl IntoElement {
        let panel = cx.entity().downgrade();
        let (id, title) = (doc.id.clone(), doc.title.clone());
        Button::new(DELETE_BUTTON_ID)
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::Trash))
            .label(t!("vault.delete").to_string())
            .on_click(move |_, window, cx| super::super::doc_menu::confirm_delete(panel.clone(), id.clone(), title.clone(), window, cx))
    }
}
