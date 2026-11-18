//! The entry editor: the prompt it answers, the Markdown text field (or its preview, where
//! `[[wikilinks]]` to posts and vault documents become pills that open them), and Save,
//! Preview, "Turn into a post" (the selected text, or all of it) and Close.

use super::super::events::JournalEvent;
use super::super::panel::JournalPanel;
use super::super::text::{day_label, tr};
use super::common::{area_field, caption, control_row, ghost_button, primary_button, surface};
use crate::domain::journal::records::find;
use crate::domain::post::tokenize_post_body;
use crate::ui::features::post::{render_body_tokens, BodyLinks, OpenTextAction};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use std::rc::Rc;

const SAVE_ID: &str = "journal-editor-save";
const PREVIEW_ID: &str = "journal-editor-preview";
const POST_ID: &str = "journal-editor-post";
const CLOSE_ID: &str = "journal-editor-close";

impl JournalPanel {
    pub(super) fn render_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let Some(editor) = self.editor.clone() else { return div() };
        let saved = editor.id.as_ref().and_then(|id| find(&self.data.book.entries, id));
        let day = saved.map(|e| e.day).unwrap_or(self.data.today);
        let status = if saved.is_some() { tr("journal.editor.saved") } else { tr("journal.editor.unsaved") };
        let prompt = editor.prompt.map(|p| div().pl(theme.card_padding).border_l(theme.border_width_threadline).border_color(theme.accent_dialectic).text_size(theme.font_size_sm).text_color(theme.text_secondary).child(tr(&p.text_key())));
        let body = if editor.preview { self.entry_preview(cx).into_any_element() } else { area_field(&self.fields.entry, theme).min_h(theme.journal_editor_min_height).into_any_element() };
        surface(theme)
            .child(control_row(theme).child(div().flex_1().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(day_label(day))).child(caption(status, theme)))
            .children(prompt)
            .child(body)
            .child(caption(tr("journal.editor.links_hint"), theme))
            .child(self.editor_buttons(editor.preview, cx))
    }

    fn editor_buttons(&self, preview: bool, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let save = primary_button(SAVE_ID, LucideIcon::Check, tr("journal.editor.save")).on_click(cx.listener(|this, _, window, cx| this.save_entry(window, cx)));
        let (icon, label) = if preview { (LucideIcon::NotebookPen, tr("journal.editor.write")) } else { (LucideIcon::Eye, tr("journal.editor.preview")) };
        let toggle = ghost_button(PREVIEW_ID, icon, label).on_click(cx.listener(|this, _, _, cx| this.toggle_preview(cx)));
        let post = ghost_button(POST_ID, LucideIcon::Send, tr("journal.editor.into_post")).on_click(cx.listener(|this, _, _, cx| this.turn_into_post(cx)));
        let close = ghost_button(CLOSE_ID, LucideIcon::X, tr("journal.editor.close")).on_click(cx.listener(|this, _, window, cx| {
            this.save_entry(window, cx);
            this.close_editor(cx);
        }));
        control_row(theme).child(save).child(toggle).child(post).child(close)
    }

    /// The text as it reads, with wikilink pills that open their post or document.
    fn entry_preview(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let text = super::super::panel::JournalFields::area_text(&self.fields.entry, cx);
        let entity = cx.entity().downgrade();
        let on_wikilink: OpenTextAction = Rc::new(move |target, _, cx| {
            let _ = entity.update(cx, |_, cx| cx.emit(JournalEvent::OpenWikilink(target)));
        });
        let links = BodyLinks { on_wikilink: Some(on_wikilink), ..BodyLinks::default() };
        let previews = Rc::clone(&self.data.wikilinks);
        let tokens = tokenize_post_body(&text);
        div().w_full().min_h(theme.journal_editor_min_height).p(theme.card_padding).rounded(theme.radius_md).bg(theme.bg_elevated).child(render_body_tokens(&tokens, previews.as_ref(), theme, &links))
    }
}
