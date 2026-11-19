//! Vault operations of the Arena and of belief memory (sb F1): one startup read, queued writes.

use super::Vault;
use crate::domain::arena::ArenaId;
use crate::domain::beliefs::{BeliefBook, BeliefChange, BeliefSettings};
use crate::domain::ProfileId;
use crate::error::Result;
use crate::storage::db::ArenaRecord;
use std::collections::HashMap;

impl Vault {
    /// Every saved record of one kind, newest first (after every queued write).
    pub fn load_arena_records<T: ArenaRecord>(&self) -> Result<Vec<T>> {
        self.read(|db| db.load_arena_records())
    }

    /// Queues inserting or replacing `record`.
    pub fn save_arena_record<T: ArenaRecord>(&self, record: T) -> Result<()> {
        self.write(move |db| db.save_arena_record(&record))
    }

    /// Queues deleting the record `id` of kind `T`.
    pub fn delete_arena_record<T: ArenaRecord>(&self, id: &ArenaId) -> Result<()> {
        let id = id.clone();
        self.write(move |db| db.delete_arena_record::<T>(&id))
    }

    pub fn load_belief_book(&self) -> Result<BeliefBook> {
        self.read(|db| db.load_belief_book())
    }

    /// Queues writing `change` (one transaction).
    pub fn save_belief_change(&self, change: BeliefChange) -> Result<()> {
        self.write(move |db| db.save_belief_change(&change))
    }

    pub fn load_belief_settings(&self) -> Result<HashMap<ProfileId, BeliefSettings>> {
        self.read(|db| db.load_belief_settings())
    }

    pub fn save_belief_settings(&self, agent: &ProfileId, settings: BeliefSettings) -> Result<()> {
        let agent = agent.clone();
        self.write(move |db| db.save_belief_settings(&agent, &settings))
    }
}
