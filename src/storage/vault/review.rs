//! Vault operations of the review deck (sb C1, items since sb C1b): one startup read,
//! queued writes.

use super::Vault;
use crate::domain::review::{Deck, ItemId, ReviewEntry, ReviewItem};
use crate::error::Result;

impl Vault {
    /// Reads every item and logged answer (after every queued write).
    pub fn load_review_deck(&self) -> Result<Deck> {
        self.read(|db| db.load_review_deck())
    }

    /// Queues inserting or replacing `items`.
    pub fn save_review_items(&self, items: Vec<ReviewItem>) -> Result<()> {
        self.write(move |db| db.save_review_items(&items))
    }

    /// Queues deleting item `id` (its log stays).
    pub fn delete_review_item(&self, id: &ItemId) -> Result<()> {
        let id = id.clone();
        self.write(move |db| db.delete_review_item(&id))
    }

    /// Queues appending `entry` to the review log.
    pub fn append_review_entry(&self, entry: ReviewEntry) -> Result<()> {
        self.write(move |db| db.append_review_entry(&entry))
    }
}
