//! Vault operations of the private Journal records (sb E3): one startup read, queued writes.

use super::Vault;
use crate::domain::journal::{JournalBook, RecordId};
use crate::error::Result;
use crate::storage::db::StoredRecord;

impl Vault {
    /// Reads every private record (after every queued write).
    pub fn load_journal_book(&self) -> Result<JournalBook> {
        self.read(|db| db.load_journal_book())
    }

    /// Queues inserting or replacing `record`.
    pub fn save_journal_record<T: StoredRecord>(&self, record: T) -> Result<()> {
        self.write(move |db| db.save_journal_record(&record))
    }

    /// Queues deleting the record `id` of kind `T`.
    pub fn delete_journal_record<T: StoredRecord>(&self, id: &RecordId) -> Result<()> {
        let id = id.clone();
        self.write(move |db| db.delete_journal_record::<T>(&id))
    }
}
