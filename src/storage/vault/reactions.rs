//! Vault operations of emoji reactions and claps (R1): one startup read, queued writes.

use super::Vault;
use crate::domain::reaction::Reaction;
use crate::error::Result;

impl Vault {
    /// Reads every stored reaction (after every queued write).
    pub fn load_reactions(&self) -> Result<Vec<Reaction>> {
        self.read(|db| db.load_reactions())
    }

    /// Queues inserting or replacing `reaction` (deleting it when its count is zero).
    pub fn save_reaction(&self, reaction: &Reaction) -> Result<()> {
        let reaction = reaction.clone();
        self.write(move |db| db.save_reaction(&reaction))
    }
}
