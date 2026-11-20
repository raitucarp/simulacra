//! Inline marks: each mark, nesting, unclosed and crossing marks, escapes, bullets, code.

use super::*;

fn parse(source: &str) -> Vec<BodyToken> {
    tokenize_post_body(source)
}

#[test]
fn every_mark_parses() {
    assert_eq!(parse("a **b** c"), vec![text("a "), bold(vec![text("b")]), text(" c")]);
    assert_eq!(parse("*it*"), vec![italic(vec![text("it")])]);
    assert_eq!(parse("__u__"), vec![BodyToken::Underline(vec![text("u")])]);
    assert_eq!(parse("||secret||!"), vec![BodyToken::Spoiler(vec![text("secret")]), text("!")]);
    assert_eq!(parse("run `cargo test` now"), vec![text("run "), BodyToken::Code("cargo test".to_string()), text(" now")]);
}

#[test]
fn marks_nest() {
    assert_eq!(parse("**bold *both* bold**"), vec![bold(vec![text("bold "), italic(vec![text("both")]), text(" bold")])]);
    assert_eq!(parse("***both***"), vec![bold(vec![italic(vec![text("both")])])]);
    assert_eq!(parse("*a **b***"), vec![italic(vec![text("a "), bold(vec![text("b")])])]);
    assert_eq!(
        parse("||__**deep**__||"),
        vec![BodyToken::Spoiler(vec![BodyToken::Underline(vec![bold(vec![text("deep")])])])]
    );
    assert_eq!(parse("**a `**x**` b**"), vec![bold(vec![text("a "), BodyToken::Code("**x**".to_string()), text(" b")])]);
}

#[test]
fn unclosed_marks_stay_literal() {
    for source in ["**open", "*open", "__open", "||open", "`open", "a ** b", "x*", "**a *b**"] {
        let tokens = parse(source);
        assert_eq!(visible_text(&tokens).len() + markup_closed(&tokens), source.len(), "{source}: {tokens:?}");
    }
    assert_eq!(parse("**open"), vec![text("**open")]);
    assert_eq!(parse("**a *b**"), vec![bold(vec![text("a *b")])]);
    assert_eq!(parse("*a **b* c**"), vec![italic(vec![text("a **b")]), text(" c**")]);
}

/// Characters of delimiters that closed (their two sides) in `tokens`.
fn markup_closed(tokens: &[BodyToken]) -> usize {
    tokens
        .iter()
        .map(|token| match token.as_mark() {
            Some((mark, children)) => 2 * mark.delimiter().len() + markup_closed(children),
            None => 0,
        })
        .sum()
}

#[test]
fn marks_do_not_span_line_breaks() {
    assert_eq!(parse("**one\ntwo**"), vec![text("**one\ntwo**")]);
    assert_eq!(parse("*a*\n*b*"), vec![italic(vec![text("a")]), text("\n"), italic(vec![text("b")])]);
}

#[test]
fn backslash_escapes_markup() {
    assert_eq!(parse(r"\*not italic\*"), vec![text("*not italic*")]);
    assert_eq!(parse(r"\_\_x\_\_ \|\|y\|\| \`z\`"), vec![text("__x__ ||y|| `z`")]);
    assert_eq!(parse(r"**a \** b**"), vec![bold(vec![text("a ** b")])]);
    assert_eq!(parse(r"\#tag \[[x]]"), vec![text("#tag [[x]]")]);
    assert_eq!(parse(r"a\\b C:\path"), vec![text(r"a\b C:\path")]);
    assert_eq!(parse(r"trailing\"), vec![text(r"trailing\")]);
}

#[test]
fn line_start_bullets_are_not_italic() {
    assert_eq!(parse("* first\n* second"), vec![text("* first\n* second")]);
    assert_eq!(parse("- one\n- *two*"), vec![text("- one\n- "), italic(vec![text("two")])]);
    assert_eq!(parse("* item with *stress*"), vec![text("* item with "), italic(vec![text("stress")])]);
    assert_eq!(parse("2 * 3 * 4"), vec![text("2 * 3 * 4")]);
}

#[test]
fn underscores_inside_words_stay_literal() {
    assert_eq!(parse("snake__case__name"), vec![text("snake__case__name")]);
    assert_eq!(parse("a_b_c and _x_"), vec![text("a_b_c and _x_")]);
}

#[test]
fn code_spans_take_matching_backtick_runs() {
    assert_eq!(parse("``a ` b``"), vec![BodyToken::Code("a ` b".to_string())]);
    assert_eq!(parse("`a\nb`"), vec![text("`a\nb`")]);
    assert_eq!(parse("`*x* [[p]] #t`"), vec![BodyToken::Code("*x* [[p]] #t".to_string())]);
}

#[test]
fn odd_delimiter_runs_are_literal() {
    assert_eq!(parse("****"), vec![text("****")]);
    assert_eq!(parse("a ||| b ___ c"), vec![text("a ||| b ___ c")]);
    assert_eq!(parse("a | b"), vec![text("a | b")]);
}
