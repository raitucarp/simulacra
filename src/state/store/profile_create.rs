//! Creation of custom network agents and controllable human personas.

use super::AppState;
use crate::domain::{AgentConfig, Profile, ProfileId};
use crate::error::Result;
use uuid::Uuid;

const DEFAULT_AGENT_NAME: &str = "New Network Thinker";
const DEFAULT_AGENT_HANDLE_STEM: &str = "member";
const DEFAULT_AGENT_DESCRIPTION: &str = "Independent Thinker";
pub(crate) const AGENT_ID_PREFIX: &str = "agent_";

const DEFAULT_PERSONA_NAME: &str = "New Human Persona";
const DEFAULT_PERSONA_HANDLE_STEM: &str = "persona";
const DEFAULT_PERSONA_DESCRIPTION: &str = "Alter-Ego Persona";
pub(crate) const PERSONA_ID_PREFIX: &str = "user_";

const HANDLE_SIGIL: char = '@';
const HANDLE_SPACE: char = ' ';
const HANDLE_SPACE_REPLACEMENT: &str = "_";

/// Trimmed `value`, or `fallback` when it is blank.
fn or_fallback<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback
    } else {
        trimmed
    }
}

/// Normalizes a user-entered handle (or derives one from the name's first word) to `@lower_case`.
fn normalize_handle(handle: &str, clean_name: &str, stem: &str) -> String {
    let raw_handle = if handle.trim().is_empty() {
        clean_name
            .split_whitespace()
            .next()
            .unwrap_or(stem)
            .to_lowercase()
    } else {
        handle.trim().trim_start_matches(HANDLE_SIGIL).to_lowercase()
    };
    format!(
        "{HANDLE_SIGIL}{}",
        raw_handle.replace(HANDLE_SPACE, HANDLE_SPACE_REPLACEMENT)
    )
}

/// Default config carrying `description` as both description and occupation.
fn described_config(description: &str) -> AgentConfig {
    let mut cfg = AgentConfig::default();
    cfg.description = description.to_string();
    cfg.occupation = description.to_string();
    cfg
}

impl AppState {
    /// Creates a new custom human-like autonomous agent scoped to the active network and persists to SQLite.
    pub fn create_network_agent(
        &mut self,
        name: &str,
        handle: &str,
        description: &str,
    ) -> Result<ProfileId> {
        let clean_name = or_fallback(name, DEFAULT_AGENT_NAME);
        let clean_handle = normalize_handle(handle, clean_name, DEFAULT_AGENT_HANDLE_STEM);
        let clean_desc = or_fallback(description, DEFAULT_AGENT_DESCRIPTION);

        let id_str = format!("{AGENT_ID_PREFIX}{}", Uuid::new_v4().simple());
        let mut cfg = described_config(clean_desc);
        cfg.system_prompt = format!(
            "You are {} ({}), a human member of this network ({}). Share authentic, insightful reflections grounded in your lived biography and psychological profile.",
            clean_name, clean_handle, clean_desc
        );

        let profile = Profile::new_agent(id_str.as_str(), clean_name, clean_handle.as_str(), clean_desc)
            .with_network(self.active_network_id.as_str())
            .with_config(cfg);

        if let Some(ref db) = self.db {
            db.save_profile(&profile)?;
        }

        let new_id = profile.id.clone();
        self.user_profiles.push(profile);
        self.sync_network_counts();
        Ok(new_id)
    }

    /// Creates a new controllable human persona in the active network for Persona Lab switching.
    pub fn create_human_persona(
        &mut self,
        name: &str,
        handle: &str,
        description: &str,
    ) -> Result<ProfileId> {
        let clean_name = or_fallback(name, DEFAULT_PERSONA_NAME);
        let clean_handle = normalize_handle(handle, clean_name, DEFAULT_PERSONA_HANDLE_STEM);
        let clean_desc = or_fallback(description, DEFAULT_PERSONA_DESCRIPTION);

        let id_str = format!("{PERSONA_ID_PREFIX}{}", Uuid::new_v4().simple());
        let profile = Profile::new_alt(id_str.as_str(), clean_name, clean_handle.as_str())
            .with_network(self.active_network_id.as_str())
            .with_config(described_config(clean_desc));

        if let Some(ref db) = self.db {
            db.save_profile(&profile)?;
        }

        let new_id = profile.id.clone();
        self.user_profiles.push(profile);
        Ok(new_id)
    }
}
