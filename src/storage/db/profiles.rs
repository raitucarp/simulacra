//! Author profile persistence (human personas and autonomous network agents).

use super::{collect_rows, db_err, Database};
use crate::domain::{AgentConfig, NetworkId, Profile, ProfileId, ProfileKind, ProfileRole};
use crate::error::Result;
use rusqlite::{params, Row};

/// Description written by older releases for every agent; replaced by the persona type on load.
const LEGACY_AGENT_DESCRIPTION: &str = "Independent Thinker & Network Member";

/// Fallback JSON for profiles persisted before configs existed.
const EMPTY_CONFIG_JSON: &str = "{}";

const UPSERT_PROFILE: &str = r#"
                INSERT INTO profiles (id, name, handle, avatar_initial, role_json, network_id, avatar_path, config_json, kind)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                ON CONFLICT(id) DO UPDATE SET
                    name = excluded.name,
                    handle = excluded.handle,
                    avatar_initial = excluded.avatar_initial,
                    role_json = excluded.role_json,
                    network_id = excluded.network_id,
                    avatar_path = excluded.avatar_path,
                    config_json = excluded.config_json,
                    kind = excluded.kind;
                "#;

const DELETE_PROFILE: &str = "DELETE FROM profiles WHERE id = ?1";

const UPDATE_AUTHOR_SNAPSHOTS: &str = "UPDATE posts SET author_json = ?1 WHERE author_id = ?2";

const SELECT_PROFILES: &str =
    "SELECT id, name, handle, avatar_initial, role_json, network_id, avatar_path, config_json, kind FROM profiles";

impl Database {
    /// Saves an author profile (human persona or autonomous network agent) with full configuration.
    pub fn save_profile(&self, profile: &Profile) -> Result<()> {
        let role_json = serde_json::to_string(&profile.role).map_err(db_err)?;
        let config_json = serde_json::to_string(&profile.config).map_err(db_err)?;

        self.conn
            .execute(
                UPSERT_PROFILE,
                params![
                    profile.id.as_str(),
                    profile.name,
                    profile.handle,
                    profile.avatar_initial,
                    role_json,
                    profile.network_id.as_ref().map(|n| n.as_str()),
                    profile.avatar_path.as_deref(),
                    config_json,
                    profile.kind.key(),
                ],
            )
            .map_err(db_err)?;
        Ok(())
    }

    /// Deletes a profile from the database by ID.
    pub fn delete_profile(&self, profile_id: &ProfileId) -> Result<()> {
        self.conn
            .execute(DELETE_PROFILE, params![profile_id.as_str()])
            .map_err(db_err)?;
        Ok(())
    }

    /// Archives `profile` as the `author_json` snapshot of its posts, so they keep a
    /// displayable author after the profile itself is deleted.
    pub fn archive_post_author(&self, profile: &Profile) -> Result<()> {
        let author_json = serde_json::to_string(profile).map_err(db_err)?;
        self.conn
            .execute(UPDATE_AUTHOR_SNAPSHOTS, params![author_json, profile.id.as_str()])
            .map_err(db_err)?;
        Ok(())
    }

    /// Loads all author profiles with their network bindings, avatar paths, and psychological configs.
    pub fn load_profiles(&self) -> Result<Vec<Profile>> {
        let mut stmt = self.conn.prepare(SELECT_PROFILES).map_err(db_err)?;
        let rows = stmt.query_map([], profile_from_row).map_err(db_err)?;
        collect_rows(rows)
    }
}

/// Maps a `profiles` row into a [`Profile`], tolerating legacy/missing columns.
fn profile_from_row(row: &Row<'_>) -> rusqlite::Result<Profile> {
    let id: String = row.get(0)?;
    let name: String = row.get(1)?;
    let handle: String = row.get(2)?;
    let avatar_initial: String = row.get(3)?;
    let role_json: String = row.get(4)?;
    let network_id: Option<String> = row.get(5).unwrap_or(None);
    let avatar_path: Option<String> = row.get(6).unwrap_or(None);
    let config_json: String = row
        .get::<_, Option<String>>(7)
        .unwrap_or(None)
        .unwrap_or_else(|| EMPTY_CONFIG_JSON.to_string());

    let kind: Option<String> = row.get(8).unwrap_or(None);
    let role: ProfileRole = serde_json::from_str(&role_json).unwrap_or(ProfileRole::UserPrimary);
    let config = decode_config(&config_json, &role);

    Ok(Profile {
        id: ProfileId::new(id),
        name,
        handle,
        avatar_initial,
        role,
        network_id: network_id.map(NetworkId::new),
        avatar_path,
        config,
        kind: kind.as_deref().map(ProfileKind::from_key).unwrap_or_default(),
    })
}

/// Decodes a stored config, upgrading the legacy generic agent description to the persona type.
fn decode_config(config_json: &str, role: &ProfileRole) -> AgentConfig {
    let mut config: AgentConfig = serde_json::from_str(config_json).unwrap_or_default();
    if let ProfileRole::Agent { ref persona_type } = role {
        if config.description == LEGACY_AGENT_DESCRIPTION && !persona_type.trim().is_empty() {
            config.description = persona_type.clone();
        }
    }
    config
}
