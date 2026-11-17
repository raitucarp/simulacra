//! The field → step table, assembled from the typed field enums.

use super::{AvatarField, CountField, HourField, OptionalTextField, ScoreField, TextField, ToggleField, TraitField, TraitSet};
use crate::state::forge::step::{ForgeStep, FORGE_I18N_ROOT};

const FIELD_GROUP: &str = "field";

/// Profile fields the forge sets itself rather than through an edit control.
pub const SYSTEM_FIELDS: [(&str, ForgeStep); 2] = [
    ("role", ForgeStep::Origin),
    ("network_id", ForgeStep::Origin),
];

/// Every `(serde path, step)` pair the forge knows about.
pub fn mapped_fields() -> Vec<(&'static str, ForgeStep)> {
    let mut all: Vec<(&'static str, ForgeStep)> = SYSTEM_FIELDS.to_vec();
    all.extend(TextField::ALL.iter().map(|f| (f.path(), f.step())));
    all.extend(OptionalTextField::ALL.iter().map(|f| (f.path(), f.step())));
    all.extend(AvatarField::ALL.iter().map(|f| (f.path(), f.step())));
    all.extend(ScoreField::ALL.iter().map(|f| (f.path(), f.step())));
    all.extend(HourField::ALL.iter().map(|f| (f.path(), f.step())));
    all.extend(CountField::ALL.iter().map(|f| (f.path(), f.step())));
    all.extend(ToggleField::ALL.iter().map(|f| (f.path(), f.step())));
    all.extend(TraitField::ALL.iter().map(|f| (f.path(), f.step())));
    all.extend(TraitSet::ALL.iter().map(|f| (f.path(), f.step())));
    all
}

/// The step that owns the field with serde name `path`.
pub fn field_step(path: &str) -> Option<ForgeStep> {
    mapped_fields()
        .into_iter()
        .find_map(|(candidate, step)| (candidate == path).then_some(step))
}

/// Serde paths of every field `step` edits, in declaration order.
pub fn fields_for_step(step: ForgeStep) -> Vec<&'static str> {
    mapped_fields()
        .into_iter()
        .filter_map(|(path, owner)| (owner == step).then_some(path))
        .collect()
}

/// i18n key of a field's display name: `forge.field.{path}`.
pub fn field_label_key(path: &str) -> String {
    format!("{FORGE_I18N_ROOT}.{FIELD_GROUP}.{path}")
}

/// Upper bound of every [`ScoreField`].
pub const MAX_SCORE: u8 = 100;
/// Last hour of the day for [`HourField`]s.
pub const MAX_HOUR: u8 = 23;
/// Upper bound of [`CountField::Age`].
pub const MAX_AGE: u16 = 150;
