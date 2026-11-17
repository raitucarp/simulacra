//! Every editable preference as a typed [`SettingField`]: its control kind and pure get / set
//! against [`Preferences`], so the settings screen renders, edits, resets and searches every
//! field the same way.

mod macros;
mod value;

mod content;
mod general;
mod models;
mod simulation;
mod system;

pub use content::{ImportField, WritingField};
pub use general::{AppearanceField, GeneralField};
pub use models::{AiField, EmbeddingField, ImageField};
pub use simulation::{AgentField, LearningField};
pub use system::{AdvancedField, NotificationField, PrivacyField};
pub use value::{FieldKind, NumberSpec, NumberUnit, SettingValue};

use super::Preferences;
use macros::setting_fields;

setting_fields! {
    General(GeneralField) in general => "general",
    Appearance(AppearanceField) in appearance => "appearance",
    Writing(WritingField) in writing => "writing",
    Ai(AiField) in ai => "ai",
    Embeddings(EmbeddingField) in embeddings => "embeddings",
    Images(ImageField) in images => "images",
    Agents(AgentField) in agents => "agents",
    Learning(LearningField) in learning => "learning",
    Import(ImportField) in import => "import",
    Privacy(PrivacyField) in privacy => "privacy",
    Notifications(NotificationField) in notifications => "notifications",
    Advanced(AdvancedField) in advanced => "advanced",
}

impl SettingField {
    /// `category.field`, unique across all preferences.
    pub fn id(self) -> String {
        format!("{}.{}", self.category().key(), self.name())
    }

    /// The field's value in the default preferences.
    pub fn default_value(self) -> SettingValue {
        self.get(&Preferences::default())
    }

    /// Whether the field differs from its default.
    pub fn is_modified(self, prefs: &Preferences) -> bool {
        self.get(prefs) != self.default_value()
    }

    /// `prefs` with only this field put back to its default.
    pub fn reset(self, prefs: Preferences) -> Preferences {
        self.set(prefs, &self.default_value())
    }
}
