//! Vault operations of the review deck (sb C1): one startup read, queued writes.

use super::Vault;
use crate::domain::review::{Card, CardId, Deck, ReviewEntry};
use crate::error::Result;

impl Vault {
    /// Reads every card and logged answer (after every queued write).
    pub fn load_review_deck(&self) -> Result<Deck> {
        self.read(|db| db.load_review_deck())
    }

    /// Queues inserting or replacing `cards`.
    pub fn save_review_cards(&self, cards: Vec<Card>) -> Result<()> {
        self.write(move |db| db.save_review_cards(&cards))
    }

    /// Queues deleting card `id`.
    pub fn delete_review_card(&self, id: &CardId) -> Result<()> {
        let id = id.clone();
        self.write(move |db| db.delete_review_card(&id))
    }

    /// Queues appending `entry` to the review log.
    pub fn append_review_entry(&self, entry: ReviewEntry) -> Result<()> {
        self.write(move |db| db.append_review_entry(&entry))
    }
}
