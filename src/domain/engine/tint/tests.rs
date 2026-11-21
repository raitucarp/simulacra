use super::*;
use crate::domain::engine::{OutputMapping, SchemaPreset};

fn tinting(preset: SchemaPreset) -> StructuredOutput {
    StructuredOutput { may_tint: true, ..StructuredOutput::from_preset(preset) }
}

#[test]
fn tinting_is_off_by_default_and_changes_nothing_while_off() {
    let output = StructuredOutput::default();
    assert!(!output.may_tint);
    assert_eq!(output.with_tint_field(), output);
    let json = serde_json::to_value(&output).expect("serializes");
    assert!(json.get("may_tint").is_none(), "an off setting is not written");
    let old: StructuredOutput = serde_json::from_str(r#"{"preset":"post","schema":"{}"}"#).expect("old JSON loads");
    assert!(!old.may_tint && old.mapping.tint.is_empty());
}

#[test]
fn an_agent_that_may_tint_is_offered_an_optional_tint_field() {
    for preset in SchemaPreset::ALL {
        let output = tinting(preset).with_tint_field();
        assert_eq!(output.mapping.tint, TINT_FIELD);
        let schema = output.schema_value().expect("still an object");
        let property = &schema["properties"][TINT_FIELD];
        let keys: Vec<&str> = property["enum"].as_array().expect("an enum").iter().filter_map(|key| key.as_str()).collect();
        assert_eq!(keys, Tint::ALL.iter().map(|tint| tint.key()).collect::<Vec<_>>(), "{preset:?}");
        let required = schema["required"].as_array().cloned().unwrap_or_default();
        assert!(!required.iter().any(|field| field == TINT_FIELD), "{preset:?}: the tint stays optional");
        assert_eq!(output.with_tint_field(), output, "applying twice changes nothing");
    }
}

#[test]
fn a_hand_mapped_tint_is_kept_and_the_schema_left_alone() {
    let mut output = tinting(SchemaPreset::Post);
    output.mapping = OutputMapping { tint: "meta.colour".into(), ..output.mapping };
    let requested = output.with_tint_field();
    assert_eq!(requested.mapping.tint, "meta.colour");
    assert_eq!(requested.schema, output.schema);
    let broken = StructuredOutput { schema: "not json".into(), ..tinting(SchemaPreset::Post) };
    assert_eq!(broken.with_tint_field().schema, "not json");
}

#[test]
fn only_known_tints_parse() {
    assert_eq!(parse_tint(" Amber "), Some(Tint::Amber));
    assert_eq!(parse_tint("SLATE"), Some(Tint::Slate));
    assert_eq!(parse_tint("teal"), None);
    assert_eq!(parse_tint(""), None);
}
