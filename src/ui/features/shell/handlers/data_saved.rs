//! The Knowledge page's Saved view in the shell (sb F2): the shelf it shows (bookmarks or a
//! collection's posts and documents in order), its events, and "Learn this collection",
//! which hands the shelf to the Review page as a connection map.

use super::super::view::AppView;
use crate::domain::data::saved::{collection_items, moved, CollectionEntry, ItemRef};
use crate::domain::document::{CollectionId, DocumentId};
use crate::state::semantic::hits::shortened;
use crate::ui::features::data::{format_time, tr, DataEvent, SavedItemView, SavedTarget, SavedView, ShelfChip};
use jiff::Timestamp;
use gpui_kit::*;

/// Characters of a post shown as its row title.
const TITLE_CHARS: usize = 96;
const BYLINE_SEPARATOR: &str = " · ";

impl AppView {
    /// The ordered items of the shelf being shown.
    pub(in crate::ui::features::shell) fn shelf_items(&self) -> Vec<CollectionEntry> {
        match &self.data.target {
            SavedTarget::Bookmarks => self.data.shelf.bookmarks.iter().filter(|b| self.state.get_post(&b.post).is_some()).enumerate().map(|(position, b)| CollectionEntry {
                collection: CollectionId::default(),
                item: ItemRef::Post(b.post.clone()),
                position: position as i64,
                note: String::new(),
                added_at: b.created_at,
            }).collect(),
            SavedTarget::Collection(id) => {
                let members: Vec<DocumentId> = self.state.documents.iter().filter(|d| d.record.collections.contains(id)).map(|d| DocumentId::new(d.id.clone())).collect();
                let exists = |item: &ItemRef| matches!(item, ItemRef::Post(post) if self.state.get_post(post).is_some());
                collection_items(id, &self.data.shelf.entries, &members, &exists, Timestamp::now())
            }
        }
    }

    fn item_view(&self, entry: &CollectionEntry) -> Option<SavedItemView> {
        let (title, subtitle) = match &entry.item {
            ItemRef::Post(id) => {
                let post = self.state.get_post(id)?;
                (shortened(&post.content, TITLE_CHARS), format!("{}{BYLINE_SEPARATOR}{}", self.state.author_of(post).name, format_time(post.created_at)))
            }
            ItemRef::Document(id) => {
                let doc = self.state.documents.iter().find(|d| d.id == id.as_str())?;
                let creators: Vec<&str> = doc.record.creators.iter().map(|c| c.name.as_str()).collect();
                (doc.title.clone(), std::iter::once(tr(&format!("vault.kind.{}", doc.record.kind.key()))).chain(creators.iter().map(|c| c.to_string())).collect::<Vec<_>>().join(BYLINE_SEPARATOR))
            }
        };
        Some(SavedItemView { item: entry.item.clone(), title, subtitle, note: entry.note.clone() })
    }

    /// What the Saved view shows.
    pub(in crate::ui::features::shell) fn saved_view(&self) -> SavedView {
        let bookmarks = ShelfChip { target: SavedTarget::Bookmarks, label: tr("data.saved.bookmarks"), count: self.data.shelf.bookmarks.len() };
        let collections = self.network_collections().into_iter().map(|(id, name)| {
            let posts = self.data.shelf.entries_of(&id).filter(|e| matches!(e.item, ItemRef::Post(_))).count();
            let documents = self.state.documents.iter().filter(|d| d.record.collections.contains(&id)).count();
            ShelfChip { target: SavedTarget::Collection(id), label: name, count: posts + documents }
        });
        SavedView {
            target: self.data.target.clone(),
            chips: std::iter::once(bookmarks).chain(collections).collect(),
            items: self.shelf_items().iter().filter_map(|entry| self.item_view(entry)).collect(),
            reorderable: matches!(self.data.target, SavedTarget::Collection(_)),
            new_collection: self.data.inputs.collection_name.clone(),
            note: self.data.inputs.note.clone(),
            editing_note: self.data.editing_note.clone(),
        }
    }

    pub(in crate::ui::features::shell) fn on_saved_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            DataEvent::ShowLibrary => self.data.showing_saved = false,
            DataEvent::ShowSaved => self.show_saved(window, cx),
            DataEvent::SelectShelf(target) => (self.data.target, self.data.editing_note) = (target, None),
            DataEvent::CreateCollection => self.create_collection(window, cx),
            DataEvent::MoveItem { from, to } => self.move_item(from, to),
            DataEvent::RemoveItem(item) => self.remove_shelf_item(item, window, cx),
            DataEvent::EditNote(item) => self.edit_note(item, window, cx),
            DataEvent::SaveNote => self.save_note(cx),
            DataEvent::CancelNote => self.data.editing_note = None,
            DataEvent::OpenItem(item) => self.open_saved_item(item, window, cx),
            DataEvent::LearnShelf => self.learn_shelf(window, cx),
            _ => {}
        }
    }

    fn create_collection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let name = self.data.inputs.collection_name.read(cx).value().trim().to_string();
        let Some(id) = self.state.ensure_collection(&name) else { return };
        self.data.inputs.collection_name.update(cx, |input, cx| input.set_value("", window, cx));
        if let Some(post) = self.data.pending_add.take() {
            self.file_post(&post, &id, window, cx);
        }
        self.data.target = SavedTarget::Collection(id);
        self.push_vault_data(cx);
    }

    fn move_item(&mut self, from: usize, to: usize) {
        if !matches!(self.data.target, SavedTarget::Collection(_)) || from == to {
            return;
        }
        let order = moved(&self.shelf_items(), from, to);
        self.store_entries(order);
    }

    /// Keeps `entries` in memory and in the vault (replacing the same items).
    pub(in crate::ui::features::shell) fn store_entries(&mut self, entries: Vec<CollectionEntry>) {
        self.data.shelf.entries.retain(|e| !entries.iter().any(|n| n.collection == e.collection && n.item == e.item));
        self.data.shelf.entries.extend(entries.iter().cloned());
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_collection_entries(entries);
        }
    }

    fn remove_shelf_item(&mut self, item: ItemRef, window: &mut Window, cx: &mut Context<Self>) {
        match (&self.data.target, &item) {
            (SavedTarget::Bookmarks, ItemRef::Post(post)) => self.on_post_data_event(DataEvent::ToggleBookmark(post.clone()), window, cx),
            (SavedTarget::Collection(id), _) => self.remove_entry(&id.clone(), item, cx),
            _ => {}
        }
    }
}
