//! Vault operations of the embedding index (sb B3): queued writes, one startup read.

use super::Vault;
use crate::domain::embedding::{DailyUsage, EmbeddingOwner, StoredChunk, VectorIndex};
use crate::error::Result;
use crate::storage::SettingKey;

impl Vault {
    /// Reads every stored vector into an index (after every queued write).
    pub fn load_vector_index(&self) -> Result<VectorIndex> {
        self.read(|db| db.load_embeddings().map(VectorIndex::from_rows))
    }

    /// Queues an upsert of finished chunks.
    pub fn save_embeddings(&self, rows: Vec<(EmbeddingOwner, StoredChunk)>) -> Result<()> {
        self.write(move |db| db.save_embeddings(&rows))
    }

    /// Queues removing `owner`'s chunks from index `keep` on.
    pub fn trim_embeddings(&self, owner: &EmbeddingOwner, keep: usize) -> Result<()> {
        let owner = owner.clone();
        self.write(move |db| db.trim_embeddings(&owner, keep))
    }

    /// Today's embedding spend as last saved.
    pub fn load_embedding_usage(&self) -> DailyUsage {
        self.get_setting(SettingKey::EmbeddingUsage).ok().flatten().map(|raw| DailyUsage::from_json(&raw)).unwrap_or_default()
    }

    /// Queues saving the embedding spend.
    pub fn save_embedding_usage(&self, usage: &DailyUsage) -> Result<()> {
        self.set_setting(SettingKey::EmbeddingUsage, &usage.to_json())
    }
}
