//! Vault operations of version history, bookmarks and collection items (sb F2): startup
//! reads, queued writes, and the synchronous steps of export, snapshot and restore.

use super::Vault;
use crate::domain::data::history::{Revision, RevisionSubject, POST_SUBJECT};
use crate::domain::data::saved::{Bookmark, CollectionEntry, ItemRef, SavedShelf};
use crate::domain::document::CollectionId;
use crate::domain::PostId;
use crate::error::Result;
use crate::storage::db::{verify_database_file, DatabaseSummary, SettingKey};
use std::path::PathBuf;

impl Vault {
    /// Every bookmark and collection item.
    pub fn load_saved_shelf(&self) -> Result<SavedShelf> {
        self.read(|db| db.load_saved_shelf())
    }

    /// How many earlier versions each post has, by post id.
    pub fn post_revision_counts(&self) -> Result<Vec<(String, usize)>> {
        self.read(|db| db.revision_counts(POST_SUBJECT))
    }

    /// Every earlier version of `subject`, newest first (after every queued write).
    pub fn revisions_of(&self, subject: RevisionSubject) -> Result<Vec<Revision>> {
        self.read(move |db| db.revisions_of(&subject))
    }

    pub fn save_revision(&self, revision: Revision) -> Result<()> {
        self.write(move |db| db.save_revision(&revision))
    }

    /// Queues the edited body of `post` with the revision keeping its previous text.
    pub fn edit_post_content(&self, post: crate::domain::Post, revision: Revision) -> Result<()> {
        self.write(move |db| db.edit_post_content(&post, &revision))
    }

    pub fn save_bookmark(&self, bookmark: Bookmark) -> Result<()> {
        self.write(move |db| db.save_bookmark(&bookmark))
    }

    pub fn delete_bookmark(&self, post: PostId) -> Result<()> {
        self.write(move |db| db.delete_bookmark(&post))
    }

    pub fn save_collection_entries(&self, entries: Vec<CollectionEntry>) -> Result<()> {
        self.write(move |db| db.save_collection_entries(&entries))
    }

    pub fn delete_collection_entry(&self, collection: CollectionId, item: ItemRef) -> Result<()> {
        self.write(move |db| db.delete_collection_entry(&collection, &item))
    }

    /// The allow-listed settings rows that hold a value (for a plain export).
    pub fn setting_rows(&self, keys: &'static [SettingKey]) -> Result<Vec<(String, String)>> {
        self.read(move |db| Ok(keys.iter().filter_map(|key| db.get_setting(*key).ok().flatten().map(|value| (key.as_str().to_string(), value))).collect()))
    }

    /// The vault's schema version (the export manifest names it).
    pub fn schema_version(&self) -> Result<i64> {
        self.read(|db| db.schema_version())
    }

    /// A consistent copy of the whole database at `path` (which must not exist).
    pub fn backup_to(&self, path: PathBuf) -> Result<()> {
        self.read(move |db| db.backup_to(&path))
    }

    /// Checks the database at `path` and counts what it holds.
    pub fn verify_database(&self, path: PathBuf) -> Result<DatabaseSummary> {
        self.read(move |_| verify_database_file(&path))
    }

    /// Keeps a copy of the live database at `keep`, then replaces it with `source`.
    pub fn restore_database(&self, source: PathBuf, keep: PathBuf) -> Result<()> {
        self.read(move |db| {
            db.backup_to(&keep)?;
            db.restore_from_file(&source)
        })
    }
}
