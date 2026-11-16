//! Post deletion: a thought together with its content-less instant reposts.

use super::{db_err, Database};
use crate::domain::PostId;
use crate::error::Result;
use rusqlite::params;

/// Removes a post and every empty-content repost that points at it.
const DELETE_POST_WITH_REPOSTS: &str =
    "DELETE FROM posts WHERE id = ?1 OR (repost_id = ?1 AND content = '')";

impl Database {
    /// Deletes `post_id` and its instant reposts; replies and quotes of it are kept.
    pub fn delete_post_with_reposts(&self, post_id: &PostId) -> Result<()> {
        self.conn
            .execute(DELETE_POST_WITH_REPOSTS, params![post_id.as_str()])
            .map_err(db_err)?;
        Ok(())
    }
}
