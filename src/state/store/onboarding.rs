//! The first-run welcome: whether to offer it, and remembering that it was completed or
//! skipped (`SettingKey::OnboardingDone` in the `settings` table).

use super::AppState;
use crate::domain::ProfileId;
use crate::state::seed::seed_profiles;
use crate::storage::SettingKey;
use std::collections::HashSet;

/// A fresh vault has not finished the welcome yet.
const ONBOARDING_DONE_DEFAULT: bool = false;

/// Whether the welcome should open: never finished or skipped, and no thinker made yet.
pub fn needs_onboarding(done: bool, user_created_agents: usize) -> bool {
    !done && user_created_agents == 0
}

impl AppState {
    /// Whether the welcome was completed or skipped (persisted; `false` on a fresh vault).
    pub fn onboarding_done(&self) -> bool {
        self.db
            .as_ref()
            .and_then(|db| db.get_setting(SettingKey::OnboardingDone).ok().flatten())
            .and_then(|raw| raw.parse::<bool>().ok())
            .unwrap_or(ONBOARDING_DONE_DEFAULT)
    }

    /// Persists whether the welcome was completed or skipped.
    pub fn set_onboarding_done(&self, done: bool) {
        if let Some(ref db) = self.db {
            if let Err(e) = db.set_setting(SettingKey::OnboardingDone, &done.to_string()) {
                eprintln!("simulacra: failed to persist the onboarding flag: {e}");
            }
        }
    }

    /// Agents the user made: every agent that is not one of the seeded demo members.
    pub fn user_created_agents(&self) -> usize {
        let seeded: HashSet<ProfileId> = seed_profiles().into_iter().map(|profile| profile.id).collect();
        self.user_profiles.iter().filter(|profile| profile.is_agent() && !seeded.contains(&profile.id)).count()
    }

    /// Whether to open the welcome at startup.
    pub fn should_offer_onboarding(&self) -> bool {
        needs_onboarding(self.onboarding_done(), self.user_created_agents())
    }
}
