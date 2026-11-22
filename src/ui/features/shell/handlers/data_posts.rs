//! A post's data actions (sb F2): Edit opens the composer in edit mode and saving keeps the
//! old text as a revision; Bookmark toggles; the collection items file the post in a vault
//! collection (or ask for a new one on the Saved view).

use super::super::view::AppView;
use crate::domain::data::saved::{next_position, toggled_bookmark, CollectionEntry, ItemRef};
use crate::domain::document::CollectionId;
use crate::domain::preferences::ToastKind;
use crate::domain::post::Tint;
use crate::domain::symbols::PostSymbols;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{tr, DataEvent, SavedTarget};
use jiff::Timestamp;
use gpui_kit::component::notification::Notification;
use gpui_kit::*;

impl AppView {
    pub(in crate::ui::features::shell) fn on_post_data_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            DataEvent::EditPost(id) => self.open_post_editor(&id, window, cx),
            DataEvent::SetArchived(id, archived) => self.set_archived(&id, archived, window, cx),
            DataEvent::SetAutoArchive(id, after) => {
                let set = self.state.set_auto_archive(&id, after);
                self.report_data_error(set, window, cx);
            }
            DataEvent::SetAutoDelete(id, after) => self.request_auto_delete(id, after, window, cx),
            DataEvent::ToggleBookmark(id) => self.toggle_bookmark(&id, window, cx),
            DataEvent::AddToCollection(id, collection) => self.file_post(&id, &collection, window, cx),
            DataEvent::RemoveFromCollection(id, collection) => self.remove_entry(&collection, ItemRef::Post(id), cx),
            DataEvent::NewCollectionFor(id) => {
                self.data.pending_add = Some(id);
                self.show_saved(window, cx);
                self.data.inputs.collection_name.update(cx, |input, cx| input.focus(window, cx));
            }
            _ => {}
        }
    }

    /// The composer on one of the user's own posts.
    fn open_post_editor(&mut self, id: &PostId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(id).filter(|p| self.state.author_of(p).is_human_persona()) else { return };
        let target = self.state.authored(post);
        self.composer_modal.update(cx, |panel, cx| panel.open_edit(target, window, cx));
    }

    /// Saves the composer's edit of `id`: its text (the previous one becomes a revision),
    /// then its tint (sb C3) and symbols (S5), which keep no revision.
    pub(crate) fn save_post_edit(&mut self, id: PostId, (text, tint, symbols): (String, Option<Tint>, PostSymbols), window: &mut Window, cx: &mut Context<Self>) {
        let saved = self.state.edit_post(&id, &text).and_then(|revision| self.state.set_post_tint(&id, tint).map(|_| revision));
        let saved = saved.and_then(|revision| self.state.set_post_symbols(&id, symbols).map(|_| revision));
        match saved {
            Ok(revision) => {
                if revision.is_some() {
                    *self.data.revision_counts.entry(id.as_str().to_string()).or_default() += 1;
                }
                self.close_composer_modal(window, cx);
                self.toast(ToastKind::Confirmation, Notification::success(tr("data.edit.saved")), window, cx);
            }
            Err(err) => {
                let message = publish_error_message(&err);
                self.composer_modal.update(cx, |panel, cx| panel.set_error(message, cx));
            }
        }
    }

    /// Archives the post `id` or brings it back to the feed (Y5).
    fn set_archived(&mut self, id: &PostId, archived: bool, window: &mut Window, cx: &mut Context<Self>) {
        match self.state.set_archived(id, archived) {
            Ok(_) => {
                let text = if archived { tr("data.archived") } else { tr("data.unarchived") };
                self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
            }
            Err(err) => self.notify_data(publish_error_message(&err), window, cx),
        }
    }

    /// Says why a post action failed; a success shows itself on the card.
    fn report_data_error<T>(&self, result: crate::error::Result<T>, window: &mut Window, cx: &mut Context<Self>) {
        if let Err(err) = result {
            self.notify_data(publish_error_message(&err), window, cx);
        }
    }

    fn toggle_bookmark(&mut self, id: &PostId, window: &mut Window, cx: &mut Context<Self>) {
        let was = self.data.shelf.is_bookmarked(id);
        self.data.shelf.bookmarks = toggled_bookmark(&self.data.shelf.bookmarks, id, Timestamp::now());
        if let Some(vault) = self.state.db.as_ref() {
            let saved = self.data.shelf.bookmarks.iter().find(|b| &b.post == id).cloned();
            let _ = match saved {
                Some(bookmark) => vault.save_bookmark(bookmark),
                None => vault.delete_bookmark(id.clone()),
            };
        }
        let text = if was { tr("data.saved.unbookmarked") } else { tr("data.saved.bookmarked") };
        self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
    }

    /// Adds post `id` at the end of `collection`.
    pub(in crate::ui::features::shell) fn file_post(&mut self, id: &PostId, collection: &CollectionId, window: &mut Window, cx: &mut Context<Self>) {
        let item = ItemRef::Post(id.clone());
        if self.data.shelf.entries.iter().any(|e| &e.collection == collection && e.item == item) {
            return;
        }
        let entry = CollectionEntry { collection: collection.clone(), item, position: next_position(&self.data.shelf.entries, collection), note: String::new(), added_at: Timestamp::now() };
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_collection_entries(vec![entry.clone()]);
        }
        self.data.shelf.entries.push(entry);
        let name = self.state.library.collection(collection).map(|c| c.name.clone()).unwrap_or_default();
        let note = Notification::success(rust_i18n::t!("data.saved.added", name = name).to_string());
        self.toast(ToastKind::Confirmation, note, window, cx);
    }

    /// Takes `item` out of `collection` (a document is unfiled through its record).
    pub(in crate::ui::features::shell) fn remove_entry(&mut self, collection: &CollectionId, item: ItemRef, cx: &mut Context<Self>) {
        if let ItemRef::Document(doc) = &item {
            self.unfile_document(doc.as_str(), collection, cx);
        }
        self.data.shelf.entries.retain(|e| !(&e.collection == collection && e.item == item));
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.delete_collection_entry(collection.clone(), item);
        }
    }

    fn unfile_document(&mut self, id: &str, collection: &CollectionId, cx: &mut Context<Self>) {
        let Some(doc) = self.state.documents.iter().find(|d| d.id == id).cloned() else { return };
        let collections = doc.record.collections.iter().filter(|c| *c != collection).cloned().collect();
        let record = crate::domain::document::CatalogueRecord { collections, ..doc.record.clone() };
        let _ = self.state.update_document(id, &doc.title, &doc.content, record);
        self.push_vault_data(cx);
    }

    /// The Knowledge page's Saved view.
    pub(in crate::ui::features::shell) fn show_saved(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.data.showing_saved = true;
        if let SavedTarget::Collection(id) = &self.data.target {
            if self.state.library.collection(id).is_none() {
                self.data.target = SavedTarget::Bookmarks;
            }
        }
        self.select_nav(crate::state::NavigationTab::Knowledge, window, cx);
    }
}
