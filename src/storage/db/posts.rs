//! Post persistence: full upsert and timeline loading.

use super::post_row::{post_from_row, stored_moment, SELECT_POSTS};
use super::{collect_rows, db_err, sql_bool, Database};
use super::EMPTY_AUTHOR_SNAPSHOT;
use crate::domain::Post;
use crate::error::Result;
use rusqlite::params;

const UPSERT_POST: &str = r#"
                INSERT INTO posts (
                    id, network_id, author_id, author_json, content, created_at,
                    likes, is_liked, replies_count, mentions_json, linked_posts_json,
                    reply_to_id, quote_id, repost_id, link_url, image_path,
                    reposts_count, is_reposted, extras_json,
                    kind, status, publish_at, archive_at, delete_at, kind_json
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19,
                        ?20, ?21, ?22, ?23, ?24, ?25)
                ON CONFLICT(id) DO UPDATE SET
                    network_id = excluded.network_id,
                    author_id = excluded.author_id,
                    content = excluded.content,
                    created_at = excluded.created_at,
                    likes = excluded.likes,
                    is_liked = excluded.is_liked,
                    replies_count = excluded.replies_count,
                    mentions_json = excluded.mentions_json,
                    linked_posts_json = excluded.linked_posts_json,
                    reply_to_id = excluded.reply_to_id,
                    quote_id = excluded.quote_id,
                    repost_id = excluded.repost_id,
                    link_url = excluded.link_url,
                    image_path = excluded.image_path,
                    reposts_count = excluded.reposts_count,
                    is_reposted = excluded.is_reposted,
                    extras_json = excluded.extras_json,
                    kind = excluded.kind,
                    status = excluded.status,
                    publish_at = excluded.publish_at,
                    archive_at = excluded.archive_at,
                    delete_at = excluded.delete_at,
                    kind_json = excluded.kind_json;
                "#;

impl Database {
    /// Saves or updates a post in the database with full attachments and relation metadata.
    pub fn save_post(&self, post: &Post) -> Result<()> {
        let mentions_json = serde_json::to_string(&post.mentions).map_err(db_err)?;
        let linked_posts_json = serde_json::to_string(&post.linked_posts).map_err(db_err)?;
        let extras_json = serde_json::to_string(&post.extras).map_err(db_err)?;
        let created_at_str = crate::time::rfc3339(post.created_at);
        let life = &post.lifecycle;

        self.conn
            .execute(
                UPSERT_POST,
                params![
                    post.id.as_str(),
                    post.network_id.as_str(),
                    post.author_id.as_str(),
                    EMPTY_AUTHOR_SNAPSHOT,
                    post.content,
                    created_at_str,
                    post.likes as i64,
                    sql_bool(post.is_liked),
                    post.replies_count as i64,
                    mentions_json,
                    linked_posts_json,
                    post.reply_to_id.as_ref().map(|id| id.as_str()),
                    post.quote_id.as_ref().map(|id| id.as_str()),
                    post.repost_id.as_ref().map(|id| id.as_str()),
                    post.link_url.as_deref(),
                    post.image_path.as_deref(),
                    post.reposts_count as i64,
                    sql_bool(post.is_reposted),
                    extras_json,
                    post.kind.key(),
                    life.status.key(),
                    stored_moment(life.publish_at), stored_moment(life.archive_at), stored_moment(life.delete_at),
                    post.kind.payload_json(), // M1
                ],
            )
            .map_err(db_err)?;
        Ok(())
    }

    /// Loads all posts ordered by creation date descending.
    pub fn load_posts(&self) -> Result<Vec<Post>> {
        let mut stmt = self.conn.prepare(SELECT_POSTS).map_err(db_err)?;
        let rows = stmt.query_map([], post_from_row).map_err(db_err)?;
        collect_rows(rows)
    }
}
