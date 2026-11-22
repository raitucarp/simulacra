//! Many posts (imports) or revisions in one transaction, so a book lands in the vault all at
//! once or not at all.

use super::{db_err, Database};
use crate::domain::Post;
use crate::error::Result;

impl Database {
    /// Upserts every post inside one transaction; on any failure nothing is stored.
    pub fn save_posts(&self, posts: &[Post]) -> Result<()> {
        let transaction = self.conn.unchecked_transaction().map_err(db_err)?;
        for post in posts {
            self.save_post(post)?;
        }
        transaction.commit().map_err(db_err)
    }
}

impl Database {
    /// Stores every revision inside one transaction; on any failure nothing is stored.
    pub fn save_revisions(&self, revisions: &[crate::domain::data::history::Revision]) -> Result<()> {
        let transaction = self.conn.unchecked_transaction().map_err(db_err)?;
        for revision in revisions {
            self.save_revision(revision)?;
        }
        transaction.commit().map_err(db_err)
    }
}
