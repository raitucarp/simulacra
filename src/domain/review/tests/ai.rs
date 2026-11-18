use crate::domain::review::ai::{parse_suggestion, suggest_prompt, SuggestError, MAX_SUGGESTION_CHARS};
use crate::domain::review::AnchorSide;

#[test]
fn the_prompt_asks_for_the_face_the_learner_writes() {
    let question = suggest_prompt("  Conatus is the striving to persist.  ", AnchorSide::Back, "German");
    assert!(question.contains("Conatus is the striving to persist.") && question.contains("write in German"));
    assert!(question.contains("question"));
    let answer = suggest_prompt("Conatus?", AnchorSide::Front, "English");
    assert!(answer.contains("short answer") && answer.contains(r#"{"text":"…"}"#));
}

#[test]
fn replies_parse_through_fences_and_are_capped() {
    let reply = "Sure:\n```json\n{\"text\":\"  What does every thing strive for?  \"}\n```";
    assert_eq!(parse_suggestion(reply).unwrap(), "What does every thing strive for?");
    let long = format!(r#"{{"text":"{}"}}"#, "x".repeat(MAX_SUGGESTION_CHARS * 2));
    assert_eq!(parse_suggestion(&long).unwrap().chars().count(), MAX_SUGGESTION_CHARS);
}

#[test]
fn unusable_replies_say_why() {
    assert_eq!(parse_suggestion("sorry, no"), Err(SuggestError::NoJsonObject));
    assert!(matches!(parse_suggestion("{not json}"), Err(SuggestError::Malformed(_))));
    assert_eq!(parse_suggestion(r#"{"text":"  "}"#), Err(SuggestError::Empty));
}
