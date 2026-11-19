//! Bookmarks and collection items (sb F2): one load at startup, then single-row writes.

use super::{collect_rows, db_err, parse_timestamp, Database};
use crate::domain::data::saved::{Bookmark, CollectionEntry, ItemRef, SavedShelf};
use crate::domain::document::CollectionId;
use crate::domain::PostId;
use crate::error::Result;
use rusqlite::{params, Row};

const SELECT_BOOKMARKS: &str = "SELECT post_id, created_at FROM bookmarks ORDER BY created_at DESC, post_id";
const INSERT_BOOKMARK: &str = "INSERT OR REPLACE INTO bookmarks (post_id, created_at) VALUES (?1, ?2)";
const DELETE_BOOKMARK: &str = "DELETE FROM bookmarks WHERE post_id = ?1";
const SELECT_ENTRIES: &str = "SELECT collection_id, item_kind, item_id, position, note, added_at FROM collection_items ORDER BY collection_id, position, added_at";
const UPSERT_ENTRY: &str = r#"
    INSERT INTO collection_items (collection_id, item_kind, item_id, position, note, added_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
    ON CONFLICT(collection_id, item_kind, item_id) DO UPDATE SET position = excluded.position, note = excluded.note
"#;
const DELETE_ENTRY: &str = "DELETE FROM collection_items WHERE collection_id = ?1 AND item_kind = ?2 AND item_id = ?3";
const DELETE_COLLECTION_ENTRIES: &str = "DELETE FROM collection_items WHERE collection_id = ?1";

fn entry_from_row(row: &Row<'_>) -> rusqlite::Result<Option<CollectionEntry>> {
    let (collection, kind, id): (String, String, String) = (row.get(0)?, row.get(1)?, row.get(2)?);
    let (position, note, added_at): (i64, String, String) = (row.get(3)?, row.get(4)?, row.get(5)?);
    Ok(ItemRef::from_parts(&kind, &id).map(|item| CollectionEntry { collection: CollectionId::new(collection), item, position, note, added_at: parse_timestamp(&added_at) }))
}

impl Database {
    /// Every bookmark (newest first) and collection item.
    pub fn load_saved_shelf(&self) -> Result<SavedShelf> {
        let mut stmt = self.conn.prepare(SELECT_BOOKMARKS).map_err(db_err)?;
        let rows = stmt.query_map([], |row| Ok(Bookmark { post: PostId::new(row.get::<_, String>(0)?), created_at: parse_timestamp(&row.get::<_, String>(1)?) })).map_err(db_err)?;
        let bookmarks = collect_rows(rows)?;
        let mut stmt = self.conn.prepare(SELECT_ENTRIES).map_err(db_err)?;
        let entries = collect_rows(stmt.query_map([], entry_from_row).map_err(db_err)?)?.into_iter().flatten().collect();
        Ok(SavedShelf { bookmarks, entries })
    }

    pub fn save_bookmark(&self, bookmark: &Bookmark) -> Result<()> {
        self.conn.execute(INSERT_BOOKMARK, params![bookmark.post.as_str(), crate::time::rfc3339(bookmark.created_at)]).map_err(db_err)?;
        Ok(())
    }

    pub fn delete_bookmark(&self, post: &PostId) -> Result<()> {
        self.conn.execute(DELETE_BOOKMARK, [post.as_str()]).map_err(db_err)?;
        Ok(())
    }

    /// Inserts or updates entries (place and note), in one transaction.
    pub fn save_collection_entries(&self, entries: &[CollectionEntry]) -> Result<()> {
        let transaction = self.conn.unchecked_transaction().map_err(db_err)?;
        for e in entries {
            let values = params![e.collection.as_str(), e.item.kind(), e.item.id(), e.position, e.note, crate::time::rfc3339(e.added_at)];
            self.conn.execute(UPSERT_ENTRY, values).map_err(db_err)?;
        }
        transaction.commit().map_err(db_err)
    }

    /// Takes `item` out of `collection`.
    pub fn delete_collection_entry(&self, collection: &CollectionId, item: &ItemRef) -> Result<()> {
        self.conn.execute(DELETE_ENTRY, params![collection.as_str(), item.kind(), item.id()]).map_err(db_err)?;
        Ok(())
    }

    /// Forgets every item of a deleted collection.
    pub fn delete_collection_entries(&self, collection: &CollectionId) -> Result<()> {
        self.conn.execute(DELETE_COLLECTION_ENTRIES, [collection.as_str()]).map_err(db_err)?;
        Ok(())
    }
}
