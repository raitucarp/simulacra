//! [`FormKind`]: which non-human being a form body is (one catalog folder per kind).

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// i18n namespace of form labels (`forms.kind_<kind>`, `forms.form_<form>`, …).
pub const FORM_I18N_ROOT: &str = "forms";

/// The non-human body kinds; humans use the generated [`PortraitSpec`](super::super::PortraitSpec).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FormKind {
    Creature,
    Plant,
    #[default]
    Object,
    Element,
    Machine,
    Place,
    Abstraction,
}

impl FormKind {
    /// Every kind in picker order.
    pub const ALL: [FormKind; 7] =
        [Self::Creature, Self::Plant, Self::Object, Self::Element, Self::Machine, Self::Place, Self::Abstraction];

    /// Stable snake_case key: catalog folder, serde name and i18n suffix.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Creature => "creature",
            Self::Plant => "plant",
            Self::Object => "object",
            Self::Element => "element",
            Self::Machine => "machine",
            Self::Place => "place",
            Self::Abstraction => "abstraction",
        }
    }

    /// The kind named `key`, if any.
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.key() == key)
    }

    /// i18n key of this kind's label (`forms.kind_object`).
    pub fn label_key(self) -> String {
        format!("{FORM_I18N_ROOT}.kind_{}", self.key())
    }
}

impl Serialize for FormKind {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.key())
    }
}

/// Unknown kinds (from a newer release) load as the default kind instead of failing the
/// whole avatar, and with it the agent's config.
impl<'de> Deserialize<'de> for FormKind {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let key = String::deserialize(deserializer)?;
        Ok(Self::from_key(&key).unwrap_or_default())
    }
}
