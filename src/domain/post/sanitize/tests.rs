use super::*;
use crate::domain::post::{tokenize_post_body, BodyToken};

#[test]
fn supported_marks_pass_through_unchanged() {
    let text = "**bold** *italic* __under__ `code` ||secret|| #Tag [[Note]]";
    assert_eq!(sanitize_agent_markdown(text), text);
}

#[test]
fn headings_become_their_text_but_hashtags_stay() {
    assert_eq!(sanitize_agent_markdown("# Title\nbody"), "Title\nbody");
    assert_eq!(sanitize_agent_markdown("### Closed heading ###"), "Closed heading");
    assert_eq!(sanitize_agent_markdown("#Tag at the start"), "#Tag at the start");
    assert_eq!(sanitize_agent_markdown("Setext title\n====\nbody"), "Setext title\nbody");
    assert!(!tokenize_post_body(&sanitize_agent_markdown("## A **bold** claim")).is_empty());
    assert_eq!(sanitize_agent_markdown("## A **bold** claim"), "A **bold** claim");
}

#[test]
fn rules_quotes_and_bullets_are_reduced() {
    assert_eq!(sanitize_agent_markdown("one\n\n---\n\ntwo"), "one\n\ntwo");
    assert_eq!(sanitize_agent_markdown("> quoted\n>> nested"), "quoted\nnested");
    assert_eq!(sanitize_agent_markdown("* first\n+ second\n- third\n  * nested"), "- first\n- second\n- third\n  - nested");
    assert_eq!(sanitize_agent_markdown("1. keep numbers"), "1. keep numbers");
}

#[test]
fn links_images_strikethrough_and_breaks_become_plain_text() {
    assert_eq!(sanitize_agent_markdown("see [the docs](https://example.org/x)"), "see the docs (https://example.org/x)");
    assert_eq!(sanitize_agent_markdown("[https://a.io](https://a.io)"), "https://a.io");
    assert_eq!(sanitize_agent_markdown("![a cat](cat.png) purrs"), "a cat purrs");
    assert_eq!(sanitize_agent_markdown("<https://a.io> now"), "https://a.io now");
    assert_eq!(sanitize_agent_markdown("~~old~~ new"), "old new");
    assert_eq!(sanitize_agent_markdown("line<br>next<BR/>last"), "line\nnext\nlast");
}

#[test]
fn tables_lose_their_separator_row() {
    assert_eq!(sanitize_agent_markdown("| a | b |\n|---|:-:|\n| 1 | 2 |"), "| a | b |\n| 1 | 2 |");
}

#[test]
fn code_is_never_touched() {
    let fenced = "```rust\n# not a heading\n* not a bullet\n\n\n[x](y)\n```";
    assert_eq!(sanitize_agent_markdown(fenced), fenced);
    assert_eq!(sanitize_agent_markdown("run `[x](y) ~~z~~` now"), "run `[x](y) ~~z~~` now");
    assert_eq!(sanitize_agent_markdown("~~~py\nx = 1\n~~~"), "```py\nx = 1\n```");
    let tokens = tokenize_post_body(&sanitize_agent_markdown("~~~py\nx = 1\n~~~"));
    assert_eq!(tokens, vec![BodyToken::CodeBlock { lang: Some("py".into()), code: "x = 1".into() }]);
}

#[test]
fn blank_lines_collapse_and_the_answer_is_trimmed() {
    assert_eq!(sanitize_agent_markdown("\n\none\n\n\n\ntwo\n\n"), "one\n\ntwo");
    assert_eq!(sanitize_agent_markdown("one\r\ntwo"), "one\ntwo");
    assert_eq!(sanitize_agent_markdown(""), "");
}

#[test]
fn escapes_and_unclosed_constructs_survive() {
    assert_eq!(sanitize_agent_markdown(r"a \*literal\* star"), r"a \*literal\* star");
    assert_eq!(sanitize_agent_markdown("[not a link] and `open"), "[not a link] and `open");
    assert_eq!(sanitize_agent_markdown("caf\u{e9} \u{1F600} [x](y)"), "caf\u{e9} \u{1F600} x (y)");
}
