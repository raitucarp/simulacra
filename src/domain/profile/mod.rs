//! Profile entity representing human personas and autonomous network member agents.

mod constructors;
mod handle;
mod kind;
mod queries;
mod unknown;

pub use handle::{canonical_handle, handle_key};
pub use kind::{ProfileKind, ANONYMOUS_PROFILE_ID};

use super::agent_config::AgentConfig;
use super::ids::{NetworkId, ProfileId};
use serde::{Deserialize, Serialize};

/// Default network that unscoped autonomous agents belong to.
pub const DEFAULT_AGENT_NETWORK: &str = "net_philosophy";
/// Prefix of a rendered `@handle`.
pub const HANDLE_PREFIX: char = '@';

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
    /// Regular profile, pseudonym or the anonymous account (omitted from JSON when regular).
    #[serde(default, skip_serializing_if = "ProfileKind::is_standard")]
    pub kind: ProfileKind,
}

impl From<Profile> for ProfileId {
    /// A profile stands in for its id wherever an author reference is expected.
    #[inline]
    fn from(profile: Profile) -> Self {
        profile.id
    }
}

impl From<&Profile> for ProfileId {
    #[inline]
    fn from(profile: &Profile) -> Self {
        profile.id.clone()
    }
}

/// Upper-cased first character of `name`, or `fallback` when the name is empty.
fn avatar_initial_of(name: &str, fallback: char) -> String {
    name.chars().next().unwrap_or(fallback).to_uppercase().to_string()
}

#[cfg(test)]
mod tests;
