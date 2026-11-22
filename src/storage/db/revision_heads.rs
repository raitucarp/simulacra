//! Version heads (V3): the v9 migration gives every stored version the time and bump of the
//! edit that made it, and every post edited before heads were stored the head one minor
//! version after its newest revision (see [`VersionHead::after`]).

use super::{collect_rows, db_err, Database};
use crate::domain::data::history::{step_bump, RevisionSubject, SemVer, VersionHead};
use crate::domain::PostId;
use crate::error::Result;
use rusqlite::params;

/// v9: every revision has its edit time and bump; every edited post has a head.
pub(super) const REVISION_HEADS_VERSION: i64 = 9;

/// A version was written when the one it was edited from was replaced; an original when its
/// post was written.
const BACKFILL_EDIT_TIMES: &str = r#"
    UPDATE revisions SET edited_at = (SELECT parent.saved_at FROM revisions AS parent WHERE parent.id = revisions.parent_id)
        WHERE edited_at IS NULL AND parent_id IS NOT NULL;
    UPDATE revisions SET edited_at = (SELECT created_at FROM posts WHERE posts.id = revisions.subject_id)
        WHERE edited_at IS NULL AND parent_id IS NULL AND subject_kind = 'post';
"#;
/// Versions edited from another without a stored bump, with both version numbers.
const SELECT_BUMPLESS: &str = "SELECT child.id, parent.version, child.version FROM revisions AS child \
    JOIN revisions AS parent ON parent.id = child.parent_id WHERE child.bump = ''";
const SET_BUMP: &str = "UPDATE revisions SET bump = ?1 WHERE id = ?2";
/// Posts with revisions but no stored head.
const SELECT_HEADLESS: &str = "SELECT id FROM posts WHERE json_valid(extras_json) AND json_type(extras_json, '$.head') IS NULL \
    AND id IN (SELECT subject_id FROM revisions WHERE subject_kind = 'post')";
const SET_HEAD: &str = "UPDATE posts SET extras_json = json_set(extras_json, '$.head', json(?1)) WHERE id = ?2";

impl Database {
    /// v8 → v9: edit times, bumps and heads of versions stored before heads.
    pub(super) fn migrate_revision_heads(&self) -> Result<()> {
        self.conn.execute_batch(BACKFILL_EDIT_TIMES).map_err(db_err)?;
        self.backfill_bumps()?;
        self.backfill_heads()
    }

    fn backfill_bumps(&self) -> Result<()> {
        let mut stmt = self.conn.prepare(SELECT_BUMPLESS).map_err(db_err)?;
        let rows = stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))).map_err(db_err)?;
        for (id, from, to) in collect_rows(rows)? {
            let bump = step_bump(SemVer::parse(&from).unwrap_or_default(), SemVer::parse(&to).unwrap_or_default());
            self.conn.execute(SET_BUMP, params![bump.key(), id]).map_err(db_err)?;
        }
        Ok(())
    }

    fn backfill_heads(&self) -> Result<()> {
        let mut stmt = self.conn.prepare(SELECT_HEADLESS).map_err(db_err)?;
        let ids = collect_rows(stmt.query_map([], |row| row.get::<_, String>(0)).map_err(db_err)?)?;
        for id in ids {
            let revisions = self.revisions_of(&RevisionSubject::Post(PostId::new(id.clone())))?;
            let Some(newest) = revisions.first() else { continue };
            let head = serde_json::to_string(&VersionHead::after(newest)).map_err(db_err)?;
            self.conn.execute(SET_HEAD, params![head, id]).map_err(db_err)?;
        }
        Ok(())
    }
}
