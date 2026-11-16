//! Every field and trait option of the Voice, Daily Life, Relationships and Grimoire steps is
//! explained and labeled in English and Indonesian (flat keys in the `# forge steps B` block).

use crate::domain::agent_config::AgentConfig;
use crate::domain::TraitOption;
use crate::state::forge::{fields_for_step, ForgeStep, ToggleField, TraitField, TraitValue, TraitVisitor};
use crate::ui::features::forge::text::tr_opt;

const EN: &str = include_str!("../../../../../../../../locales/en.yml");
const ID: &str = include_str!("../../../../../../../../locales/id.yml");
const STEPS: [ForgeStep; 4] = [ForgeStep::Voice, ForgeStep::DailyLife, ForgeStep::Relationships, ForgeStep::Grimoire];
const EXPLAIN_PARTS: [&str; 2] = ["what", "effect"];
const TOGGLE_HINT_ROOT: &str = "forge.ui.toggle";

/// The label and description keys of every option of the visited trait.
struct OptionKeys;

impl TraitVisitor for OptionKeys {
    type Output = Vec<String>;

    fn visit<T: TraitOption>(self, _: T, _: fn(T) -> TraitValue) -> Vec<String> {
        T::ALL.iter().flat_map(|option| [option.label_key(), option.description_key()]).collect()
    }
}

/// Whether `yaml` has a top-level flat entry for `key` with a non-empty value.
fn has_flat_key(yaml: &str, key: &str) -> bool {
    let prefix = format!("{key}: ");
    yaml.lines().any(|line| line.strip_prefix(&prefix).is_some_and(|value| value.trim().len() > "\"\"".len()))
}

fn wanted_keys() -> Vec<String> {
    let mut keys = Vec::new();
    for step in STEPS {
        for path in fields_for_step(step) {
            keys.extend(EXPLAIN_PARTS.iter().map(|part| format!("forge.explain.{path}.{part}")));
            if let Some(field) = TraitField::from_path(path) {
                keys.extend(field.read(&AgentConfig::default()).visit(OptionKeys));
            }
        }
    }
    let injections = ToggleField::ALL.iter().filter(|field| field.step() == ForgeStep::Grimoire);
    keys.extend(injections.map(|field| format!("{TOGGLE_HINT_ROOT}.{}", field.path())));
    keys
}

#[test]
fn steps_b_fields_and_options_are_explained_in_both_locales() {
    let keys = wanted_keys();
    assert!(keys.len() > 150, "the four steps have many keys, found {}", keys.len());
    for (locale, yaml) in [("en", EN), ("id", ID)] {
        let missing: Vec<&String> = keys.iter().filter(|key| !has_flat_key(yaml, key)).collect();
        assert!(missing.is_empty(), "{locale} is missing {missing:?}");
    }
}

#[test]
fn indonesian_texts_are_translations_not_copies() {
    let same: Vec<String> = wanted_keys()
        .into_iter()
        .filter(|key| key.ends_with(".desc") || key.ends_with(".what"))
        .filter(|key| EN.lines().find(|l| l.starts_with(&format!("{key}: "))) == ID.lines().find(|l| l.starts_with(&format!("{key}: "))))
        .collect();
    assert!(same.is_empty(), "untranslated: {same:?}");
}

#[test]
fn flat_step_b_keys_resolve_at_runtime() {
    assert!(tr_opt("traits.chronotype.night_owl_insomniac.desc").is_some());
    assert!(tr_opt("forge.explain.prompt_template.what").is_some());
    assert!(tr_opt("forge.ui.toggle.inject_network_grounding").is_some());
}
