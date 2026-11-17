use super::*;
use crate::domain::portrait::{AvatarMode, EmblemSpec, PhotoCrop, PortraitSpec};

/// Forms per kind and seeds sampled by the generator tests.
const FORMS_PER_KIND: usize = 8;
const SAMPLE_SEEDS: u64 = 24;
/// Layers every kind's catalog stacks.
const LAYERS: [FormLayer; 8] = [
    FormLayer::Back, FormLayer::Aura, FormLayer::Body, FormLayer::Finish,
    FormLayer::Markings, FormLayer::Mouth, FormLayer::Eyes, FormLayer::Accessory,
];

#[test]
fn every_kind_has_a_complete_catalog() {
    for kind in FormKind::ALL {
        let catalog = FormCatalog::of(kind);
        assert_eq!(catalog.variants(FormLayer::Body).len(), FORMS_PER_KIND, "{kind:?}");
        assert_eq!(catalog.stack.len(), LAYERS.len(), "{kind:?}: stack");
        for layer in LAYERS {
            assert!(catalog.stack.contains(&layer), "{kind:?}: {layer:?} not stacked");
            assert!(!catalog.variants(layer).is_empty(), "{kind:?}: {layer:?} has no variants");
        }
        assert!(!catalog.primary.is_empty() && !catalog.secondary.is_empty(), "{kind:?}: swatches");
        assert!(catalog.layers[&FormLayer::Finish].clip && catalog.layers[&FormLayer::Eyes].anchor, "{kind:?}: flags");
        assert_eq!(FormKind::from_key(kind.key()), Some(kind));
    }
}

#[test]
fn kind_and_form_keys_match_the_embodiment_keys() {
    let object = FormCatalog::of(FormKind::Object).variants(FormLayer::Body);
    assert!(object.iter().any(|f| f == "tin_can") && object.iter().any(|f| f == "teacup"));
    assert!(FormCatalog::of(FormKind::Plant).variants(FormLayer::Body).iter().any(|f| f == "oak_tree"));
    assert!(FormCatalog::of(FormKind::Element).variants(FormLayer::Body).iter().any(|f| f == "water"));
    let keys: Vec<&str> = FormKind::ALL.iter().map(|k| k.key()).collect();
    assert_eq!(keys, ["creature", "plant", "object", "element", "machine", "place", "abstraction"]);
}

