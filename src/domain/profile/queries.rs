//! Derived-field synchronization, membership checks, and display helpers.

use super::{avatar_initial_of, Profile, ProfileRole, DEFAULT_AGENT_NETWORK, HANDLE_PREFIX};
use crate::domain::ids::NetworkId;

const SYNC_FALLBACK_INITIAL: char = 'H';
const PRIMARY_BADGE: &str = "YOU";
const ALT_BADGE: &str = "PERSONA";
const UNNAMED_AGENT_BADGE: &str = "MEMBER";

impl Profile {
    /// Synchronizes `avatar_initial` and `ProfileRole::Agent` description with updated fields.
    pub fn sync_derived_fields(&mut self) {
        if self.avatar_initial.trim().is_empty() {
            self.avatar_initial = avatar_initial_of(&self.name, SYNC_FALLBACK_INITIAL);
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
                    network_id.as_str() == DEFAULT_AGENT_NETWORK
                } else {
                    true
                }
            }
        }
    }

    /// Returns human-readable role badge string.
    pub fn role_label(&self) -> &str {
        match &self.role {
            ProfileRole::UserPrimary => PRIMARY_BADGE,
            ProfileRole::UserAlt => ALT_BADGE,
            ProfileRole::Agent { persona_type } => {
                if persona_type.is_empty() {
                    UNNAMED_AGENT_BADGE
                } else {
                    persona_type.as_str()
                }
            }
        }
    }

    /// Returns the handle formatted with a leading `@` symbol.
    pub fn display_handle(&self) -> String {
        if self.handle.starts_with(HANDLE_PREFIX) {
            self.handle.clone()
        } else {
            format!("{HANDLE_PREFIX}{}", self.handle)
        }
    }
}
