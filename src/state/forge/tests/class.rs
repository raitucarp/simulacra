use super::*;
use crate::domain::archetype::CLASS_PRESERVED_FIELDS;
use crate::domain::{ArchetypeClass, MbtiType};

#[test]
fn preview_lists_what_will_change_grouped_by_step() {
    let draft = named(agent(), "Theo", "@theo");
    let class = ArchetypeClass::ContrarianTrickster;
    let changes = preview_diff(&draft, class);
    assert!(!changes.is_empty());
    assert!(changes.windows(2).all(|pair| pair[0].step <= pair[1].step), "sorted by step");
    assert!(changes.iter().all(|c| !CLASS_PRESERVED_FIELDS.contains(&c.field.as_str())));
    let mbti = changes.iter().find(|c| c.field == "mbti").expect("mbti changes");
    assert_eq!(mbti.step, ForgeStep::Birthsign);
    assert_eq!(mbti.label_key, "forge.field.mbti");
    assert_eq!(mbti.before, ChangeValue::Trait(TraitValue::Mbti(MbtiType::Intj)));
    assert_eq!(mbti.after, ChangeValue::Trait(TraitValue::Mbti(MbtiType::Estp)));
    let extraversion = changes.iter().find(|c| c.field == "extraversion").expect("extraversion changes");
    assert_eq!(extraversion.after, ChangeValue::Number(i64::from(class.config().extraversion)));
    let bio = changes.iter().find(|c| c.field == "bio").expect("bio changes");
    assert_eq!(bio.after, ChangeValue::Text(class.config().bio));
}

#[test]
fn applying_a_class_is_one_undoable_step_that_keeps_identity() {
    let mut draft = named(agent(), "Theo", "@theo");
    draft.edit(DraftEdit::Text(TextField::PrimaryLanguage, "Indonesian".into()));
    let before = draft.profile().clone();
    let class = ArchetypeClass::StoicSage;
    assert!(apply_class(&mut draft, class));
    assert!(preview_diff(&draft, class).is_empty());
    assert_eq!(draft.profile().name, "Theo");
    assert_eq!(draft.profile().config.primary_language, "Indonesian");
    assert_eq!(draft.profile().config.mbti, class.config().mbti);
    assert!(!apply_class(&mut draft, class), "re-applying changes nothing");
    assert!(draft.undo());
    assert_eq!(draft.profile(), &before);
}

#[test]
fn every_class_applies_cleanly() {
    for class in ArchetypeClass::ALL {
        let mut draft = agent();
        apply_class(&mut draft, class);
        assert_eq!(draft.profile().config.description, class.config().description, "{class:?}");
        assert!(validate(&draft, &[]).iter().all(|i| i.field != "prompt_template"), "{class:?}");
    }
}
