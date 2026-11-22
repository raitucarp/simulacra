//! Full-text index of post text (Q1): `posts_fts` (FTS5), one row per post with text.
//!
//! Triggers on `posts` keep it in step with every write (saving, editing and deleting a post,
//! whichever path does it); instant reposts, which have no text, are never indexed. The v7
//! migration indexes the posts stored before the index existed. Rows are found by
//! `post_id`, never by rowid, so copies made with `VACUUM INTO` stay consistent. A locked
//! post's `content` is only the placeholder (X2), so its text never reaches the index.

use super::{collect_rows, db_err, Database};
use crate::domain::document::query::MIN_TERM_CHARS;
use crate::domain::post::PostStatus;
use crate::domain::PostId;
use crate::error::Result;
use rusqlite::params;

/// v7: every post with text has a row in `posts_fts`.
pub(super) const POSTS_FTS_VERSION: i64 = 7;

/// The index and the triggers keeping it in sync, created idempotently.
const CREATE_POSTS_FTS: &str = r#"
    CREATE VIRTUAL TABLE IF NOT EXISTS posts_fts USING fts5(
        post_id UNINDEXED, content,
        tokenize = 'unicode61 remove_diacritics 2'
    );
    CREATE TRIGGER IF NOT EXISTS posts_fts_after_insert AFTER INSERT ON posts
    WHEN new.content != '' BEGIN
        INSERT INTO posts_fts (post_id, content) VALUES (new.id, new.content);
    END;
    CREATE TRIGGER IF NOT EXISTS posts_fts_after_edit AFTER UPDATE OF content ON posts
    WHEN old.content IS NOT new.content BEGIN
        DELETE FROM posts_fts WHERE post_id = old.id;
        INSERT INTO posts_fts (post_id, content) SELECT new.id, new.content WHERE new.content != '';
    END;
    CREATE TRIGGER IF NOT EXISTS posts_fts_after_delete AFTER DELETE ON posts
    WHEN old.content != '' BEGIN
        DELETE FROM posts_fts WHERE post_id = old.id;
    END;
    -- Q7: leaving scheduled posts out reads only them, not every post.
    CREATE INDEX IF NOT EXISTS idx_posts_status ON posts(status);
"#;

/// Rebuilds the index from `posts`.
const REINDEX_POSTS: &str = r#"
    DELETE FROM posts_fts;
    INSERT INTO posts_fts (post_id, content) SELECT id, content FROM posts WHERE content != '';
"#;

/// Matching post ids and their bm25 score (lower is better), best first, ties by id. Posts
/// with status `?3` (scheduled, not yet published) are left out.
const SCORE_POSTS: &str = "SELECT post_id, bm25(posts_fts) FROM posts_fts WHERE posts_fts MATCH ?1 \
    AND post_id NOT IN (SELECT id FROM posts WHERE status = ?3) ORDER BY bm25(posts_fts), post_id LIMIT ?2";

/// The searchable terms of `text`: runs of letters and digits, lower-cased.
pub fn search_terms(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !c.is_alphanumeric()).filter(|term| !term.is_empty()).map(str::to_lowercase)
}

/// The searchable words of `words` (terms at least [`MIN_TERM_CHARS`] long), in order.
pub fn word_terms(words: &[String]) -> Vec<String> {
    words.iter().flat_map(|word| search_terms(word).filter(|term| term.chars().count() >= MIN_TERM_CHARS).collect::<Vec<_>>()).collect()
}

/// The FTS5 expression for `text`: every term (letters and digits, at least
/// [`MIN_TERM_CHARS`] long) required, each as a quoted prefix, so user text can never inject
/// FTS syntax. `None` without terms.
pub fn post_match_expression(text: &str) -> Option<String> {
    let terms: Vec<String> = word_terms(&[text.to_string()]).into_iter().map(|term| format!("\"{term}\"*")).collect();
    (!terms.is_empty()).then(|| terms.join(" "))
}

/// The FTS5 expression for a query (Q3): every word required, whole except the last, which
/// is a prefix (the one still being typed; a prefix of every word made common prefixes cost
/// most of a search at 100 000 posts, Q7), and every phrase as consecutive terms. All quoted,
/// so nothing typed is read as FTS syntax. `None` without terms.
pub fn query_match_expression(words: &[String], phrases: &[String]) -> Option<String> {
    let terms = word_terms(words);
    let last = terms.len().saturating_sub(1);
    let words = terms.iter().enumerate().map(|(n, term)| if n == last { format!("\"{term}\"*") } else { format!("\"{term}\"") });
    let phrases = phrases.iter().filter_map(|phrase| {
        let terms: Vec<String> = search_terms(phrase).collect();
        (!terms.is_empty()).then(|| format!("\"{}\"", terms.join(" ")))
    });
    let parts: Vec<String> = words.chain(phrases).collect();
    (!parts.is_empty()).then(|| parts.join(" "))
}

impl Database {
    /// Creates `posts_fts` and its triggers when missing (after `posts` has every column).
    pub(super) fn create_posts_index(&self) -> Result<()> {
        self.conn.execute_batch(CREATE_POSTS_FTS).map_err(db_err)
    }

    /// v6 → v7: indexes every post stored before the index existed.
    pub(super) fn migrate_posts_index(&self) -> Result<()> {
        self.conn.execute_batch(REINDEX_POSTS).map_err(db_err)
    }

    /// Ids of at most `limit` published or archived posts whose text matches every term of
    /// `text`, best first (scheduled posts stay hidden until they are published).
    pub fn search_posts(&self, text: &str, limit: usize) -> Result<Vec<PostId>> {
        let Some(expression) = post_match_expression(text) else { return Ok(Vec::new()) };
        Ok(self.score_posts(&expression, limit)?.into_iter().map(|(id, _)| id).collect())
    }

    /// At most `limit` published or archived posts matching the FTS5 `expression` (built by
    /// [`query_match_expression`]), best first, with their bm25 score (lower is better).
    pub fn score_posts(&self, expression: &str, limit: usize) -> Result<Vec<(PostId, f64)>> {
        let mut stmt = self.conn.prepare_cached(SCORE_POSTS).map_err(db_err)?;
        let hidden = PostStatus::Scheduled.key();
        let rows = stmt
            .query_map(params![expression, limit as i64, hidden], |row| Ok((PostId::new(row.get::<_, String>(0)?), row.get::<_, f64>(1)?)))
            .map_err(db_err)?;
        collect_rows(rows)
    }
}
