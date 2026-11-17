use super::*;

#[test]
fn every_kind_but_human_has_eight_forms_and_human_has_the_person() {
    for kind in BodyKind::ALL {
        let expected = if kind == BodyKind::Human { 1 } else { 8 };
        assert_eq!(kind.forms().len(), expected, "{kind:?}");
        assert_eq!(kind.default_form().kind(), kind);
    }
    assert_eq!(BodyForm::ALL.len(), 57);
}

#[test]
fn the_default_body_is_a_natural_coherent_human() {
    let body = Embodiment::default();
    assert!(body.is_human() && body.is_coherent());
    assert_eq!(body, Embodiment::natural(BodyForm::Person));
}

#[test]
fn a_tin_can_is_a_carried_metal_object() {
    let can = Embodiment::natural(BodyForm::TinCan);
    assert_eq!(can.body_kind, BodyKind::Object);
    assert_eq!((can.material, can.mobility), (Material::Metal, Mobility::Carried));
    assert!(!can.is_human());
    assert_eq!(BodyForm::TinCan.noun(), "a tin can");
}

#[test]
fn kind_and_form_stay_coherent_whichever_is_set() {
    let mut body = Embodiment::default();
    body.set_kind(BodyKind::Plant);
    assert_eq!(body.form, BodyForm::OakTree);
    body.set_form(BodyForm::Whale);
    assert_eq!(body.body_kind, BodyKind::Creature);
    body.set_kind(BodyKind::Creature);
    assert_eq!(body.form, BodyForm::Whale, "a matching form is kept");
    assert!(body.is_coherent());
}

#[test]
fn missing_keys_read_as_the_default_body_of_the_form() {
    let body: Embodiment = serde_json::from_str(r#"{"form":"Cat"}"#).expect("partial body");
    assert_eq!(body.form, BodyForm::Cat);
    assert_eq!((body.body_kind, body.material), (BodyKind::Creature, Material::Flesh));
}

#[test]
fn a_stored_kind_and_form_that_disagree_are_reconciled() {
    let kind_only: Embodiment = serde_json::from_str(r#"{"body_kind":"Machine"}"#).expect("kind only");
    assert_eq!(kind_only.form, BodyForm::Robot);
    let clash: Embodiment = serde_json::from_str(r#"{"body_kind":"Plant","form":"Owl"}"#).expect("clash");
    assert_eq!(clash.body_kind, BodyKind::Creature);
    let round: Embodiment = serde_json::from_value(serde_json::to_value(Embodiment::natural(BodyForm::Ice)).unwrap()).unwrap();
    assert_eq!(round, Embodiment::natural(BodyForm::Ice));
}

#[test]
fn every_form_has_its_own_nature_row() {
    for form in BodyForm::ALL {
        assert!(super::nature::NATURES.iter().filter(|(f, _)| *f == form).count() == 1, "{form:?}");
    }
}
