//! Version history rows (sb F2) and the one write that edits a post's text.

use super::{collect_rows, db_err, parse_timestamp, Database};
use crate::domain::data::history::{Revision, RevisionId, RevisionSubject, SemVer};
use crate::domain::Post;
use crate::error::Result;
use rusqlite::{params, Row};

const INSERT_REVISION: &str = "INSERT OR REPLACE INTO revisions (id, subject_kind, subject_id, saved_at, title, content, record_json, parent_id, version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";
const SELECT_REVISIONS: &str = "SELECT id, subject_kind, subject_id, saved_at, title, content, record_json, parent_id, version FROM revisions WHERE subject_kind = ?1 AND subject_id = ?2 ORDER BY saved_at DESC, id DESC";
const COUNT_REVISIONS: &str = "SELECT subject_id, COUNT(*) FROM revisions WHERE subject_kind = ?1 GROUP BY subject_id";
const UPDATE_POST_BODY: &str = "UPDATE posts SET content = ?1, mentions_json = ?2, linked_posts_json = ?3 WHERE id = ?4";

fn revision_from_row(row: &Row<'_>) -> rusqlite::Result<Option<Revision>> {
    let (kind, id): (String, String) = (row.get(1)?, row.get(2)?);
    let Some(subject) = RevisionSubject::from_parts(&kind, &id) else { return Ok(None) };
    Ok(Some(Revision {
        id: RevisionId::new(row.get::<_, String>(0)?),
        subject,
        saved_at: parse_timestamp(&row.get::<_, String>(3)?),
        title: row.get(4)?,
        content: row.get(5)?,
        record_json: row.get(6)?,
        parent_id: row.get::<_, Option<String>>(7)?.map(RevisionId::new),
        version: SemVer::parse(&row.get::<_, String>(8)?),
    }))
}

impl Database {
    /// Stores one earlier version, numbered as the next of its subject's chain unless it
    /// already has a version (see [`SemVer`]).
    pub fn save_revision(&self, revision: &Revision) -> Result<()> {
        let revision = self.chained(revision)?;
        let (subject, parent) = (&revision.subject, revision.parent_id.as_ref().map(RevisionId::as_str));
        let version = revision.version.map(|v| v.to_string()).unwrap_or_default();
        let values = params![revision.id.as_str(), subject.kind(), subject.id(), crate::time::rfc3339(revision.saved_at), revision.title, revision.content, revision.record_json, parent, version];
        self.conn.execute(INSERT_REVISION, values).map_err(db_err)?;
        Ok(())
    }

    /// Every earlier version of `subject`, newest first.
    pub fn revisions_of(&self, subject: &RevisionSubject) -> Result<Vec<Revision>> {
        let mut stmt = self.conn.prepare(SELECT_REVISIONS).map_err(db_err)?;
        let rows = stmt.query_map(params![subject.kind(), subject.id()], revision_from_row).map_err(db_err)?;
        Ok(collect_rows(rows)?.into_iter().flatten().collect())
    }

    /// How many earlier versions each subject of `kind` has, by subject id.
    pub fn revision_counts(&self, kind: &str) -> Result<Vec<(String, usize)>> {
        let mut stmt = self.conn.prepare(COUNT_REVISIONS).map_err(db_err)?;
        let rows = stmt.query_map([kind], |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)? as usize))).map_err(db_err)?;
        collect_rows(rows)
    }

    /// Stores the edited body of `post` (text, mentions, links) and `revision` (its previous
    /// text), together.
    pub fn edit_post_content(&self, post: &Post, revision: &Revision) -> Result<()> {
        let mentions = serde_json::to_string(&post.mentions).map_err(db_err)?;
        let linked = serde_json::to_string(&post.linked_posts).map_err(db_err)?;
        let transaction = self.conn.unchecked_transaction().map_err(db_err)?;
        self.save_revision(revision)?;
        self.conn.execute(UPDATE_POST_BODY, params![post.content, mentions, linked, post.id.as_str()]).map_err(db_err)?;
        transaction.commit().map_err(db_err)
    }
}
