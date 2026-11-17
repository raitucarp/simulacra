use super::*;
use crate::domain::palette::Palette;
use crate::domain::traits::{BodyForm, Weekday};
use crate::domain::TraitOption;
use crate::domain::engine::{OutputMode, SchemaPreset, ToolKind};

fn find(key: &str) -> &'static Palette {
    Palette::find(key).unwrap_or_else(|| panic!("palette {key}"))
}

#[test]
fn step_palettes_only_touch_their_own_step() {
    for palette in Palette::all().iter().filter(|p| !p.is_full()) {
        let step = palette_step(palette).unwrap_or_else(|| panic!("{}: unknown step", palette.key));
        for field in palette.config.keys() {
            assert_eq!(field_step(field), Some(step), "{}: {field} belongs to another step", palette.key);
        }
        assert!(step_palettes(step).iter().any(|p| p.key == palette.key));
        let changes = preview_palette(&agent(), palette);
        assert!(!changes.is_empty(), "{}: changes nothing on a fresh draft", palette.key);
        assert!(changes.iter().all(|c| c.step == step), "{}: previews another step", palette.key);
    }
}

#[test]
fn every_full_palette_applies_cleanly_as_one_undo_step() {
    for palette in Palette::full(None) {
        let mut draft = named(agent(), "Theo", "@theo");
        let before = draft.profile().clone();
        assert!(apply_palette(&mut draft, palette), "{}", palette.key);
        assert!(preview_palette(&draft, palette).is_empty(), "{}: nothing left to change", palette.key);
        assert!(is_balanced_build(&draft.profile().config), "{}: over budget", palette.key);
        assert!(validate(&draft, &[]).iter().all(|i| i.severity != Severity::Error), "{}: {:?}", palette.key, validate(&draft, &[]));
        assert_eq!(draft_palette(&draft).map(|p| p.key.as_str()), Some(palette.key.as_str()));
        assert!(draft.undo());
        assert_eq!(draft.profile(), &before, "{}: undo restores the draft", palette.key);
    }
}

#[test]
fn the_tin_can_becomes_a_tin_can_and_stays_editable() {
    let mut draft = named(agent(), "Theo", "@theo");
    let tin_can = find("melancholy_tin_can");
    let changes = preview_palette(&draft, tin_can);
    let form = changes.iter().find(|c| c.field == "form").expect("form changes");
    assert_eq!(form.step, ForgeStep::Embodiment);
    assert!(changes.iter().any(|c| c.field == "avatar" && c.step == ForgeStep::Appearance));
    apply_palette(&mut draft, tin_can);
    assert_eq!(draft.profile().config.embodiment.form, BodyForm::TinCan);
    assert_eq!(draft.profile().name, "Theo", "names are the user's");
    assert!(draft.edit(DraftEdit::Trait(TraitValue::Mbti(crate::domain::MbtiType::Estj))));
    assert_eq!(draft.profile().config.mbti, crate::domain::MbtiType::Estj);
    assert_eq!(draft.profile().config.palette.as_deref(), Some("melancholy_tin_can"), "tweaks keep the badge");
}

#[test]
fn a_step_palette_keeps_the_rest_and_the_badge() {
    let mut draft = agent();
    apply_class(&mut draft, crate::domain::ArchetypeClass::StoicSage);
    let mbti = draft.profile().config.mbti;
    assert!(apply_palette(&mut draft, find("weekend_wanderer")));
    let cfg = &draft.profile().config;
    assert_eq!(cfg.mbti, mbti);
    assert_eq!(cfg.active_days.keys(), [Weekday::Saturday.key(), Weekday::Sunday.key()]);
    assert_eq!(cfg.palette.as_deref(), Some("stoic_sage"));
}

#[test]
fn engine_palettes_change_tools_and_output_but_not_the_model() {
    let mut draft = agent();
    apply_palette(&mut draft, find("illustrator"));
    let engine = &draft.profile().config.engine;
    let Some(structured) = engine.structured() else { panic!("structured output") };
    assert_eq!(structured.preset, SchemaPreset::Illustrated);
    assert_eq!(structured.schema, SchemaPreset::Illustrated.schema(), "the preset brings its schema");
    let changes = preview_palette(&draft, find("deep_reasoner"));
    let tools = changes.iter().find(|c| c.field == "engine.tools").expect("tools row");
    assert_eq!(tools.after, ChangeValue::Keys(vec!["forge.palette.engine_value.vault_search".into(), "forge.palette.engine_value.read_network".into()]));
    apply_palette(&mut draft, find("deep_reasoner"));
    apply_palette(&mut draft, find("fast_and_cheap"));
    let engine = &draft.profile().config.engine;
    assert_eq!(engine.output, OutputMode::FreeText);
    assert!(engine.tools.iter().all(|t| t.kind() != ToolKind::VaultSearch));
    assert_eq!(engine.text, Default::default(), "the model stays the global one");
}

#[test]
fn a_draft_remembers_its_palette_or_its_legacy_class() {
    let mut draft = agent();
    assert!(draft_palette(&draft).is_none());
    draft.transform(|p| p.config.archetype_class = Some(crate::domain::ArchetypeClass::MysticVisionary));
    assert_eq!(draft_palette(&draft).map(|p| p.key.as_str()), Some("mystic_visionary"));
    assert!(clear_palette(&mut draft));
    assert!(draft_palette(&draft).is_none());
}
