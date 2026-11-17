//! Profile constructors and builder-style modifiers.

use super::{avatar_initial_of, Profile, ProfileKind, ProfileRole, DEFAULT_AGENT_NETWORK};
use crate::domain::agent_config::AgentConfig;
use crate::domain::ids::{NetworkId, ProfileId};

const PRIMARY_FALLBACK_INITIAL: char = 'U';
const ALT_FALLBACK_INITIAL: char = 'A';
const AGENT_FALLBACK_INITIAL: char = 'S';
const PRIMARY_DESCRIPTION: &str = "Primary Architect";
const ALT_DESCRIPTION: &str = "Alter-Ego Persona";

impl Profile {
    /// Creates a new primary user profile.
    pub fn new_primary(
        id: impl Into<ProfileId>,
        name: impl Into<String>,
        handle: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let initial = avatar_initial_of(&name, PRIMARY_FALLBACK_INITIAL);
        Self::assemble(id.into(), name, handle.into(), initial, ProfileRole::UserPrimary, None, PRIMARY_DESCRIPTION.to_string())
    }

    /// Creates an alternate user persona controllable via Persona Lab.
    pub fn new_alt(
        id: impl Into<ProfileId>,
        name: impl Into<String>,
        handle: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let initial = avatar_initial_of(&name, ALT_FALLBACK_INITIAL);
        Self::assemble(id.into(), name, handle.into(), initial, ProfileRole::UserAlt, None, ALT_DESCRIPTION.to_string())
    }

    /// Creates an autonomous human-like agent member.
    pub fn new_agent(
        id: impl Into<ProfileId>,
        name: impl Into<String>,
        handle: impl Into<String>,
        persona_type: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let persona_type = persona_type.into();
        let initial = avatar_initial_of(&name, AGENT_FALLBACK_INITIAL);
        let role = ProfileRole::Agent { persona_type: persona_type.clone() };
        let network = Some(NetworkId::new(DEFAULT_AGENT_NETWORK));
        Self::assemble(id.into(), name, handle.into(), initial, role, network, persona_type)
    }

    /// Shared field assembly; the default config receives the role-specific description.
    fn assemble(
        id: ProfileId,
        name: String,
        handle: String,
        avatar_initial: String,
        role: ProfileRole,
        network_id: Option<NetworkId>,
        description: String,
    ) -> Self {
        let config = AgentConfig { description, ..AgentConfig::default() };
        Self { id, name, handle, avatar_initial, role, network_id, avatar_path: None, config, kind: ProfileKind::Standard }
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
}
