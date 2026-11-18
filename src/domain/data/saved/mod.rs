//! Collections and bookmarks (sb F2).
//!
//! - **Collections** are the vault's collections (`collections` table, [`CollectionId`]),
//!   extended so one collection holds posts and documents together, in order, with a note per
//!   item. A document's membership stays where the vault keeps it (its catalogue record);
//!   [`CollectionEntry`] rows add the order and the note, and are the membership of posts.
//! - **Bookmarks** mark any post, newest first.

mod learn;
mod order;

pub use learn::collection_map;
pub use order::{collection_items, moved, next_position, toggled_bookmark};

use crate::domain::document::{CollectionId, DocumentId};
use crate::domain::PostId;
use chrono::{DateTime, Utc};

/// `item_kind` of a post entry.
pub const POST_ITEM: &str = "post";
/// `item_kind` of a document entry.
pub const DOCUMENT_ITEM: &str = "document";

/// What a collection entry points at.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemRef {
    Post(PostId),
    Document(DocumentId),
}

impl ItemRef {
    /// Stable key stored in `collection_items.item_kind`. Never rename one.
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Post(_) => POST_ITEM,
            Self::Document(_) => DOCUMENT_ITEM,
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Post(id) => id.as_str(),
            Self::Document(id) => id.as_str(),
        }
    }

    /// The reference stored as `(kind, id)`, if the kind is known.
    pub fn from_parts(kind: &str, id: &str) -> Option<Self> {
        match kind {
            POST_ITEM => Some(Self::Post(PostId::new(id))),
            DOCUMENT_ITEM => Some(Self::Document(DocumentId::new(id))),
            _ => None,
        }
    }
}

/// One item of a collection: its place, its note and when it was added.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionEntry {
    pub collection: CollectionId,
    pub item: ItemRef,
    pub position: i64,
    pub note: String,
    pub added_at: DateTime<Utc>,
}

/// A bookmarked post.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookmark {
    pub post: PostId,
    pub created_at: DateTime<Utc>,
}

/// Every bookmark and collection entry, as loaded from the vault.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SavedShelf {
    pub bookmarks: Vec<Bookmark>,
    pub entries: Vec<CollectionEntry>,
}

impl SavedShelf {
    pub fn is_bookmarked(&self, post: &PostId) -> bool {
        self.bookmarks.iter().any(|b| &b.post == post)
    }

    /// Ids of the collections holding `item`.
    pub fn collections_of(&self, item: &ItemRef) -> Vec<CollectionId> {
        self.entries.iter().filter(|e| &e.item == item).map(|e| e.collection.clone()).collect()
    }

    /// Every stored entry of `collection`.
    pub fn entries_of<'a>(&'a self, collection: &'a CollectionId) -> impl Iterator<Item = &'a CollectionEntry> + 'a {
        self.entries.iter().filter(move |e| &e.collection == collection)
    }
}

#[cfg(test)]
mod tests;
