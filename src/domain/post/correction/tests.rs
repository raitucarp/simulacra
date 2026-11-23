use super::*;

#[test]
fn a_reason_is_needed_and_kept_tidy() {
    assert_eq!(clean_reason("  The study was withdrawn.  \n\n  See the note. "), Some("The study was withdrawn.\nSee the note.".to_string()));
    assert_eq!(clean_reason("   "), None);
    assert_eq!(clean_reason(&"x".repeat(MAX_REASON_CHARS + 1)), None);
}

#[test]
fn agents_read_the_mark_and_it_round_trips() {
    let at: Timestamp = "2026-11-02T10:00:00Z".parse().expect("time");
    let retraction = Correction { kind: CorrectionKind::Retracted, reason: "The source was a parody.".into(), at, told: Vec::new() };
    assert_eq!(retraction.agent_note(), "[Retracted by its author: The source was a parody.]");
    let correction = Correction { kind: CorrectionKind::Corrected, reason: "It is 12%, not 21%.".into(), told: vec![ProfileId::new("agent_a")], ..retraction.clone() };
    assert!(correction.agent_note().starts_with("[Corrected by its author:"));
    let json = serde_json::to_string(&correction).expect("json");
    assert!(json.contains("\"kind\":\"corrected\"") && json.contains("agent_a"));
    assert_eq!(serde_json::from_str::<Correction>(&json).expect("back"), correction);
    assert!(!serde_json::to_string(&retraction).expect("json").contains("told"), "nobody told, nothing stored");
}
