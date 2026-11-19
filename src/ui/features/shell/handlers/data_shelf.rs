//! The Saved view's per-item actions in the shell (sb F2): notes, opening an item, and
//! "Learn this collection".

use super::super::view::AppView;
use crate::domain::data::saved::{collection_map, CollectionEntry, ItemRef};
use crate::ui::features::data::{tr, SavedTarget};
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Opens the note editor on `item`, filled with its note.
    pub(in crate::ui::features::shell) fn edit_note(&mut self, item: ItemRef, window: &mut Window, cx: &mut Context<Self>) {
        let note = self.shelf_items().into_iter().find(|e| e.item == item).map(|e| e.note).unwrap_or_default();
        self.data.inputs.note.update(cx, |input, cx| {
            input.set_value(note, window, cx);
            input.focus(window, cx);
        });
        self.data.editing_note = Some(item);
    }

    /// Saves the note being edited (collections only: bookmarks carry no notes).
    pub(in crate::ui::features::shell) fn save_note(&mut self, cx: &mut Context<Self>) {
        let Some(item) = self.data.editing_note.take() else { return };
        if !matches!(self.data.target, SavedTarget::Collection(_)) {
            return;
        }
        let note = self.data.inputs.note.read(cx).value().trim().to_string();
        let entry = self.shelf_items().into_iter().find(|e| e.item == item);
        if let Some(entry) = entry {
            self.store_entries(vec![CollectionEntry { note, ..entry }]);
        }
    }

    /// Shows a post in the timeline or a document's vault entry.
    pub(in crate::ui::features::shell) fn open_saved_item(&mut self, item: ItemRef, window: &mut Window, cx: &mut Context<Self>) {
        match item {
            ItemRef::Post(id) => self.reveal_post(id, window, cx),
            ItemRef::Document(id) => {
                self.data.showing_saved = false;
                self.open_vault_document(id.as_str(), window, cx);
            }
        }
    }

    /// The shelf as a connection map in the Review page's editor (sb C1b).
    pub(in crate::ui::features::shell) fn learn_shelf(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let items = self.shelf_items();
        if items.is_empty() {
            return;
        }
        let name = match &self.data.target {
            SavedTarget::Bookmarks => tr("data.saved.bookmarks"),
            SavedTarget::Collection(id) => self.state.library.collection(id).map(|c| c.name.clone()).unwrap_or_default(),
        };
        let map = collection_map(t!("data.saved.map_title", name = name).to_string(), &items);
        self.open_map_editor(map, None, window, cx);
    }
}
