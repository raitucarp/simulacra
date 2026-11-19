//! Ordering collection items, filing documents that have no entry yet, and toggling
//! bookmarks: pure functions returning the new rows to store.

use super::{Bookmark, CollectionEntry, ItemRef};
use crate::domain::document::{CollectionId, DocumentId};
use crate::domain::PostId;
use jiff::Timestamp;

/// The ordered items of `collection`.
///
/// - Post entries are kept while `exists` says the post is still there.
/// - Document entries are kept while the document is still filed under the collection
///   (`members`, from the documents' records).
/// - Member documents without an entry follow, in `members` order, with no note.
pub fn collection_items(collection: &CollectionId, entries: &[CollectionEntry], members: &[DocumentId], exists: &dyn Fn(&ItemRef) -> bool, now: Timestamp) -> Vec<CollectionEntry> {
    let keeps = |entry: &&CollectionEntry| match &entry.item {
        ItemRef::Post(_) => exists(&entry.item),
        ItemRef::Document(id) => members.contains(id),
    };
    let mut items: Vec<CollectionEntry> = entries.iter().filter(|e| &e.collection == collection).filter(keeps).cloned().collect();
    items.sort_by(|a, b| a.position.cmp(&b.position).then(a.added_at.cmp(&b.added_at)));
    let start = next_position(&items, collection);
    let unfiled = members.iter().filter(|id| !items.iter().any(|e| e.item == ItemRef::Document((*id).clone()))).cloned().collect::<Vec<_>>();
    let appended = unfiled.into_iter().enumerate().map(|(offset, id)| CollectionEntry {
        collection: collection.clone(),
        item: ItemRef::Document(id),
        position: start + offset as i64,
        note: String::new(),
        added_at: now,
    });
    items.extend(appended);
    items
}

/// The position after the last item of `collection`.
pub fn next_position(entries: &[CollectionEntry], collection: &CollectionId) -> i64 {
    entries.iter().filter(|e| &e.collection == collection).map(|e| e.position + 1).max().unwrap_or(0)
}

/// `items` with the item at `from` moved to `to`, renumbered from 0 (every row changes
/// place, so every row is returned to be stored).
pub fn moved(items: &[CollectionEntry], from: usize, to: usize) -> Vec<CollectionEntry> {
    let mut order = items.to_vec();
    if from < order.len() {
        let item = order.remove(from);
        order.insert(to.min(order.len()), item);
    }
    order.into_iter().enumerate().map(|(position, entry)| CollectionEntry { position: position as i64, ..entry }).collect()
}

/// `bookmarks` without `post` when it is bookmarked, else with it first (newest first).
pub fn toggled_bookmark(bookmarks: &[Bookmark], post: &PostId, now: Timestamp) -> Vec<Bookmark> {
    if bookmarks.iter().any(|b| &b.post == post) {
        return bookmarks.iter().filter(|b| &b.post != post).cloned().collect();
    }
    std::iter::once(Bookmark { post: post.clone(), created_at: now }).chain(bookmarks.iter().cloned()).collect()
}
