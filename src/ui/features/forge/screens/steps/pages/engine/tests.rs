//! Every caption, explainer, section title and option card of the Engine step is translated
//! in every locale (flat keys in the `# p8 engine ui` block).

use crate::agent::llm::LlmProvider;
use crate::domain::engine::{
    HttpMethod, ImageEngineKind, ImageSize, OutputKind, RequestKind, SchemaPreset, SdFlavor, SearchBackend, TextSource, ToolKind,
    TriggerKind,
};
use crate::domain::TraitOption;
use crate::state::forge::{EngineSection, ForgeStep, IssueKind};

const LOCALES: [&str; 10] = [
    include_str!("../../../../../../../../locales/en.yml"),
    include_str!("../../../../../../../../locales/id.yml"),
    include_str!("../../../../../../../../locales/de.yml"),
    include_str!("../../../../../../../../locales/fr.yml"),
    include_str!("../../../../../../../../locales/es.yml"),
    include_str!("../../../../../../../../locales/pt.yml"),
    include_str!("../../../../../../../../locales/ru.yml"),
    include_str!("../../../../../../../../locales/ar.yml"),
    include_str!("../../../../../../../../locales/ja.yml"),
    include_str!("../../../../../../../../locales/zh-CN.yml"),
];
/// The page sources, scanned for the pseudo-fields they caption.
const PAGES: [&str; 6] = [
    include_str!("model.rs"),
    include_str!("request.rs"),
    include_str!("output.rs"),
    include_str!("image.rs"),
    include_str!("tools.rs"),
    include_str!("custom_tool.rs"),
];
const FIELD_CALL: &str = "engine_field(\"";
const ENGINE_ISSUE_PREFIX: &str = "engine_";

fn has_flat_key(yaml: &str, key: &str) -> bool {
    let prefix = format!("{key}: ");
    yaml.lines().any(|line| line.strip_prefix(&prefix).is_some_and(|value| value.trim().len() > "\"\"".len()))
}

fn option_keys<T: TraitOption>() -> Vec<String> {
    T::ALL.iter().flat_map(|option| [option.label_key(), option.description_key()]).collect()
}

/// Every pseudo-field path the pages caption.
fn captioned_paths() -> Vec<String> {
    let mut paths: Vec<String> = PAGES
        .iter()
        .flat_map(|source| source.split(FIELD_CALL).skip(1).filter_map(|rest| rest.split('"').next().map(str::to_string)))
        .collect();
    paths.sort();
    paths.dedup();
    paths
}

fn wanted_keys() -> Vec<String> {
    let mut keys = Vec::new();
    for path in captioned_paths() {
        keys.extend([format!("forge.field.{path}"), format!("forge.explain.{path}.what"), format!("forge.explain.{path}.effect")]);
    }
    for section in EngineSection::ALL {
        keys.extend([section.label_key(), section.intro_key(), format!("{}_tab", section.label_key())]);
    }
    let step = ForgeStep::Engine;
    keys.extend([step.label_key(), step.tagline_key(), step.guide_tip_key()]);
    keys.extend(IssueKind::ALL.iter().filter(|kind| kind.key().starts_with(ENGINE_ISSUE_PREFIX)).map(|kind| kind.message_key()));
    keys.extend(option_keys::<TextSource>());
    keys.extend(option_keys::<LlmProvider>());
    keys.extend(option_keys::<RequestKind>());
    keys.extend(option_keys::<HttpMethod>());
    keys.extend(option_keys::<OutputKind>());
    keys.extend(option_keys::<SchemaPreset>());
    keys.extend(option_keys::<ImageEngineKind>());
    keys.extend(option_keys::<SdFlavor>());
    keys.extend(option_keys::<ImageSize>());
    keys.extend(option_keys::<TriggerKind>());
    keys.extend(option_keys::<ToolKind>());
    keys.extend(option_keys::<SearchBackend>());
    keys
}

#[test]
fn every_engine_caption_section_and_card_is_translated_everywhere() {
    let keys = wanted_keys();
    assert!(captioned_paths().len() > 25, "the pages caption many fields: {:?}", captioned_paths());
    for yaml in LOCALES {
        let missing: Vec<&String> = keys.iter().filter(|key| !has_flat_key(yaml, key)).collect();
        assert!(missing.is_empty(), "missing {missing:?}");
    }
}

#[test]
fn descriptions_are_translations_not_copies() {
    let english = |key: &str| LOCALES[0].lines().find(|l| l.starts_with(&format!("{key}: "))).map(str::to_string);
    for yaml in &LOCALES[1..] {
        let same: Vec<String> = wanted_keys()
            .into_iter()
            .filter(|key| key.ends_with(".what") || key.ends_with("_intro"))
            .filter(|key| english(key) == yaml.lines().find(|l| l.starts_with(&format!("{key}: "))).map(str::to_string))
            .collect();
        assert!(same.is_empty(), "untranslated: {same:?}");
    }
}
