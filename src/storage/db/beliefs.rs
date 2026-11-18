//! Rows of agent belief memory (sb F1): the beliefs, the append-only revision log and each
//! agent's update settings. A [`BeliefChange`] is written in one transaction, so a belief and
//! the revision that explains it land together.

use super::{collect_rows, db_err, Database};
use crate::domain::beliefs::{Belief, BeliefBook, BeliefChange, BeliefSettings, Revision};
use crate::domain::ProfileId;
use crate::error::Result;
use rusqlite::params;
use std::collections::HashMap;

const UPSERT_BELIEF: &str = "INSERT INTO agent_beliefs (id, agent_id, updated_at, record_json) VALUES (?1, ?2, ?3, ?4)
     ON CONFLICT(id) DO UPDATE SET agent_id = excluded.agent_id, updated_at = excluded.updated_at, record_json = excluded.record_json";
const DELETE_BELIEF: &str = "DELETE FROM agent_beliefs WHERE id = ?1";
const INSERT_REVISION: &str = "INSERT INTO belief_revisions (belief_id, agent_id, at, record_json) VALUES (?1, ?2, ?3, ?4)";
const UPSERT_SETTINGS: &str = "INSERT INTO belief_settings (agent_id, settings_json) VALUES (?1, ?2)
     ON CONFLICT(agent_id) DO UPDATE SET settings_json = excluded.settings_json";

impl Database {
    fn load_json_rows<T: serde::de::DeserializeOwned>(&self, sql: &str) -> Result<Vec<T>> {
        let mut stmt = self.conn.prepare(sql).map_err(db_err)?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0)).map_err(db_err)?;
        Ok(collect_rows(rows)?.into_iter().filter_map(|raw| serde_json::from_str(&raw).ok()).collect())
    }

    /// Every belief and the whole revision log (oldest first).
    pub fn load_belief_book(&self) -> Result<BeliefBook> {
        let beliefs: Vec<Belief> = self.load_json_rows("SELECT record_json FROM agent_beliefs ORDER BY updated_at, id")?;
        let revisions: Vec<Revision> = self.load_json_rows("SELECT record_json FROM belief_revisions ORDER BY seq")?;
        Ok(BeliefBook { beliefs, revisions })
    }

    /// Writes `change` in one transaction.
    pub fn save_belief_change(&self, change: &BeliefChange) -> Result<()> {
        let tx = self.conn.unchecked_transaction().map_err(db_err)?;
        for belief in &change.upserted {
            let json = serde_json::to_string(belief).map_err(db_err)?;
            tx.execute(UPSERT_BELIEF, params![belief.id.as_str(), belief.agent.as_str(), belief.updated_at.to_rfc3339(), json]).map_err(db_err)?;
        }
        for id in &change.removed {
            tx.execute(DELETE_BELIEF, params![id.as_str()]).map_err(db_err)?;
        }
        for revision in &change.revisions {
            let json = serde_json::to_string(revision).map_err(db_err)?;
            tx.execute(INSERT_REVISION, params![revision.belief.as_str(), revision.agent.as_str(), revision.at.to_rfc3339(), json]).map_err(db_err)?;
        }
        tx.commit().map_err(db_err)
    }

    /// Every agent's update settings.
    pub fn load_belief_settings(&self) -> Result<HashMap<ProfileId, BeliefSettings>> {
        let mut stmt = self.conn.prepare("SELECT agent_id, settings_json FROM belief_settings").map_err(db_err)?;
        let rows = stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))).map_err(db_err)?;
        let parsed = collect_rows(rows)?.into_iter().filter_map(|(agent, raw)| Some((ProfileId::new(agent), serde_json::from_str(&raw).ok()?)));
        Ok(parsed.collect())
    }

    pub fn save_belief_settings(&self, agent: &ProfileId, settings: &BeliefSettings) -> Result<()> {
        let json = serde_json::to_string(settings).map_err(db_err)?;
        self.conn.execute(UPSERT_SETTINGS, params![agent.as_str(), json]).map_err(db_err)?;
        Ok(())
    }
}
