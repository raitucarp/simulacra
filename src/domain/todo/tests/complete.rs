//! D11: completing `^`, `^^`, `&` and `@` on to-do lines.

use super::super::complete::{complete, suggestions, typing_at, CompletionKind, CompletionSources, Suggestion};
use jiff::civil::{date, Date};

/// Sunday 22 November 2026.
fn today() -> Date {
    date(2026, 11, 22)
}

fn sources() -> CompletionSources {
    CompletionSources { contexts: vec!["home".into(), "Hardware".into(), "phone".into()], agents: vec!["ada".into(), "adler".into(), "basho".into()] }
}

fn offered(text: &str) -> Vec<String> {
    let typing = typing_at(text, text.len()).expect("typing a token");
    suggestions(&typing, &sources(), today()).into_iter().map(|s| s.token).collect()
}

#[test]
fn the_word_at_the_cursor_is_read_only_on_to_do_lines() {
    let typing = typing_at("[ ] call mum ^fr", 16).unwrap();
    assert_eq!((typing.range, typing.kind, typing.typed.as_str()), (13..16, CompletionKind::Due, "fr"));
    assert_eq!(typing_at("note\n- [ ] x ^^we", 17).unwrap().kind, CompletionKind::Repeat);
    assert_eq!(typing_at("[ ] tidy &", 10).unwrap().kind, CompletionKind::Context);
    assert!(typing_at("just a post ^fri", 16).is_none(), "not a to-do line");
    assert!(typing_at("[ ] plain words", 15).is_none(), "no sigil");
    assert!(typing_at("[ ] ", 4).is_none());
    let mid = "[ ] a @ad\n[ ] b";
    assert_eq!(typing_at(mid, 9).unwrap().typed, "ad", "the line the cursor is on");
}

#[test]
fn due_phrases_come_with_their_day() {
    let typing = typing_at("[ ] x ^t", 8).unwrap();
    let found = suggestions(&typing, &sources(), today());
    assert_eq!(found[0], Suggestion { token: "^today".into(), day: Some(today()) });
    assert_eq!(found[1], Suggestion { token: "^tomorrow".into(), day: Some(date(2026, 11, 23)) });
    assert_eq!(found.iter().map(|s| s.token.as_str()).collect::<Vec<_>>(), vec!["^today", "^tomorrow", "^tue", "^thu"]);
    assert_eq!(offered("[ ] x ^next"), vec!["^next week", "^next month"]);
    assert_eq!(offered("[ ] x ^^w"), vec!["^^weekdays", "^^weekly"]);
}

#[test]
fn contexts_and_agents_match_ignoring_case_and_skip_the_word_typed() {
    assert_eq!(offered("[ ] x &h"), vec!["&home", "&Hardware"]);
    assert_eq!(offered("[ ] x &HOME"), Vec::<String>::new(), "already typed in full");
    assert_eq!(offered("[ ] x @ad"), vec!["@ada", "@adler"]);
    assert_eq!(offered("[ ] x @"), vec!["@ada", "@adler", "@basho"]);
}

#[test]
fn completing_replaces_the_word_and_leaves_one_space() {
    let text = "[ ] call mum ^fr !!";
    let typing = typing_at(text, 16).unwrap();
    assert_eq!(complete(text, &typing, "^fri"), ("[ ] call mum ^fri !!".to_string(), 18));
    let end = "[ ] tidy &h";
    let typing = typing_at(end, end.len()).unwrap();
    assert_eq!(complete(end, &typing, "&home"), ("[ ] tidy &home ".to_string(), 15));
}
