//! One typed change to a draft profile.

use super::fields::{
    AvatarField, CountField, HourField, OptionalTextField, ScoreField, TextField, ToggleField, TraitValue,
    MAX_AGE, MAX_HOUR, MAX_SCORE,
};
use super::step::ForgeStep;
use crate::domain::portrait::AvatarMode;
use crate::domain::Profile;
use serde::{Deserialize, Serialize};

/// A single edit the forge can apply (and undo). Numbers are clamped to their field's range.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DraftEdit {
    Text(TextField, String),
    OptionalText(OptionalTextField, Option<String>),
    Avatar(AvatarField, Option<AvatarMode>),
    Score(ScoreField, u8),
    Hour(HourField, u8),
    Count(CountField, u16),
    Toggle(ToggleField, bool),
    Trait(TraitValue),
}

/// Writes `value` into `slot`, reporting whether anything changed.
fn assign<T: PartialEq>(slot: &mut T, value: T) -> bool {
    if *slot == value {
        return false;
    }
    *slot = value;
    true
}

impl DraftEdit {
    /// Serde path of the edited field.
    pub fn path(&self) -> &'static str {
        match self {
            Self::Text(field, _) => field.path(),
            Self::OptionalText(field, _) => field.path(),
            Self::Avatar(field, _) => field.path(),
            Self::Score(field, _) => field.path(),
            Self::Hour(field, _) => field.path(),
            Self::Count(field, _) => field.path(),
            Self::Toggle(field, _) => field.path(),
            Self::Trait(value) => value.field().path(),
        }
    }

    /// The step that owns the edited field.
    pub fn step(&self) -> ForgeStep {
        match self {
            Self::Text(field, _) => field.step(),
            Self::OptionalText(field, _) => field.step(),
            Self::Avatar(field, _) => field.step(),
            Self::Score(field, _) => field.step(),
            Self::Hour(field, _) => field.step(),
            Self::Count(field, _) => field.step(),
            Self::Toggle(field, _) => field.step(),
            Self::Trait(value) => value.field().step(),
        }
    }

    /// Text field whose consecutive edits merge into one undo step.
    pub(super) fn coalesce_key(&self) -> Option<TextField> {
        match self {
            Self::Text(field, _) => Some(*field),
            _ => None,
        }
    }

    /// Applies the edit to `profile`; returns whether the profile changed.
    pub fn apply(&self, profile: &mut Profile) -> bool {
        match self {
            Self::Text(field, value) => assign(field.get_mut(profile), value.clone()),
            Self::OptionalText(field, value) => assign(field.get_mut(profile), value.clone()),
            Self::Avatar(field, value) => assign(field.get_mut(profile), value.clone()),
            Self::Score(field, value) => assign(field.get_mut(profile), (*value).min(MAX_SCORE)),
            Self::Hour(field, value) => assign(field.get_mut(profile), (*value).min(MAX_HOUR)),
            Self::Count(field, value) => assign(field.get_mut(profile), (*value).min(MAX_AGE)),
            Self::Toggle(field, value) => assign(field.get_mut(profile), *value),
            Self::Trait(value) => {
                let before = value.field().read(&profile.config);
                value.write(&mut profile.config);
                before != *value
            }
        }
    }
}
