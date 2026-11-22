//! Typed global preferences: one plain `#[serde(default)]` struct per settings category,
//! pure validation (clamping) and the one-time migration of the pre-preferences keys.
//!
//! - `categories`: the category structs and their numeric [`Bounds`].
//! - `choices`: the option sets of the one-of-many preferences ([`Choice`]).
//! - `fields`: every editable preference as a [`SettingField`] with typed get / set, so the
//!   settings screen edits, resets and searches them generically.
//! - `migrate`: folds the legacy `settings` rows into a fresh [`Preferences`].
//!
//! Each category is persisted as its own JSON row (`preferences.<category>`), so one
//! unreadable row only resets that category.

mod bounds;
mod categories;
mod choice;
mod choices;
mod fields;
mod format;
mod migrate;

#[cfg(test)]
mod tests;

pub use bounds::{Bounds, HOUR_OF_DAY};
pub use categories::*;
pub use choice::Choice;
pub use choices::*;
pub use fields::*;
pub use format::group_thousands;
pub use migrate::{migrate_legacy, LegacyValues};

use serde::{Deserialize, Serialize};

/// Every global preference, grouped by settings category.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub general: GeneralPrefs,
    pub appearance: AppearancePrefs,
    pub writing: WritingPrefs,
    pub ai: AiPrefs,
    pub embeddings: EmbeddingPrefs,
    pub images: ImagePrefs,
    pub agents: AgentPrefs,
    pub learning: LearningPrefs,
    pub import: ImportPrefs,
    pub privacy: PrivacyPrefs,
    pub notifications: NotificationPrefs,
    /// Running code blocks (P4): off until switched on.
    pub code: CodePrefs,
    pub advanced: AdvancedPrefs,
}

impl Preferences {
    /// Every category clamped into range (idempotent).
    pub fn validated(self) -> Self {
        Self {
            general: self.general.validated(),
            appearance: self.appearance.validated(),
            writing: self.writing.validated(),
            ai: self.ai.validated(),
            embeddings: self.embeddings.validated(),
            images: self.images.validated(),
            agents: self.agents.validated(),
            learning: self.learning.validated(),
            import: self.import.validated(),
            privacy: self.privacy.validated(),
            notifications: self.notifications.validated(),
            code: self.code.validated(),
            advanced: self.advanced.validated(),
        }
    }
}
