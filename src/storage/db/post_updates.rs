//! Targeted post counter updates and repost removal.

use super::{db_err, sql_bool, Database};
use crate::domain::{PostId, ProfileId};
use crate::error::Result;
use rusqlite::params;

const UPDATE_LIKE: &str = "UPDATE posts SET likes = ?1, is_liked = ?2 WHERE id = ?3";
const UPDATE_REPOST: &str = "UPDATE posts SET reposts_count = ?1, is_reposted = ?2 WHERE id = ?3";
const DELETE_REPOST: &str = "DELETE FROM posts WHERE repost_id = ?1 AND author_id = ?2";
const UPDATE_REPLIES_COUNT: &str = "UPDATE posts SET replies_count = ?1 WHERE id = ?2";

impl Database {
    /// Updates like count and flag for a post.
    pub fn update_post_like(&self, post_id: &PostId, likes: usize, is_liked: bool) -> Result<()> {
        self.conn
            .execute(
                UPDATE_LIKE,
                params![likes as i64, sql_bool(is_liked), post_id.as_str()],
            )
            .map_err(db_err)?;
        Ok(())
    }

    /// Updates repost count and flag for a post.
    pub fn update_post_repost(&self, post_id: &PostId, reposts_count: usize, is_reposted: bool) -> Result<()> {
        self.conn
            .execute(
                UPDATE_REPOST,
                params![reposts_count as i64, sql_bool(is_reposted), post_id.as_str()],
            )
            .map_err(db_err)?;
        Ok(())
    }

    /// Deletes a repost referencing target post ID authored by a specific profile.
    pub fn delete_repost_by_target_and_author(
        &self,
        target_post_id: &PostId,
        author_id: &ProfileId,
    ) -> Result<()> {
        self.conn
            .execute(DELETE_REPOST, params![target_post_id.as_str(), author_id.as_str()])
            .map_err(db_err)?;
        Ok(())
    }

    /// Updates reply count for a post.
    pub fn update_post_replies_count(&self, post_id: &PostId, replies_count: usize) -> Result<()> {
        self.conn
            .execute(UPDATE_REPLIES_COUNT, params![replies_count as i64, post_id.as_str()])
            .map_err(db_err)?;
        Ok(())
    }
}
