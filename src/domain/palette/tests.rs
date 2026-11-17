use super::catalog::{parse, scope_of};
use super::*;
use crate::domain::agent_config::AgentConfig;
use crate::domain::archetype::config_map;
use crate::domain::portrait::{AvatarMode, FormLayer};
use crate::domain::traits::BodyForm;
use crate::domain::TraitOption;
use std::collections::HashSet;
use std::path::Path;

const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");
const MIN_NAMES: usize = 4;
const MIN_FULL_PALETTES: usize = 28;
const MIN_PER_CATEGORY: usize = 3;
/// Top-level keys a palette may set beyond the serialized default config.
const OPTIONAL_KEYS: [&str; 1] = ["avatar"];

/// Whether `expected` is contained in `actual`: objects key by key, string lists as sets,
/// other lists element by element, anything else equal.
fn contains(actual: &Value, expected: &Value) -> bool {
    match (actual, expected) {
        (Value::Object(a), Value::Object(e)) => e.iter().all(|(k, v)| a.get(k).is_some_and(|got| contains(got, v))),
        (Value::Array(a), Value::Array(e)) if e.iter().all(Value::is_string) => {
            a.len() == e.len() && e.iter().all(|item| a.contains(item))
        }
        (Value::Array(a), Value::Array(e)) => a.len() == e.len() && a.iter().zip(e).all(|(got, want)| contains(got, want)),
        _ => actual == expected,
    }
}

#[test]
fn every_catalog_file_is_placed_and_parses() {
    let mut keys = HashSet::new();
    for path in PaletteFiles::iter() {
        let file = PaletteFiles::get(&path).expect("listed file");
        let palette = parse(&path, &file.data).unwrap_or_else(|| panic!("{path} is misplaced or malformed"));
        assert!(keys.insert(palette.key.clone()), "duplicate palette key {}", palette.key);
        assert!(!palette.config.is_empty(), "{path} sets nothing");
    }
    assert_eq!(keys.len(), Palette::all().len());
    assert!(scope_of("misc/x.json").is_none() && scope_of("steps/voice.json").is_none());
}

#[test]
fn the_gallery_covers_every_category() {
    assert!(Palette::full(None).count() >= MIN_FULL_PALETTES);
    for category in PaletteCategory::ALL {
        assert!(Palette::full(Some(category)).count() >= MIN_PER_CATEGORY, "{category:?}");
        assert!(Path::new(ASSETS_DIR).join(category.illustration()).is_file(), "{category:?} icon");
    }
    assert!(Path::new(ASSETS_DIR).join(category_illustration(ALL_CATEGORIES_KEY)).is_file());
    let thinkers: Vec<&str> = Palette::full(Some(PaletteCategory::Thinkers)).map(|p| p.key.as_str()).collect();
    let classes: Vec<&str> = crate::domain::ArchetypeClass::ALL.iter().map(|c| c.key()).collect();
    assert_eq!(thinkers, classes, "the eight classes are the thinkers, in class order");
}

#[test]
fn every_palette_sets_real_fields_without_resetting_the_config() {
    let mut known: HashSet<String> = config_map(&AgentConfig::default()).keys().cloned().collect();
    known.extend(OPTIONAL_KEYS.iter().map(|k| k.to_string()));
    for palette in Palette::all() {
        let applied = config_map(&palette.applied(&AgentConfig::default()));
        for (field, value) in palette.config.iter().filter(|(field, _)| !OPTIONAL_KEYS.contains(&field.as_str())) {
            assert!(known.contains(field), "{}: unknown field {field}", palette.key);
            assert!(applied.get(field).is_some_and(|got| contains(got, value)), "{}: {field} did not take {value}", palette.key);
        }
        assert!(palette.is_full() || palette.names.is_empty(), "{}: names only suit full palettes", palette.key);
    }
}

#[test]
fn full_palettes_have_names_texts_and_art() {
    for palette in Palette::full(None) {
        let key = &palette.key;
        assert!(palette.names.len() >= MIN_NAMES, "{key} names");
        let unique: HashSet<_> = palette.names.iter().map(|n| n.name.as_str()).collect();
        assert_eq!(unique.len(), palette.names.len(), "{key} duplicate names");
        let cfg = palette.applied(&AgentConfig::default());
        assert_eq!(palette.enneagram_options.first(), Some(&cfg.enneagram), "{key} enneagram");
        assert!(!cfg.bio.is_empty() && !cfg.description.is_empty(), "{key} bio");
        assert_eq!(cfg.palette.as_deref(), Some(key.as_str()));
        assert!(Path::new(ASSETS_DIR).join(palette.illustration()).is_file(), "{key} art");
    }
}

#[test]
fn step_palettes_have_cards() {
    assert!(Palette::all().iter().filter(|p| !p.is_full()).count() >= 20);
    for palette in Palette::all().iter().filter(|p| !p.is_full()) {
        assert!(Path::new(ASSETS_DIR).join(palette.illustration()).is_file(), "{} card", palette.key);
    }
}

#[test]
fn a_palette_body_comes_with_a_matching_form_avatar() {
    for palette in Palette::full(None).filter(|p| p.sets("form")) {
        let cfg = palette.applied(&AgentConfig::default());
        let form: BodyForm = serde_json::from_value(palette.config["form"].clone()).expect("form");
        assert_eq!(cfg.embodiment.form, form, "{}", palette.key);
        let Some(AvatarMode::Form(spec)) = &cfg.avatar else { panic!("{}: no form avatar", palette.key) };
        assert_eq!(spec.form(), form.key(), "{}", palette.key);
        let parts = palette.config.get("avatar").and_then(|a| a.get("parts")).and_then(Value::as_object);
        for (layer, variant) in parts.into_iter().flatten() {
            let layer: FormLayer = serde_json::from_value(Value::String(layer.clone())).expect("layer");
            assert_eq!(Some(spec.part(layer)), variant.as_str(), "{}: {layer:?} variant", palette.key);
        }
        assert!(palette.changes_for(&cfg).is_empty(), "{}: applying twice changes nothing", palette.key);
    }
}

#[test]
fn a_full_palette_keeps_an_emblem_and_a_thinker_keeps_the_body() {
    let tin_can = Palette::find("melancholy_tin_can").expect("tin can");
    let emblem = AvatarMode::Emblem(Default::default());
    let mut cfg = AgentConfig { avatar: Some(emblem.clone()), ..AgentConfig::default() };
    tin_can.apply_to(&mut cfg);
    assert_eq!(cfg.embodiment.form, BodyForm::TinCan);
    assert_eq!(cfg.avatar, Some(emblem), "an emblem is the user's own choice");
    Palette::find("stoic_sage").expect("sage").apply_to(&mut cfg);
    assert_eq!(cfg.embodiment.form, BodyForm::TinCan, "a thinker's mind in a tin can");
    assert_eq!(cfg.palette.as_deref(), Some("stoic_sage"));
}

#[test]
fn json_merges_objects_deeply_and_replaces_the_rest() {
    let mut base = serde_json::json!({"a": {"b": 1, "c": [1, 2]}, "d": "x"});
    merge_json(&mut base, &serde_json::json!({"a": {"c": [3], "e": true}}));
    assert_eq!(base, serde_json::json!({"a": {"b": 1, "c": [3], "e": true}, "d": "x"}));
    let mut output = serde_json::json!({"format": "json_schema", "preset": "thread", "schema": "{}"});
    merge_json(&mut output, &serde_json::json!({"format": "free_text"}));
    assert_eq!(output, serde_json::json!({"format": "free_text"}), "another variant replaces the old one");
}
