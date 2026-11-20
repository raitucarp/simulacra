//! Fenced code blocks: spanning lines, language tags, unclosed fences, prose around them.

use super::*;

#[test]
fn a_block_spans_lines_and_keeps_its_language() {
    let source = "Look:\n```rust\nfn main() {\n    println!(\"**hi**\");\n}\n```\nDone.";
    assert_eq!(
        tokenize_post_body(source),
        vec![text("Look:"), code_block(Some("rust"), "fn main() {\n    println!(\"**hi**\");\n}"), text("Done.")]
    );
}

#[test]
fn missing_or_malformed_language_is_plain() {
    assert_eq!(tokenize_post_body("```\nplain\n```"), vec![code_block(None, "plain")]);
    assert_eq!(tokenize_post_body("```  \nplain\n```"), vec![code_block(None, "plain")]);
    assert_eq!(tokenize_post_body("```{weird}\nx\n```"), vec![code_block(None, "x")]);
    assert_eq!(tokenize_post_body("```Python3 extra words\nx\n```"), vec![code_block(Some("python3"), "x")]);
    assert_eq!(tokenize_post_body("```c++\nx\n```"), vec![code_block(Some("c++"), "x")]);
    assert_eq!(tokenize_post_body("```notalanguage\nx\n```"), vec![code_block(Some("notalanguage"), "x")]);
}

#[test]
fn an_unclosed_fence_is_literal_text() {
    assert_eq!(tokenize_post_body("```rust\nfn x() {}"), vec![text("```rust\nfn x() {}")]);
    assert_eq!(
        tokenize_post_body("```\na\n```\nb\n```"),
        vec![code_block(None, "a"), text("b\n```")]
    );
}

#[test]
fn closing_fence_needs_at_least_the_opening_run() {
    assert_eq!(tokenize_post_body("````\na\n```\nb\n````"), vec![code_block(None, "a\n```\nb")]);
    assert_eq!(tokenize_post_body("```\na\n`````"), vec![code_block(None, "a")]);
}

#[test]
fn backticks_on_one_line_are_not_a_block() {
    assert_eq!(tokenize_post_body("```code```"), vec![BodyToken::Code("code".to_string())]);
    assert_eq!(tokenize_post_body("say ```x``` ok"), vec![text("say "), BodyToken::Code("x".to_string()), text(" ok")]);
}

#[test]
fn blocks_handle_crlf_empty_bodies_and_blank_lines() {
    assert_eq!(tokenize_post_body("```js\r\nlet a;\r\n```\r\n"), vec![code_block(Some("js"), "let a;")]);
    assert_eq!(tokenize_post_body("```\n```"), vec![code_block(None, "")]);
    assert_eq!(
        tokenize_post_body("a\n\n```\n\nx\n\n```\n\nb"),
        vec![text("a\n"), code_block(None, "\nx\n"), text("\nb")]
    );
}

#[test]
fn an_escaped_fence_stays_text() {
    assert_eq!(tokenize_post_body("\\```\nx\n```"), vec![text("```\nx\n```")]);
}

#[test]
fn marks_around_blocks_still_parse() {
    assert_eq!(
        tokenize_post_body("**before**\n```\n*raw*\n```\n*after*"),
        vec![bold(vec![text("before")]), code_block(None, "*raw*"), italic(vec![text("after")])]
    );
}
