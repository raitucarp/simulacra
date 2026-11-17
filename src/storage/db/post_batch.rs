//! Many posts in one transaction (imports), so a book lands in the vault all at once or not
//! at all.

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
