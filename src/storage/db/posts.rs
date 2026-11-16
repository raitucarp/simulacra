//! Post persistence: full upsert and timeline loading.

use super::{collect_rows, db_err, parse_timestamp, sql_bool, Database};
use crate::domain::{NetworkId, Post, PostId, Profile};
use crate::error::Result;
use rusqlite::{params, Row};

/// Placeholder author used when a stored author snapshot cannot be decoded.
const UNKNOWN_AUTHOR_ID: &str = "unknown";
const UNKNOWN_AUTHOR_NAME: &str = "Author";
const UNKNOWN_AUTHOR_HANDLE: &str = "@unknown";

const UPSERT_POST: &str = r#"
                INSERT INTO posts (
                    id, network_id, author_id, author_json, content, created_at,
                    likes, is_liked, replies_count, mentions_json, linked_posts_json,
                    reply_to_id, quote_id, repost_id, link_url, image_path,
                    reposts_count, is_reposted
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)
                ON CONFLICT(id) DO UPDATE SET
                    network_id = excluded.network_id,
                    author_id = excluded.author_id,
                    author_json = excluded.author_json,
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
                    is_reposted = excluded.is_reposted;
                "#;

const SELECT_POSTS: &str = r#"
                SELECT
                    id, network_id, author_json, content, created_at,
                    likes, is_liked, replies_count, mentions_json, linked_posts_json,
                    reply_to_id, quote_id, repost_id, link_url, image_path,
                    reposts_count, is_reposted
                FROM posts
                ORDER BY created_at DESC
                "#;

impl Database {
    /// Saves or updates a post in the database with full attachments and relation metadata.
    pub fn save_post(&self, post: &Post) -> Result<()> {
        let author_json = serde_json::to_string(&post.author).map_err(db_err)?;
        let mentions_json = serde_json::to_string(&post.mentions).map_err(db_err)?;
        let linked_posts_json = serde_json::to_string(&post.linked_posts).map_err(db_err)?;
        let created_at_str = post.created_at.to_rfc3339();

        self.conn
            .execute(
                UPSERT_POST,
                params![
                    post.id.as_str(),
                    post.network_id.as_str(),
                    post.author.id.as_str(),
                    author_json,
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

/// Maps a `posts` row into a [`Post`], tolerating malformed JSON columns.
fn post_from_row(row: &Row<'_>) -> rusqlite::Result<Post> {
    let id: String = row.get(0)?;
    let network_id: String = row.get(1)?;
    let author_json: String = row.get(2)?;
    let content: String = row.get(3)?;
    let created_at_str: String = row.get(4)?;
    let likes: i64 = row.get(5)?;
    let is_liked: i64 = row.get(6)?;
    let replies_count: i64 = row.get(7)?;
    let mentions_json: String = row.get(8)?;
    let linked_posts_json: String = row.get(9)?;
    let reply_to_id: Option<String> = row.get(10)?;
    let quote_id: Option<String> = row.get(11)?;
    let repost_id: Option<String> = row.get(12)?;
    let link_url: Option<String> = row.get(13)?;
    let image_path: Option<String> = row.get(14)?;
    let reposts_count: i64 = row.get(15)?;
    let is_reposted: i64 = row.get(16)?;

    Ok(Post {
        id: PostId::new(id),
        network_id: NetworkId::new(network_id),
        author: decode_author(&author_json),
        content,
        created_at: parse_timestamp(&created_at_str),
        likes: likes as usize,
        is_liked: is_liked != 0,
        replies_count: replies_count as usize,
        mentions: serde_json::from_str(&mentions_json).unwrap_or_default(),
        linked_posts: serde_json::from_str(&linked_posts_json).unwrap_or_default(),
        reply_to_id: reply_to_id.map(PostId::new),
        quote_id: quote_id.map(PostId::new),
        repost_id: repost_id.map(PostId::new),
        link_url,
        image_path,
        reposts_count: reposts_count as usize,
        is_reposted: is_reposted != 0,
    })
}

/// Decodes an author snapshot, falling back to a placeholder primary profile.
fn decode_author(author_json: &str) -> Profile {
    serde_json::from_str(author_json).unwrap_or_else(|_| {
        Profile::new_primary(UNKNOWN_AUTHOR_ID, UNKNOWN_AUTHOR_NAME, UNKNOWN_AUTHOR_HANDLE)
    })
}
