//! Network domain selection, membership queries and derived counters.

use super::AppState;
use crate::domain::{GroundingDocument, Network, NetworkId, Profile};
use crate::error::{AppError, Result};
use crate::storage::SettingKey;

/// Recomputes every network's `doc_count` and `agent_count` from the documents and profiles
/// (the only source of truth; seed `networks.json` counts are never trusted) and returns the
/// networks whose counts changed.
pub(super) fn recount_networks<'a>(
    networks: &'a mut [Network],
    profiles: &[Profile],
    documents: &[GroundingDocument],
) -> Vec<&'a Network> {
    let mut changed = Vec::new();
    for net in networks.iter_mut() {
        let agents = profiles.iter().filter(|p| p.is_agent() && p.belongs_to_network(&net.id)).count();
        let docs = documents.iter().filter(|d| d.network_id == net.id).count();
        if net.agent_count != agents || net.doc_count != docs {
            net.agent_count = agents;
            net.doc_count = docs;
            changed.push(&*net);
        }
    }
    changed
}

impl AppState {
    /// Returns a reference to the active network domain.
    #[inline]
    pub fn current_network(&self) -> Option<&Network> {
        self.networks.iter().find(|n| n.id == self.active_network_id)
    }

    /// Switches the active network domain and persists selection.
    pub fn switch_network(&mut self, network_id: &NetworkId) -> Result<()> {
        if self.networks.iter().any(|n| &n.id == network_id) {
            self.active_network_id = network_id.clone();
            if let Some(ref db) = self.db {
                let _ = db.set_setting(SettingKey::ActiveNetwork, network_id.as_str());
            }
            Ok(())
        } else {
            Err(AppError::NetworkNotFound(network_id.to_string()))
        }
    }

    /// Returns all autonomous human-like member agents belonging to the currently active network.
    pub fn current_network_agents(&self) -> Vec<&Profile> {
        self.agents_for_network(&self.active_network_id)
    }

    /// Returns all autonomous human-like member agents belonging to the specified network.
    pub fn agents_for_network(&self, network_id: &NetworkId) -> Vec<&Profile> {
        self.user_profiles
            .iter()
            .filter(|p| p.is_agent() && p.belongs_to_network(network_id))
            .collect()
    }

    /// Synchronizes each network's `agent_count` and `doc_count` with actual state and
    /// queues the changed networks for the vault.
    pub fn sync_network_counts(&mut self) {
        let changed = recount_networks(&mut self.networks, &self.user_profiles, &self.documents);
        if let Some(ref db) = self.db {
            for net in changed {
                let _ = db.save_network(net);
            }
        }
    }
}
