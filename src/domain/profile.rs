//! Profile entity representing human personas and autonomous network member agents.

use super::agent_config::AgentConfig;
use super::ids::{NetworkId, ProfileId};
use serde::{Deserialize, Serialize};

/// Role classification for identity separation and rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileRole {
    /// Primary user identity (Architect of this mind network).
    UserPrimary,
    /// Alternate user identity / alter-ego controllable in Persona Lab.
    UserAlt,
    /// Autonomous human-like member spawned into a specific network.
    Agent { persona_type: String },
}

/// Profile metadata and full psychological/linguistic configuration for authorship and interaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub id: ProfileId,
    pub name: String,
    pub handle: String,
    pub avatar_initial: String,
    pub role: ProfileRole,
    /// Network membership scope (`Some(net_id)` for network-bound agents and personas; `None` for global user personas).
    #[serde(default)]
    pub network_id: Option<NetworkId>,
    /// Optional local filesystem path to an uploaded custom avatar image.
    #[serde(default)]
    pub avatar_path: Option<String>,
    /// Comprehensive human psychology, intelligence, linguistics, schedule, and prompt assembly configuration.
    #[serde(default)]
    pub config: AgentConfig,
}

impl Profile {
    /// Creates a new primary user profile.
    pub fn new_primary(
        id: impl Into<ProfileId>,
        name: impl Into<String>,
        handle: impl Into<String>,
    ) -> Self {
        let name_str = name.into();
        let initial = name_str
            .chars()
            .next()
            .unwrap_or('U')
            .to_uppercase()
            .to_string();
        let mut config = AgentConfig::default();
        config.description = "Primary Architect".to_string();
        Self {
            id: id.into(),
            name: name_str,
            handle: handle.into(),
            avatar_initial: initial,
            role: ProfileRole::UserPrimary,
            network_id: None,
            avatar_path: None,
            config,
        }
    }

    /// Creates an alternate user persona controllable via Persona Lab.
    pub fn new_alt(
        id: impl Into<ProfileId>,
        name: impl Into<String>,
        handle: impl Into<String>,
    ) -> Self {
        let name_str = name.into();
        let initial = name_str
            .chars()
            .next()
            .unwrap_or('A')
            .to_uppercase()
            .to_string();
        let mut config = AgentConfig::default();
        config.description = "Alter-Ego Persona".to_string();
        Self {
            id: id.into(),
            name: name_str,
            handle: handle.into(),
            avatar_initial: initial,
            role: ProfileRole::UserAlt,
            network_id: None,
            avatar_path: None,
            config,
        }
    }

    /// Creates an autonomous human-like agent member.
    pub fn new_agent(
        id: impl Into<ProfileId>,
        name: impl Into<String>,
        handle: impl Into<String>,
        persona_type: impl Into<String>,
    ) -> Self {
        let name_str = name.into();
        let persona_type_str = persona_type.into();
        let initial = name_str
            .chars()
            .next()
            .unwrap_or('S')
            .to_uppercase()
            .to_string();
        let mut config = AgentConfig::default();
        config.description = persona_type_str.clone();
        Self {
            id: id.into(),
            name: name_str,
            handle: handle.into(),
            avatar_initial: initial,
            role: ProfileRole::Agent {
                persona_type: persona_type_str,
            },
            network_id: Some(NetworkId::new("net_philosophy")),
            avatar_path: None,
            config,
        }
    }

    /// Assigns this profile/agent to a specific network domain.
    #[must_use]
    pub fn with_network(mut self, network_id: impl Into<NetworkId>) -> Self {
        self.network_id = Some(network_id.into());
        self
    }

    /// Sets an uploaded avatar image file path.
    #[must_use]
    pub fn with_avatar_path(mut self, avatar_path: impl Into<String>) -> Self {
        let path_str = avatar_path.into();
        if path_str.trim().is_empty() {
            self.avatar_path = None;
        } else {
            self.avatar_path = Some(path_str);
        }
        self
    }

    /// Attaches a full psychological and linguistic configuration.
    #[must_use]
    pub fn with_config(mut self, config: AgentConfig) -> Self {
        if let ProfileRole::Agent { ref mut persona_type } = self.role {
            if !config.description.trim().is_empty() {
                *persona_type = config.description.clone();
            }
        }
        self.config = config;
        self
    }

    /// Synchronizes `avatar_initial` and `ProfileRole::Agent` description with updated fields.
    pub fn sync_derived_fields(&mut self) {
        if self.avatar_initial.trim().is_empty() {
            self.avatar_initial = self
                .name
                .chars()
                .next()
                .unwrap_or('H')
                .to_uppercase()
                .to_string();
        }
        if let ProfileRole::Agent { ref mut persona_type } = self.role {
            if !self.config.description.trim().is_empty() {
                *persona_type = self.config.description.clone();
            }
        }
    }

    /// Returns whether this profile is an autonomous network agent.
    #[inline]
    pub fn is_agent(&self) -> bool {
        matches!(self.role, ProfileRole::Agent { .. })
    }

    /// Returns whether this profile is a human-controllable persona (Primary or Alt).
    #[inline]
    pub fn is_human_persona(&self) -> bool {
        !self.is_agent()
    }

    /// Returns whether this profile is a member of the specified `network_id`.
    ///
    /// - Autonomous agents belong strictly to a single network (`self.network_id`).
    /// - Human personas belong to `network_id` if scoped to it or if global (`None`).
    #[inline]
    pub fn belongs_to_network(&self, network_id: &NetworkId) -> bool {
        match &self.network_id {
            Some(net_id) => net_id == network_id,
            None => {
                if self.is_agent() {
                    network_id.as_str() == "net_philosophy"
                } else {
                    true
                }
            }
        }
    }

    /// Returns human-readable role badge string.
    pub fn role_label(&self) -> &str {
        match &self.role {
            ProfileRole::UserPrimary => "YOU",
            ProfileRole::UserAlt => "PERSONA",
            ProfileRole::Agent { persona_type } => {
                if persona_type.is_empty() {
                    "MEMBER"
                } else {
                    persona_type.as_str()
                }
            }
        }
    }

    /// Returns the handle formatted with a leading `@` symbol.
    pub fn display_handle(&self) -> String {
        if self.handle.starts_with('@') {
            self.handle.clone()
        } else {
            format!("@{}", self.handle)
        }
    }
}
