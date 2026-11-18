//! Classic thought experiments as editable starting points, written neutrally. Each preset
//! has two variables of two values, so it runs as a four-cell grid. Their wording lives in
//! the locale files (`arena.preset.<key>.*`); [`preset_scenario`] builds the scenario from a
//! translator, so this module stays free of any language.

use super::scenario::{parse_values, Scenario, Variable};
use crate::domain::document::keyed::keyed_enum;

/// Locale key prefix of the presets.
pub const PRESET_KEY_PREFIX: &str = "arena.preset";
/// The locale fields of one preset: its texts, then each variable's name and values.
const TEXT_FIELDS: [&str; 3] = ["title", "premise", "question"];
const VARIABLE_FIELDS: [(&str, &str); 2] = [("v1", "v1_values"), ("v2", "v2_values")];

keyed_enum! {
    /// A classic thought experiment.
    PresetKey {
        Trolley => "trolley",
        MarysRoom => "marys_room",
        ShipOfTheseus => "ship_of_theseus",
        ExperienceMachine => "experience_machine",
        VeilOfIgnorance => "veil_of_ignorance",
        ChineseRoom => "chinese_room",
        Newcomb => "newcomb",
        OriginalPosition => "original_position",
    }
}

impl PresetKey {
    /// The locale key of `field` (`arena.preset.trolley.premise`).
    pub fn locale_key(self, field: &str) -> String {
        format!("{PRESET_KEY_PREFIX}.{}.{field}", self.key())
    }

    /// Every locale key the preset needs.
    pub fn locale_keys(self) -> Vec<String> {
        let variables = VARIABLE_FIELDS.iter().flat_map(|(name, values)| [*name, *values]);
        TEXT_FIELDS.iter().copied().chain(variables).map(|field| self.locale_key(field)).collect()
    }
}

/// The scenario of `preset`, its texts looked up with `translate`.
pub fn preset_scenario(preset: PresetKey, translate: impl Fn(&str) -> String) -> Scenario {
    let text = |field: &str| translate(&preset.locale_key(field));
    let variables = VARIABLE_FIELDS.iter().map(|(name, values)| Variable { name: text(name), values: parse_values(&text(values)) }).collect();
    Scenario { title: text("title"), premise: text("premise"), variables, question: text("question") }
}
