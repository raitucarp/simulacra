//! The typed global preferences (`domain::preferences`): read through
//! [`AppState::preferences`], replaced (validated, then queued for the vault) through
//! [`AppState::update_preferences`].

use super::{AppState, NavigationTab};
use crate::domain::preferences::{Preferences, StartupScreen};
use crate::i18n::Language;

impl NavigationTab {
    /// The navigation tab a startup screen opens.
    pub fn of_startup(screen: StartupScreen) -> Self {
        match screen {
            StartupScreen::Timeline => Self::Timeline,
            StartupScreen::Knowledge => Self::Knowledge,
            StartupScreen::MindGraph => Self::MindGraph,
            StartupScreen::AgentLab => Self::AgentLab,
        }
    }
}

impl AppState {
    /// Every global preference (`state.preferences().<category>.<field>`).
    pub fn preferences(&self) -> &Preferences {
        &self.preferences
    }

    /// Validates `prefs`, adopts them and queues them for the vault (a failed write is
    /// reported through the vault's failure channel).
    pub fn update_preferences(&mut self, prefs: Preferences) {
        let prefs = prefs.validated();
        if prefs == self.preferences {
            return;
        }
        if let Some(ref db) = self.db {
            if let Err(err) = db.save_preferences(&prefs) {
                eprintln!("simulacra: preferences not queued: {err}");
            }
        }
        self.preferences = prefs;
    }

    /// Whether the autonomous scheduler is switched on (off on a fresh vault).
    pub fn scheduler_enabled(&self) -> bool {
        self.preferences.agents.autonomous_posting
    }

    /// Persists the autonomous-scheduler switch.
    pub fn set_scheduler_enabled(&mut self, enabled: bool) {
        let mut prefs = self.preferences.clone();
        prefs.agents.autonomous_posting = enabled;
        self.update_preferences(prefs);
    }

    /// Persists the interface language.
    pub fn set_language(&mut self, language: Language) {
        let mut prefs = self.preferences.clone();
        prefs.general.language = language;
        self.update_preferences(prefs);
    }
}
