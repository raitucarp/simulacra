//! Network domain selection, membership queries and derived counters.

use super::{AppState, ACTIVE_NETWORK_KEY};
use crate::domain::{Network, NetworkId, Profile};
use crate::error::{AppError, Result};

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
                let _ = db.set_setting(ACTIVE_NETWORK_KEY, network_id.as_str());
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

    /// Synchronizes each network's `agent_count` and `doc_count` with actual state.
    pub fn sync_network_counts(&mut self) {
        for net in &mut self.networks {
            let agent_cnt = self
                .user_profiles
                .iter()
                .filter(|p| p.is_agent() && p.belongs_to_network(&net.id))
                .count();
            let doc_cnt = self
                .documents
                .iter()
                .filter(|d| d.network_id == net.id)
                .count();
            if net.agent_count != agent_cnt || net.doc_count != doc_cnt {
                net.agent_count = agent_cnt;
                net.doc_count = doc_cnt;
                if let Some(ref db) = self.db {
                    let _ = db.save_network(net);
                }
            }
        }
    }
}
