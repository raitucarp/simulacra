use super::*;
use crate::domain::engine::SchemaPreset;
use crate::domain::PostId;

fn reacting(preset: SchemaPreset) -> StructuredOutput {
    StructuredOutput { may_react: true, ..StructuredOutput::from_preset(preset) }
}

#[test]
fn reacting_is_off_by_default_and_changes_nothing_while_off() {
    let output = StructuredOutput::default();
    assert!(!output.may_react);
    assert_eq!(output.with_reactions_field(), output);
    let json = serde_json::to_value(&output).expect("serializes");
    assert!(json.get("may_react").is_none(), "an off setting is not written");
    assert!(json["mapping"].get(REACTIONS_FIELD).is_none(), "an unmapped field is not written");
    let old: StructuredOutput = serde_json::from_str(r#"{"preset":"post","schema":"{}"}"#).expect("old JSON loads");
    assert!(!old.may_react && old.mapping.reactions.is_empty());
}

#[test]
fn an_agent_that_may_react_is_offered_an_optional_reactions_field() {
    for preset in SchemaPreset::ALL {
        let output = reacting(preset).with_reactions_field();
        assert_eq!(output.mapping.reactions, REACTIONS_FIELD);
        let schema = output.schema_value().expect("still an object");
        let property = &schema["properties"][REACTIONS_FIELD];
        assert_eq!(property["type"], "array", "{preset:?}");
        assert_eq!(property["maxItems"], MAX_REACTIONS_PER_ANSWER);
        let required = schema["required"].as_array().cloned().unwrap_or_default();
        assert!(!required.iter().any(|field| field == REACTIONS_FIELD), "{preset:?}: reactions stay optional");
        assert_eq!(output.with_reactions_field(), output, "applying twice changes nothing");
    }
    let both = StructuredOutput { may_tint: true, ..reacting(SchemaPreset::Post) }.with_tint_field().with_reactions_field();
    let schema = both.schema_value().expect("object");
    assert!(schema["properties"].get(crate::domain::engine::TINT_FIELD).is_some() && schema["properties"].get(REACTIONS_FIELD).is_some());
}

#[test]
fn answers_keep_offered_emoji_once_and_at_most_three() {
    let items: Vec<String> = ["\u{1F525}", " \u{1F525} ", "lol", "\u{1F44F}", "\u{1F914}", "\u{2764}\u{FE0F}"].map(String::from).to_vec();
    assert_eq!(parse_reactions(&items), vec!["\u{1F525}", "\u{1F44F}", "\u{1F914}"]);
    let newer = crate::domain::emoji::EMOJI.iter().find(|emoji| !crate::domain::emoji::is_pickable(emoji.glyph)).expect("a newer emoji");
    assert!(parse_reactions(&[newer.glyph.to_string()]).is_empty(), "only emoji the picker offers");
}

#[test]
fn each_agent_reacts_at_most_its_hourly_limit() {
    let now = crate::time::utc(2026, 11, 20, 12, 0, 0);
    let row = |agent: &str, minutes_ago: i64| Reaction {
        post_id: PostId::new(format!("p{minutes_ago}")),
        profile_id: ProfileId::new(agent),
        emoji: "\u{1F525}".into(),
        count: 1,
        updated_at: now - SignedDuration::from_mins(minutes_ago),
    };
    let ada = ProfileId::new("ada");
    assert_eq!(reactions_left(&ada, std::iter::empty(), now), AGENT_REACTIONS_PER_HOUR);
    let rows = [row("ada", 5), row("ada", 30), row("ada", 59), row("ada", 61), row("ada", 300), row("bo", 1), row("bo", 2)];
    assert_eq!(reactions_left(&ada, rows.iter(), now), AGENT_REACTIONS_PER_HOUR - 3, "only the last hour counts, and only Ada's");
    let busy: Vec<Reaction> = (0..10).map(|minute| row("ada", minute)).collect();
    assert_eq!(reactions_left(&ada, busy.iter(), now), 0);
}
