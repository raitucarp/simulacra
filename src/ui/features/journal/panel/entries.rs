//! Writing entries: a new one (blank, on today's prompt or from a template), editing one,
//! saving, deleting, another prompt, the preview, and "Turn into a post".

use super::{EntryEditor, JournalPanel};
use crate::domain::journal::records::find;
use crate::domain::journal::templates::{template_body, Template};
use crate::domain::journal::{JournalEntry, RecordId};
use crate::ui::features::journal::events::JournalEvent;
use crate::ui::features::journal::tab::JournalTab;
use crate::ui::features::journal::text::tr;
use jiff::Timestamp;
use gpui_kit::{Context, Window};

impl JournalPanel {
    /// Opens the editor on a new entry for today (with today's prompt, or a template).
    pub fn new_entry(&mut self, template: Option<Template>, with_prompt: bool, window: &mut Window, cx: &mut Context<Self>) {
        let body = template.map(|t| template_body(&t.section_keys().iter().map(|k| tr(k)).collect::<Vec<_>>())).unwrap_or_default();
        let prompt = with_prompt.then(|| self.current_prompt());
        self.editor = Some(EntryEditor { id: None, prompt, template, preview: false });
        self.open_editor_with(body, window, cx);
    }

    /// Opens the entry `id` (from the palette).
    pub fn open_entry(&mut self, id: &RecordId, window: &mut Window, cx: &mut Context<Self>) {
        self.edit_entry(id, window, cx);
    }

    /// Opens the editor on the entry `id`.
    pub(in crate::ui::features::journal) fn edit_entry(&mut self, id: &RecordId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = find(&self.data.book.entries, id).cloned() else { return };
        self.editor = Some(EntryEditor { id: Some(entry.id), prompt: entry.prompt, template: entry.template, preview: false });
        self.open_editor_with(entry.body, window, cx);
    }

    fn open_editor_with(&mut self, body: String, window: &mut Window, cx: &mut Context<Self>) {
        self.tab = JournalTab::Journal;
        self.fields.entry.update(cx, |state, cx| {
            state.set_value(body, window, cx);
            state.focus(window, cx);
        });
        cx.notify();
    }

    /// Saves the open entry (a blank one is not kept); the editor stays on it.
    pub(in crate::ui::features::journal) fn save_entry(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let Some(editor) = self.editor.clone() else { return };
        let body = super::JournalFields::area_text(&self.fields.entry, cx);
        if body.trim().is_empty() {
            return;
        }
        let now = Timestamp::now();
        let existing = editor.id.as_ref().and_then(|id| find(&self.data.book.entries, id)).cloned();
        let entry = match existing {
            Some(entry) => entry.edited(body, now),
            None => JournalEntry { prompt: editor.prompt, template: editor.template, ..JournalEntry::new(RecordId::random(), self.data.today, now, body) },
        };
        self.editor = Some(EntryEditor { id: Some(entry.id.clone()), ..editor });
        cx.emit(JournalEvent::SaveEntry(entry));
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn close_editor(&mut self, cx: &mut Context<Self>) {
        self.editor = None;
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn delete_entry(&mut self, id: RecordId, cx: &mut Context<Self>) {
        if self.editor.as_ref().and_then(|e| e.id.as_ref()) == Some(&id) {
            self.editor = None;
        }
        cx.emit(JournalEvent::DeleteEntry(id));
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn another_prompt(&mut self, cx: &mut Context<Self>) {
        self.prompt = Some(self.current_prompt().next());
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn toggle_preview(&mut self, cx: &mut Context<Self>) {
        if let Some(editor) = self.editor.as_mut() {
            editor.preview = !editor.preview;
            cx.notify();
        }
    }

    /// Opens the composer with the selected text of the editor (all of it without a selection).
    pub(in crate::ui::features::journal) fn turn_into_post(&mut self, cx: &mut Context<Self>) {
        let state = self.fields.entry.read(cx);
        let selected = state.selected_text().to_string();
        let text = if selected.trim().is_empty() { state.value().to_string() } else { selected };
        if !text.trim().is_empty() {
            cx.emit(JournalEvent::TurnIntoPost(text.trim().to_string()));
        }
    }

    /// "Turn into a post" on a saved entry in the list.
    pub(in crate::ui::features::journal) fn entry_into_post(&mut self, id: &RecordId, cx: &mut Context<Self>) {
        if let Some(entry) = find(&self.data.book.entries, id) {
            cx.emit(JournalEvent::TurnIntoPost(entry.body.trim().to_string()));
        }
    }
}
