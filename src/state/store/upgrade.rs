//! In-place upgrade of legacy bot-named profiles from older vaults.

use super::AppState;
use crate::domain::{Profile, ProfileId};
use crate::state::seed::seed_profiles;
use std::collections::BTreeSet;

/// Name fragments used by legacy bot-style agents.
const LEGACY_NAME_FRAGMENTS: [&str; 2] = ["GPT", "-Node"];
/// Handles used by legacy bot-style agents.
const LEGACY_HANDLES: [&str; 2] = ["@socrates_ai", "@spinoza_bot"];
/// Exact display names used by legacy personas.
const LEGACY_NAMES: [&str; 2] = ["You (Architect)", "Ghost Writer"];

/// Whether a profile predates the human-thinker model (bot naming or unscoped agent).
fn is_legacy_profile(p: &Profile) -> bool {
    LEGACY_NAME_FRAGMENTS.iter().any(|f| p.name.contains(f))
        || LEGACY_HANDLES.contains(&p.handle.as_str())
        || LEGACY_NAMES.contains(&p.name.as_str())
        || (p.is_agent() && p.network_id.is_none())
}

impl AppState {
    /// Automatically upgrades legacy bot-named profiles (`Socrates-GPT`, `Spinoza-Node`, etc.)
    /// or missing per-network human agents in an existing SQLite database so all agents are
    /// modeled as authentic human thinkers scoped to their respective networks. Seeds the
    /// user deleted (`dismissed`) stay deleted.
    pub(super) fn upgrade_legacy_profiles_if_needed(&mut self, dismissed: &BTreeSet<ProfileId>) {
        let has_legacy_bot = self.user_profiles.iter().any(is_legacy_profile);
        let fresh_seeds = seed_profiles();
        let mut mutated = false;

        if has_legacy_bot {
            mutated |= self.overwrite_with_seeds(&fresh_seeds);
        }
        // Ensure all seeded per-network human agents and personas exist
        mutated |= self.add_missing_seeds(fresh_seeds, dismissed);

        // Ensure active_profile_id points to a human-controlled persona rather than an autonomous agent
        let active_is_agent = self
            .user_profiles
            .iter()
            .find(|p| p.id == self.active_profile_id)
            .map_or(true, |p| p.is_agent());
        if active_is_agent {
            self.activate_first_human_persona();
        }

        if mutated {
            self.persist_profile_snapshots();
        }
    }

    /// Replaces existing profiles that share an id with a fresh seed; returns whether any changed.
    fn overwrite_with_seeds(&mut self, fresh_seeds: &[Profile]) -> bool {
        let mut mutated = false;
        for seeded in fresh_seeds {
            if let Some(existing) = self.user_profiles.iter_mut().find(|p| p.id == seeded.id) {
                *existing = seeded.clone();
                mutated = true;
            }
        }
        mutated
    }

    /// Appends seeds that are neither present nor dismissed by the user; returns whether any
    /// were added (a seed shipped by a newer version is added once, like a first run).
    fn add_missing_seeds(&mut self, fresh_seeds: Vec<Profile>, dismissed: &BTreeSet<ProfileId>) -> bool {
        let mut mutated = false;
        for seeded in fresh_seeds {
            let present = self.user_profiles.iter().any(|p| p.id == seeded.id);
            if !present && !dismissed.contains(&seeded.id) {
                self.user_profiles.push(seeded);
                mutated = true;
            }
        }
        mutated
    }

    /// Persists every profile to SQLite (posts resolve their authors by id).
    fn persist_profile_snapshots(&mut self) {
        let Some(ref db) = self.db else { return };
        for prof in &self.user_profiles {
            let _ = db.save_profile(prof);
        }
    }
}
