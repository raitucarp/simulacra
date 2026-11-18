use crate::domain::learning::bloom::BloomLevel;
use crate::domain::learning::tutor::reply::{checked_question, level_of, MAX_POINTS, MAX_QUESTION_CHARS};
use crate::domain::learning::tutor::{parse_summary, parse_turn_reply, ReplyProblem, Verdict};

#[test]
fn a_structured_reply_parses_with_its_assessment() {
    let text = r#"```json
{"assessment": {"level": "Analyze", "verdict": "SOLID", "right": ["names the cause", "names the cause", " "], "misconceptions": ["confuses licence with freedom"]},
 "question": "Which assumption does that rest on?"}
```"#;
    let reply = parse_turn_reply(text).expect("parses");
    let assessment = reply.assessment.expect("assessed");
    assert_eq!(assessment.reached, Some(BloomLevel::Analyse), "American spelling and case are tolerated");
    assert_eq!(assessment.verdict, Verdict::Solid);
    assert_eq!(assessment.right, vec!["names the cause".to_string()], "blank and repeated points are dropped");
    assert_eq!(assessment.misconceptions.len(), 1);
    assert_eq!(reply.question, "Which assumption does that rest on?");
}

#[test]
fn an_opening_reply_has_no_assessment() {
    let reply = parse_turn_reply("{\"question\": \"What do you already think it means?\"}").unwrap();
    assert!(reply.assessment.is_none());
}

#[test]
fn an_unknown_verdict_reads_as_partial_and_an_unknown_level_as_none() {
    let reply = parse_turn_reply(r#"{"assessment": {"level": "genius", "verdict": "great"}, "question": "Why?"}"#).unwrap();
    let assessment = reply.assessment.unwrap();
    assert_eq!(assessment.verdict, Verdict::Partial);
    assert_eq!(assessment.reached, None);
    assert_eq!(level_of(" Evaluate "), Some(BloomLevel::Evaluate));
}

#[test]
fn the_tutor_must_ask_not_lecture() {
    assert_eq!(parse_turn_reply("{\"question\": \"Freedom is acting from reasons.\"}"), Err(ReplyProblem::NoQuestion));
    let lecture = format!("{{\"question\": \"{}?\"}}", "x".repeat(MAX_QUESTION_CHARS + 1));
    assert_eq!(parse_turn_reply(&lecture), Err(ReplyProblem::Lecture));
    assert_eq!(parse_turn_reply("{\"assessment\": {}}"), Err(ReplyProblem::NoQuestion));
    assert_eq!(parse_turn_reply("   "), Err(ReplyProblem::Unreadable));
    assert_eq!(checked_question("“Why?”").as_deref(), Ok("Why?"), "quotes around it are removed");
    assert_eq!(checked_question("なぜですか？").as_deref(), Ok("なぜですか？"), "full-width marks count");
}

#[test]
fn a_plain_text_reply_falls_back_to_its_last_question() {
    let reply = parse_turn_reply("Good start.\nWhat would a counterexample look like?\n").unwrap();
    assert_eq!(reply.question, "What would a counterexample look like?");
    assert!(reply.assessment.is_none());
}

#[test]
fn a_summary_keeps_only_real_review_questions() {
    let text = r#"{"shown": ["defines freedom"], "missing": ["the role of causes"], "review": ["What makes an action free?", "Causes matter.", "Why do causes matter?"]}"#;
    let summary = parse_summary(text, Some(BloomLevel::Understand)).unwrap();
    assert_eq!(summary.review, vec!["What makes an action free?".to_string(), "Why do causes matter?".to_string()]);
    assert_eq!(summary.mastery, Some(BloomLevel::Understand));
    assert_eq!(parse_summary("no json here", None), Err(ReplyProblem::Unreadable));
    assert_eq!(parse_summary("{\"shown\": []}", None), Err(ReplyProblem::Unreadable));
    let many = format!("{{\"shown\": [{}]}}", (0..9).map(|i| format!("\"point {i}\"")).collect::<Vec<_>>().join(","));
    assert_eq!(parse_summary(&many, None).unwrap().shown.len(), MAX_POINTS);
}
