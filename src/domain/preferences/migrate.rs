//! One-time migration of the settings rows that predate [`Preferences`] (pure: the storage
//! layer reads the raw rows and persists the result).

use super::Preferences;

/// Raw values of the legacy `settings` rows folded into the preferences.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LegacyValues {
    /// The `scheduler_enabled` row (`"true"` / `"false"`).
    pub scheduler_enabled: Option<String>,
}

/// `base` with every readable legacy value moved in; unreadable values keep the default.
pub fn migrate_legacy(base: Preferences, legacy: &LegacyValues) -> Preferences {
    let autonomous_posting = legacy
        .scheduler_enabled
        .as_deref()
        .and_then(|raw| raw.trim().parse::<bool>().ok())
        .unwrap_or(base.agents.autonomous_posting);
    let agents = super::AgentPrefs { autonomous_posting, ..base.agents };
    Preferences { agents, ..base }.validated()
}
