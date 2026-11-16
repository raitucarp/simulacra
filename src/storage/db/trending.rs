//! Trending topic persistence.

use super::{collect_rows, db_err, Database};
use crate::domain::TrendingTopic;
use crate::error::Result;
use rusqlite::{params, Row};

const SELECT_TRENDING: &str =
    "SELECT tag, post_count, category FROM trending ORDER BY post_count DESC";

const UPSERT_TRENDING: &str = r#"
                INSERT INTO trending (tag, post_count, category)
                VALUES (?1, ?2, ?3)
                ON CONFLICT(tag) DO UPDATE SET
                    post_count = excluded.post_count,
                    category = excluded.category;
                "#;

impl Database {
    /// Loads trending topics.
    pub fn load_trending(&self) -> Result<Vec<TrendingTopic>> {
        let mut stmt = self.conn.prepare(SELECT_TRENDING).map_err(db_err)?;
        let rows = stmt.query_map([], trending_from_row).map_err(db_err)?;
        collect_rows(rows)
    }

    /// Saves a trending topic.
    pub fn save_trending(&self, topic: &TrendingTopic) -> Result<()> {
        self.conn
            .execute(
                UPSERT_TRENDING,
                params![topic.tag, topic.post_count as i64, topic.category],
            )
            .map_err(db_err)?;
        Ok(())
    }
}

/// Maps a `trending` row into a [`TrendingTopic`].
fn trending_from_row(row: &Row<'_>) -> rusqlite::Result<TrendingTopic> {
    let post_count: i64 = row.get(1)?;
    Ok(TrendingTopic {
        tag: row.get(0)?,
        post_count: post_count as usize,
        category: row.get(2)?,
    })
}
