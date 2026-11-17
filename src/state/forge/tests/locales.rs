//! Every i18n key the forge model hands to the UI exists in English and Indonesian.

use super::*;
use crate::domain::ArchetypeClass;
use crate::state::forge::fields::mapped_fields;
use std::collections::HashSet;

const EN: &str = include_str!("../../../../locales/en.yml");
const ID: &str = include_str!("../../../../locales/id.yml");
const INDENT: usize = 2;
const COMMENT: char = '#';

/// Dotted paths of every leaf in a (simple, two-space indented) locale YAML file.
fn leaf_keys(yaml: &str) -> HashSet<String> {
    let mut stack: Vec<String> = Vec::new();
    let mut keys = HashSet::new();
    for line in yaml.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with(COMMENT) {
            continue;
        }
        let depth = (line.len() - trimmed.len()) / INDENT;
        let Some((key, value)) = trimmed.split_once(':') else { continue };
        stack.truncate(depth);
        stack.push(key.trim().to_string());
        if !value.trim().is_empty() {
            keys.insert(stack.join("."));
        }
    }
    keys
}

fn forge_keys() -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();
    for step in ForgeStep::ALL {
        keys.extend([step.label_key(), step.tagline_key(), step.guide_tip_key()]);
    }
    for class in ArchetypeClass::ALL {
        keys.extend([class.label_key(), class.pitch_key(), class.description_key()]);
    }
    for palette in crate::domain::Palette::all() {
        keys.extend([palette.label_key(), palette.pitch_key()]);
        keys.extend(palette.is_full().then(|| palette.description_key()));
    }
    keys.extend(crate::domain::PaletteCategory::ALL.iter().map(|category| category.label_key()));
    keys.push(crate::domain::palette::category_label_key(crate::domain::palette::ALL_CATEGORIES_KEY));
    keys.extend(engine_change_keys());
    keys.extend(ForgeKind::ALL.iter().map(|kind| kind.label_key()));
    keys.extend(mapped_fields().into_iter().map(|(path, _)| field_label_key(path)));
    keys.extend(IssueKind::ALL.iter().map(|kind| kind.message_key()));
    keys.extend(QuotientLevel::ALL.iter().map(|level| level.label_key()));
    let warnings = [
        PatchWarning::UnknownField(String::new()),
        PatchWarning::InvalidOption { field: TraitField::Mbti, value: String::new() },
        PatchWarning::WrongType(String::new()),
        PatchWarning::OutOfRange { field: String::new(), value: String::new() },
    ];
    keys.extend(warnings.iter().map(PatchWarning::message_key));
    keys.extend([ParseError::NoJsonObject, ParseError::Malformed(String::new())].iter().map(ParseError::message_key));
    keys
}

#[test]
fn forge_keys_exist_in_english_and_indonesian() {
    for (locale, yaml) in [("en", EN), ("id", ID)] {
        let available = leaf_keys(yaml);
        let missing: Vec<String> = forge_keys().into_iter().filter(|k| !available.contains(k)).collect();
        assert!(missing.is_empty(), "{locale} is missing {missing:?}");
    }
}

#[test]
fn english_and_indonesian_forge_blocks_match() {
    let forge = |yaml| leaf_keys(yaml).into_iter().filter(|k| k.starts_with("forge.")).collect::<HashSet<_>>();
    assert_eq!(forge(EN), forge(ID));
}

#[test]
fn translations_are_resolved_at_runtime() {
    let key = ForgeStep::Origin.guide_tip_key();
    let english = rust_i18n::t!(key.as_str(), locale = "en").to_string();
    let indonesian = rust_i18n::t!(key.as_str(), locale = "id").to_string();
    assert_ne!(english, key);
    assert_ne!(english, indonesian);
}

#[test]
fn yaml_boolean_words_stay_string_keys() {
    let on = rust_i18n::t!("forge.ui.on", locale = "en").to_string();
    let off = rust_i18n::t!("forge.ui.off", locale = "en").to_string();
    assert_eq!((on.as_str(), off.as_str()), ("On", "Off"));
}
