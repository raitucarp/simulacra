//! Reading `posts` rows back into [`Post`]s.

use super::parse_timestamp;
use crate::domain::post::{PostKind, PostLifecycle, PostStatus};
use crate::domain::{NetworkId, Post, PostId, ProfileId};
use jiff::Timestamp;
use rusqlite::Row;

/// Every post, newest first, with the columns [`post_from_row`] reads, in order.
pub(super) const SELECT_POSTS: &str = r#"
                SELECT
                    id, network_id, author_id, content, created_at,
                    likes, is_liked, replies_count, mentions_json, linked_posts_json,
                    reply_to_id, quote_id, repost_id, link_url, image_path,
                    reposts_count, is_reposted, extras_json,
                    kind, status, publish_at, archive_at, delete_at
                FROM posts
                ORDER BY created_at DESC
                "#;

/// Column of `kind`; the lifecycle columns follow it.
const KIND_COLUMN: usize = 18;

/// Maps a `posts` row into a [`Post`], tolerating malformed JSON columns.
pub(super) fn post_from_row(row: &Row<'_>) -> rusqlite::Result<Post> {
    let created_at_str: String = row.get(4)?;
    let (likes, is_liked, replies_count): (i64, i64, i64) = (row.get(5)?, row.get(6)?, row.get(7)?);
    let mentions_json: String = row.get(8)?;
    let linked_posts_json: String = row.get(9)?;
    let (reposts_count, is_reposted): (i64, i64) = (row.get(15)?, row.get(16)?);
    let extras_json: Option<String> = row.get(17)?;
    let id = |at: usize| row.get::<_, Option<String>>(at).map(|id| id.map(PostId::new));

    Ok(Post {
        id: PostId::new(row.get::<_, String>(0)?),
        network_id: NetworkId::new(row.get::<_, String>(1)?),
        author_id: ProfileId::new(row.get::<_, String>(2)?),
        content: row.get(3)?,
        created_at: parse_timestamp(&created_at_str),
        likes: likes as usize,
        is_liked: is_liked != 0,
        replies_count: replies_count as usize,
        mentions: serde_json::from_str(&mentions_json).unwrap_or_default(),
        linked_posts: serde_json::from_str(&linked_posts_json).unwrap_or_default(),
        reply_to_id: id(10)?,
        quote_id: id(11)?,
        repost_id: id(12)?,
        link_url: row.get(13)?,
        image_path: row.get(14)?,
        reposts_count: reposts_count as usize,
        is_reposted: is_reposted != 0,
        extras: extras_json.and_then(|json| serde_json::from_str(&json).ok()).unwrap_or_default(),
        kind: row.get::<_, Option<String>>(KIND_COLUMN)?.map_or_else(PostKind::default, |key| PostKind::from_stored(&key)),
        lifecycle: lifecycle_from_row(row, KIND_COLUMN + 1)?,
    })
}

/// The status and the three due moments stored from column `first` on (unknown statuses
/// read as published, malformed moments as none).
fn lifecycle_from_row(row: &Row<'_>, first: usize) -> rusqlite::Result<PostLifecycle> {
    let moment = |at: usize| row.get::<_, Option<String>>(at).map(|raw| raw.as_deref().and_then(crate::time::parse_timestamp));
    let status: Option<String> = row.get(first)?;
    Ok(PostLifecycle {
        status: status.map_or_else(PostStatus::default, |key| PostStatus::from_stored(&key)),
        publish_at: moment(first + 1)?,
        archive_at: moment(first + 2)?,
        delete_at: moment(first + 3)?,
    })
}

/// A due moment as stored (RFC 3339, or NULL).
pub(super) fn stored_moment(at: Option<Timestamp>) -> Option<String> {
    at.map(crate::time::rfc3339)
}
