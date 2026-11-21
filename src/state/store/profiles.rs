//! Active profile selection, persona cycling, and profile update/deletion.

use super::{AppState, PRIMARY_PROFILE_ID};
use crate::domain::profile::canonical_handle;
use crate::domain::{Profile, ProfileId};
use crate::error::{AppError, Result};
use crate::state::seed::seed_profiles;
use crate::storage::SettingKey;

/// Whether `id` names a profile shipped as seed data (re-added on startup unless dismissed).
fn is_seed_profile(id: &ProfileId) -> bool {
    seed_profiles().iter().any(|seed| &seed.id == id)
}

impl AppState {
    /// Returns a reference to the currently active author profile.
    #[inline]
    pub fn current_profile(&self) -> Option<&Profile> {
        self.user_profiles.iter().find(|p| p.id == self.active_profile_id)
    }

    /// Clones the active profile, or reports it as missing.
    pub(super) fn current_profile_or_err(&self) -> Result<Profile> {
        self.current_profile()
            .cloned()
            .ok_or_else(|| AppError::ProfileNotFound(self.active_profile_id.to_string()))
    }

    /// Switches the active authoring profile and persists selection.
    pub fn switch_profile(&mut self, profile_id: &ProfileId) -> Result<()> {
        if self.user_profiles.iter().any(|p| &p.id == profile_id) {
            self.active_profile_id = profile_id.clone();
            if let Some(ref db) = self.db {
                let _ = db.set_setting(SettingKey::ActiveProfile, profile_id.as_str());
            }
            Ok(())
        } else {
            Err(AppError::ProfileNotFound(profile_id.to_string()))
        }
    }

    /// Activates the first controllable human persona (if any) and persists the selection.
    pub(super) fn activate_first_human_persona(&mut self) {
        if let Some(first_human) = self.user_profiles.iter().find(|p| p.is_human_persona()) {
            self.active_profile_id = first_human.id.clone();
            if let Some(ref db) = self.db {
                let _ = db.set_setting(SettingKey::ActiveProfile, self.active_profile_id.as_str());
            }
        }
    }

    /// Returns all controllable human personas available for profile switching in the active network.
    pub fn current_network_personas(&self) -> Vec<&Profile> {
        self.user_profiles
            .iter()
            .filter(|p| p.is_human_persona() && p.belongs_to_network(&self.active_network_id))
            .collect()
    }

    /// Cycles the active human persona to the next controllable persona in the current network.
    pub fn cycle_active_persona(&mut self) -> Option<ProfileId> {
        let persona_ids: Vec<ProfileId> = self
            .current_network_personas()
            .into_iter()
            .map(|p| p.id.clone())
            .collect();
        if persona_ids.is_empty() {
            return None;
        }
        let current_idx = persona_ids
            .iter()
            .position(|id| id == &self.active_profile_id)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % persona_ids.len();
        let next_id = persona_ids[next_idx].clone();
        let _ = self.switch_profile(&next_id);
        Some(next_id)
    }

    /// Updates an existing agent or human persona profile, synchronizes derived fields,
    /// and persists to SQLite. Posts reference authors by id, so they pick the change up as is
    /// (cached search listings are invalidated, since they match author names and handles).
    pub fn update_profile(&mut self, mut updated: Profile) -> Result<()> {
        updated.sync_derived_fields();
        let handle = canonical_handle(&updated.handle);
        if !handle.is_empty() {
            updated.handle = handle;
        }
        let target_id = updated.id.clone();

        if let Some(existing) = self.user_profiles.iter_mut().find(|p| p.id == target_id) {
            *existing = updated.clone();
        } else {
            self.user_profiles.push(updated.clone());
        }

        self.archived_authors.remove(&target_id);
        self.index.listing_changed();
        if let Some(ref db) = self.db {
            db.save_profile(&updated)?;
        }

        self.sync_network_counts();
        Ok(())
    }

    /// Deletes an agent or custom persona profile by ID; a deleted seed profile is
    /// remembered so startup never re-seeds it.
    /// Protects the primary human persona (`user_main`) from accidental deletion.
    pub fn delete_profile(&mut self, profile_id: &ProfileId) -> Result<()> {
        if profile_id.as_str() == PRIMARY_PROFILE_ID {
            return Ok(());
        }

        let position = self.user_profiles.iter().position(|p| &p.id == profile_id);
        let Some(removed) = position.map(|idx| self.user_profiles.remove(idx)) else {
            return Ok(());
        };
        if let Some(ref db) = self.db {
            db.archive_and_delete_profile(&removed)?;
            if is_seed_profile(&removed.id) {
                db.dismiss_seed(&removed.id)?;
            }
        }
        self.archived_authors.insert(removed.id.clone(), removed);
        self.index.listing_changed();

        if &self.active_profile_id == profile_id {
            self.activate_first_human_persona();
        }

        self.sync_network_counts();
        Ok(())
    }
}
