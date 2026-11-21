//! Post deletion: a thought together with its content-less instant reposts, and (Y7) what is
//! kept about a deleted post.

use super::{db_err, Database};
use crate::domain::data::history::POST_SUBJECT;
use crate::domain::embedding::OwnerKind;
use crate::domain::PostId;
use crate::error::Result;
use rusqlite::params;

/// Removes a post and every empty-content repost that points at it.
const DELETE_POST_WITH_REPOSTS: &str =
    "DELETE FROM posts WHERE id = ?1 OR (repost_id = ?1 AND content = '')";
/// The earlier versions of a post.
const DELETE_REVISIONS_OF: &str = "DELETE FROM revisions WHERE subject_kind = ?1 AND subject_id = ?2";
/// The meaning index vectors of a post.
const DELETE_EMBEDDINGS_OF: &str = "DELETE FROM embeddings WHERE owner_kind = ?1 AND owner_id = ?2";

impl Database {
    /// Deletes `post_id` and its instant reposts; replies and quotes of it are kept.
    pub fn delete_post_with_reposts(&self, post_id: &PostId) -> Result<()> {
        self.conn
            .execute(DELETE_POST_WITH_REPOSTS, params![post_id.as_str()])
            .map_err(db_err)?;
        Ok(())
    }

    /// Deletes what is kept about the deleted post `post_id`: its earlier versions and its
    /// meaning index vectors. Reactions do not exist yet; once they do, their rows are
    /// deleted here as well.
    pub fn purge_post_traces(&self, post_id: &PostId) -> Result<()> {
        self.conn.execute(DELETE_REVISIONS_OF, params![POST_SUBJECT, post_id.as_str()]).map_err(db_err)?;
        self.conn.execute(DELETE_EMBEDDINGS_OF, params![OwnerKind::Post.key(), post_id.as_str()]).map_err(db_err)?;
        Ok(())
    }
}