#[test]
fn form_spec_serde_round_trips_and_tolerates_partial_json() {
    let spec = FormSpec::random(FormKind::Plant, "oak_tree", 5);
    let json = serde_json::to_string(&spec).unwrap();
    assert_eq!(serde_json::from_str::<FormSpec>(&json).unwrap(), spec);
    let partial: FormSpec = serde_json::from_str(r#"{"kind":"element","parts":{"body":"water"}}"#).unwrap();
    assert_eq!((partial.kind, partial.form(), partial.part(FormLayer::Eyes)), (FormKind::Element, "water", "dot"));
    let unknown: FormSpec = serde_json::from_str(r#"{"kind":"fungus","parts":{"body":"spore","eyes":"x"}}"#).unwrap();
    assert_eq!((unknown.kind, unknown.form(), unknown.part(FormLayer::Eyes)), (FormKind::Object, "tin_can", "dot"));
    assert_eq!(unknown.hex(FormColor::Primary), FormCatalog::of(FormKind::Object).primary[0].hex);
}

#[test]
fn avatar_modes_old_and_new_load() {
    let old = [
        r#"{"mode":"pcg","seed":7,"hair":"bob"}"#,
        r#"{"mode":"emblem","shape":"star","primary":"accent","secondary":"info","glyph":{},"wing":3}"#,
        r#"{"mode":"photo","path":"avatars/p.png","crop":{"zoom":2000,"offset_x":10,"offset_y":-10}}"#,
    ];
    for json in old {
        let mode: AvatarMode = serde_json::from_str(json).unwrap_or_else(|e| panic!("{json}: {e}"));
        assert!(!matches!(mode, AvatarMode::Form(_)), "{json}");
    }
    let modes = [
        AvatarMode::Form(FormSpec::random_any(FormKind::Machine, 3)),
        AvatarMode::Pcg(PortraitSpec::random(3)),
        AvatarMode::Emblem(EmblemSpec::default()),
        AvatarMode::Photo { path: "avatars/q.webp".into(), crop: PhotoCrop::default() },
    ];
    for mode in modes {
        let json = serde_json::to_string(&mode).unwrap();
        assert_eq!(serde_json::from_str::<AvatarMode>(&json).unwrap(), mode, "{json}");
    }
    let form: AvatarMode = serde_json::from_str(r#"{"mode":"form","kind":"creature"}"#).unwrap();
    assert!(matches!(form, AvatarMode::Form(spec) if spec.kind == FormKind::Creature && spec.form() == "cat"));
}

#[test]
fn random_forms_are_deterministic_valid_and_follow_hints() {
    for kind in FormKind::ALL {
        let catalog = FormCatalog::of(kind);
        for seed in 0..SAMPLE_SEEDS {
            let spec = FormSpec::random_any(kind, seed);
            assert_eq!(spec, FormSpec::random_any(kind, seed));
            for layer in LAYERS {
                let chosen = spec.parts.get(layer.key()).map(String::as_str).unwrap_or(spec.part(layer));
                assert_eq!(chosen, spec.part(layer), "{kind:?}/{seed}: {layer:?} rolled an unknown variant");
            }
            let hints = catalog.hints(spec.form());
            for (key, preferred) in &hints.prefer {
                let chosen = match key.as_str() {
                    "primary" => spec.color_key(FormColor::Primary).to_string(),
                    "secondary" => spec.color_key(FormColor::Secondary).to_string(),
                    layer => spec.parts.get(layer).cloned().unwrap_or_default(),
                };
                assert!(preferred.contains(&chosen), "{kind:?}/{}: {key} {chosen} not preferred", spec.form());
            }
        }
    }
}

#[test]
fn rerolls_and_slot_indices_change_only_their_slot() {
    let spec = FormSpec::random(FormKind::Object, "tin_can", 1);
    for slot in FormSlot::ALL {
        let next = spec.rerolled(slot, 42);
        let options = spec.slot_keys(slot).len();
        assert!(options < 2 || next.slot_index(slot) != spec.slot_index(slot), "{slot:?} did not change");
        for other in FormSlot::ALL.into_iter().filter(|s| *s != slot) {
            assert_eq!(next.slot_index(other), spec.slot_index(other), "{slot:?} changed {other:?}");
        }
        let last = options - 1;
        assert_eq!(spec.clone().with_slot_index(slot, last).slot_index(slot), last, "{slot:?}");
        assert_eq!(spec.clone().with_slot_index(slot, options), spec, "{slot:?}: out of range is a no-op");
    }
}

#[test]
fn layer_paths_follow_the_stack_and_skip_none() {
    let spec = FormSpec::new(FormKind::Object, "tin_can").with_part(FormLayer::Eyes, NONE_VARIANT);
    let paths = spec.layer_paths();
    let layers: Vec<FormLayer> = paths.iter().map(|(layer, _)| *layer).collect();
    let catalog = spec.catalog();
    let expected: Vec<FormLayer> = catalog.stack.iter().copied().filter(|l| spec.part(*l) != NONE_VARIANT).collect();
    assert_eq!(layers, expected);
    assert!(!layers.contains(&FormLayer::Eyes) && !layers.contains(&FormLayer::Finish));
    assert!(paths.contains(&(FormLayer::Body, "illustrations/forms/object/body/tin_can.svg".into())));
    assert!(paths.contains(&(FormLayer::Back, "illustrations/forms/shared/back/plain.svg".into())));
}

#[test]
fn face_anchor_matrix_moves_the_default_face() {
    assert_eq!(FaceAnchor::default().matrix(), [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    let [scale, _, _, _, tx, ty] = FaceAnchor(4.0, -10.0, 0.5).matrix();
    let (cx, cy) = FACE_CENTER;
    assert_eq!((scale * cx + tx, scale * cy + ty), (cx + 4.0, cy - 10.0), "the face centre lands on the anchor");
    let chair = FormSpec::new(FormKind::Object, "chair").anchor();
    assert_ne!(chair, FaceAnchor::default());
}

#[test]
fn form_emblems_are_stable_and_differ_by_kind() {
    let tin = EmblemSpec::from_form(FormKind::Object, "tin_can");
    assert_eq!(tin, EmblemSpec::for_form(&FormSpec::new(FormKind::Object, "tin_can")));
    assert!((1..=9).contains(&tin.wing));
    assert_ne!(tin.shape, EmblemSpec::from_form(FormKind::Plant, "tin_can").shape);
    assert_ne!(EmblemSpec::from_form(FormKind::Element, "water"), EmblemSpec::from_form(FormKind::Element, "fire"));
}

#[test]
fn labels_follow_the_i18n_scheme() {
    assert_eq!(FormKind::Plant.label_key(), "forms.kind_plant");
    assert_eq!(FormLayer::Body.variant_label_key("oak_tree"), "forms.form_oak_tree");
    assert_eq!(FormLayer::Finish.variant_label_key("rust"), "forms.v_rust");
    assert_eq!(FormSlot::Color(FormColor::Secondary).label_key(), "forms.secondary");
    assert_eq!(FormSlot::Layer(FormLayer::Aura).label_key(), "forms.layer_aura");
}
