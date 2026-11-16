//! Network domain persistence.

use super::{collect_rows, db_err, Database};
use crate::domain::{Network, NetworkId};
use crate::error::Result;
use rusqlite::{params, Row};

const COUNT_NETWORKS: &str = "SELECT COUNT(*) FROM networks";

const UPSERT_NETWORK: &str = r#"
                INSERT INTO networks (id, name, slug, description, doc_count, agent_count, color_accent)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                ON CONFLICT(id) DO UPDATE SET
                    name = excluded.name,
                    slug = excluded.slug,
                    description = excluded.description,
                    doc_count = excluded.doc_count,
                    agent_count = excluded.agent_count,
                    color_accent = excluded.color_accent;
                "#;

const SELECT_NETWORKS: &str =
    "SELECT id, name, slug, description, doc_count, agent_count, color_accent FROM networks";

impl Database {
    /// Checks if the database has any seeded networks.
    pub fn is_empty(&self) -> Result<bool> {
        let mut stmt = self.conn.prepare(COUNT_NETWORKS).map_err(db_err)?;
        let count: i64 = stmt.query_row([], |row| row.get(0)).map_err(db_err)?;
        Ok(count == 0)
    }

    /// Saves a network domain record.
    pub fn save_network(&self, net: &Network) -> Result<()> {
        self.conn
            .execute(
                UPSERT_NETWORK,
                params![
                    net.id.as_str(),
                    net.name,
                    net.slug,
                    net.description,
                    net.doc_count as i64,
                    net.agent_count as i64,
                    net.color_accent,
                ],
            )
            .map_err(db_err)?;
        Ok(())
    }

    /// Loads all network domains.
    pub fn load_networks(&self) -> Result<Vec<Network>> {
        let mut stmt = self.conn.prepare(SELECT_NETWORKS).map_err(db_err)?;
        let rows = stmt.query_map([], network_from_row).map_err(db_err)?;
        collect_rows(rows)
    }
}

/// Maps a `networks` row into a [`Network`].
fn network_from_row(row: &Row<'_>) -> rusqlite::Result<Network> {
    let id: String = row.get(0)?;
    let doc_count: i64 = row.get(4)?;
    let agent_count: i64 = row.get(5)?;
    Ok(Network {
        id: NetworkId::new(id),
        name: row.get(1)?,
        slug: row.get(2)?,
        description: row.get(3)?,
        doc_count: doc_count as usize,
        agent_count: agent_count as usize,
        color_accent: row.get(6)?,
    })
}
