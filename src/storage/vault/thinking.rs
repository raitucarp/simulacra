//! Vault operations of the thinking tools (sb E1): one startup read, queued upserts.

use super::Vault;
use crate::domain::thinking::argument::ArgumentMap;
use crate::domain::thinking::credence::Credence;
use crate::domain::thinking::tensions::TensionRecord;
use crate::domain::PostId;
use crate::error::Result;

/// Everything the thinking tools keep, as loaded at startup.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThinkingRows {
    pub maps: Vec<ArgumentMap>,
    pub credences: Vec<Credence>,
    pub tensions: Vec<TensionRecord>,
}

impl Vault {
    /// Reads every map, credence and remembered pair (after every queued write).
    pub fn load_thinking(&self) -> Result<ThinkingRows> {
        self.read(|db| Ok(ThinkingRows { maps: db.load_argument_maps()?, credences: db.load_credences()?, tensions: db.load_tension_records()? }))
    }

    /// Queues saving `map` (an empty map forgets the thread's row).
    pub fn save_argument_map(&self, map: ArgumentMap, now: i64) -> Result<()> {
        self.write(move |db| db.save_argument_map(&map, now))
    }

    pub fn save_credence(&self, credence: Credence) -> Result<()> {
        self.write(move |db| db.save_credence(&credence))
    }

    pub fn delete_credence(&self, post_id: &PostId) -> Result<()> {
        let post_id = post_id.clone();
        self.write(move |db| db.delete_credence(&post_id))
    }

    /// Queues saving every record in `records` (verdicts of a batch, or one status change).
    pub fn save_tension_records(&self, records: Vec<TensionRecord>, now: i64) -> Result<()> {
        self.write(move |db| records.iter().try_for_each(|record| db.save_tension_record(record, now)))
    }
}
