use super::*;
use crate::domain::engine::{SchemaPreset, StructuredOutput};

fn feeling_output(may_feel: bool) -> StructuredOutput {
    StructuredOutput { may_feel, ..StructuredOutput::from_preset(SchemaPreset::default()) }
}

#[test]
fn the_field_is_offered_only_while_the_agent_may_post_feelings() {
    let off = feeling_output(false).with_feeling_field();
    assert_eq!(off, feeling_output(false), "unchanged while off");
    assert!(off.mapping.feeling.is_empty());
    let on = feeling_output(true).with_feeling_field();
    assert_eq!(on.mapping.feeling, FEELING_FIELD);
    let schema = on.schema_value().expect("schema");
    assert_eq!(schema["properties"][FEELING_FIELD]["type"], "object");
    assert_eq!(schema["properties"][FEELING_FIELD]["properties"]["intensity"]["maximum"], 5);
}

#[test]
fn an_answer_names_a_feeling_with_known_emoji_on_the_scale() {
    let feeling = parse_feeling(&json!({ "emojis": ["😌", "not an emoji", "🌿"], "intensity": 9, "note": "  quiet morning  " })).expect("a feeling");
    assert_eq!(feeling, PostKind::feeling(&["😌".to_string(), "🌿".to_string()], Intensity::MAX, "quiet morning").expect("valid"));
    let middle = parse_feeling(&json!({ "emojis": ["😢"] })).expect("a feeling");
    assert!(matches!(middle, PostKind::Feeling { intensity, .. } if intensity == Intensity::default()));
    assert_eq!(parse_feeling(&json!({ "emojis": ["words"] })), None);
    assert_eq!(parse_feeling(&json!({ "intensity": 2 })), None);
    assert_eq!(parse_feeling(&json!("😌")), None);
}
