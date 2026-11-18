use crate::domain::review::ai::{draft_prompt, parse_draft_reply, DraftParseError, MAX_DRAFT_CARDS};
use crate::domain::review::DraftKind;

#[test]
fn the_prompt_carries_the_text_language_and_both_card_shapes() {
    let prompt = draft_prompt("  Conatus is the striving to persist.  ", "German");
    assert!(prompt.contains("Conatus is the striving to persist."));
    assert!(prompt.contains("write in German"));
    assert!(prompt.contains(r#""type":"qa""#) && prompt.contains("{{c1::hidden words}}"));
}

#[test]
fn replies_parse_through_fences_and_skip_bad_cards() {
    let reply = r#"Here you go:
```json
{"cards":[
  {"type":"qa","question":"What is conatus?","answer":"The striving to persist"},
  {"type":"cloze","text":"Spinoza calls it {{c1::conatus}}."},
  {"type":"cloze","text":"no deletion here"},
  {"type":"poem","text":"?"},
  {"type":"qa","question":" ","answer":"x"}
]}
```"#;
    let drafts = parse_draft_reply(reply).unwrap();
    assert_eq!(drafts.len(), 2);
    assert_eq!(drafts[0], DraftKind::Basic { front: "What is conatus?".into(), back: "The striving to persist".into() });
    assert!(matches!(&drafts[1], DraftKind::Cloze { text } if text.contains("{{c1::conatus}}")));
}

#[test]
fn unusable_replies_say_why() {
    assert_eq!(parse_draft_reply("sorry, no"), Err(DraftParseError::NoJsonObject));
    assert!(matches!(parse_draft_reply("{not json}"), Err(DraftParseError::Malformed(_))));
    assert_eq!(parse_draft_reply(r#"{"cards":[]}"#), Err(DraftParseError::NoCards));
    let many = format!(r#"{{"cards":[{}]}}"#, vec![r#"{"type":"qa","question":"q","answer":"a"}"#; 9].join(","));
    assert_eq!(parse_draft_reply(&many).unwrap().len(), MAX_DRAFT_CARDS);
}
