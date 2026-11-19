use crate::domain::preferences::*;
use std::collections::HashSet;

#[test]
fn field_ids_are_unique_and_cover_every_category() {
    let fields = SettingField::all();
    let ids: HashSet<String> = fields.iter().map(|field| field.id()).collect();
    assert_eq!(ids.len(), fields.len(), "duplicate field ids");
    for category in PreferenceCategory::ALL {
        assert!(fields.iter().any(|field| field.category() == *category), "{} has no field", category.key());
    }
}

#[test]
fn defaults_are_unmodified_and_every_field_round_trips() {
    let prefs = Preferences::default();
    for field in SettingField::all() {
        assert!(!field.is_modified(&prefs), "{} modified by default", field.id());
        let value = field.get(&prefs);
        assert_eq!(field.get(&field.set(prefs.clone(), &value)), value, "{} does not round trip", field.id());
    }
}

/// A value different from the default for every editable kind (`None` for locked fields).
fn other_value(field: SettingField, prefs: &Preferences) -> Option<SettingValue> {
    match (field.kind(), field.get(prefs)) {
        (FieldKind::Toggle, SettingValue::Toggle(on)) => Some(SettingValue::Toggle(!on)),
        (FieldKind::Choice(keys), SettingValue::Choice(key)) => keys.iter().find(|k| **k != key).map(|k| SettingValue::Choice(k)),
        (FieldKind::Number(spec), SettingValue::Number(n)) => {
            let up = spec.bounds.stepped(n, 1);
            Some(SettingValue::Number(if up != n { up } else { spec.bounds.stepped(n, -1) }))
        }
        (FieldKind::Text | FieldKind::Persona, _) => Some(SettingValue::Text("custom".into())),
        _ => None,
    }
}

#[test]
fn editing_marks_a_field_modified_and_reset_restores_it() {
    let defaults = Preferences::default();
    for field in SettingField::all() {
        let Some(value) = other_value(field, &defaults) else { continue };
        let edited = field.set(defaults.clone(), &value);
        assert!(field.is_modified(&edited), "{} not modified after edit", field.id());
        let others = SettingField::all().into_iter().filter(|other| *other != field && other.is_modified(&edited));
        assert_eq!(others.count(), 0, "editing {} changed another field", field.id());
        assert_eq!(field.reset(edited), defaults, "{} reset", field.id());
    }
}

#[test]
fn set_clamps_and_ignores_mismatched_values() {
    let prefs = Preferences::default();
    let field = SettingField::Agents(AgentField::MaxConcurrent);
    assert_eq!(field.get(&field.set(prefs.clone(), &SettingValue::Number(99))), SettingValue::Number(4));
    assert_eq!(field.get(&field.set(prefs.clone(), &SettingValue::Number(-5))), SettingValue::Number(1));
    assert_eq!(field.set(prefs.clone(), &SettingValue::Toggle(true)), prefs, "wrong kind is ignored");
    let language = SettingField::General(GeneralField::Language);
    assert_eq!(language.set(prefs.clone(), &SettingValue::Choice("klingon")), prefs, "unknown key is ignored");
    let telemetry = SettingField::Privacy(PrivacyField::Telemetry);
    assert_eq!(telemetry.kind(), FieldKind::Locked);
    assert!(!telemetry.set(prefs, &SettingValue::Toggle(true)).privacy.telemetry);
}

#[test]
fn language_choice_uses_locale_codes() {
    let field = SettingField::General(GeneralField::Language);
    let FieldKind::Choice(keys) = field.kind() else { panic!("language is a choice") };
    assert_eq!(keys.len(), crate::i18n::Language::ALL.len());
    let japanese = field.set(Preferences::default(), &SettingValue::Choice("ja"));
    assert_eq!(japanese.general.language, crate::i18n::Language::Japanese);
}
