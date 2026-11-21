use super::*;
use crate::domain::post::{tokenize_post_body, BodyToken};

/// Applies `mark` to the `[` `]`-marked selection of `marked` and returns the text with the
/// new selection marked the same way.
fn toggle(marked: &str, mark: FormatMark) -> String {
    let start = marked.find('[').expect("selection start");
    let end = marked.find(']').expect("selection end") - 1;
    let text = marked.replacen('[', "", 1).replacen(']', "", 1);
    let edit = toggle_mark(&text, start..end, mark);
    let new = edit.apply(&text);
    format!("{}[{}]{}", &new[..edit.selection.start], &new[edit.selection.clone()], &new[edit.selection.end..])
}

#[test]
fn every_mark_wraps_the_selection_and_keeps_it_selected() {
    assert_eq!(toggle("a [word] b", FormatMark::Bold), "a **[word]** b");
    assert_eq!(toggle("a [word] b", FormatMark::Italic), "a *[word]* b");
    assert_eq!(toggle("a [word] b", FormatMark::Underline), "a __[word]__ b");
    assert_eq!(toggle("a [word] b", FormatMark::Code), "a `[word]` b");
    assert_eq!(toggle("a [word] b", FormatMark::Spoiler), "a ||[word]|| b");
}

#[test]
fn toggles_off_when_already_wrapped_outside_or_inside() {
    assert_eq!(toggle("a **[word]** b", FormatMark::Bold), "a [word] b");
    assert_eq!(toggle("a [**word**] b", FormatMark::Bold), "a [word] b");
    assert_eq!(toggle("a ||[word]|| b", FormatMark::Spoiler), "a [word] b");
    assert_eq!(toggle("a [__word__] b", FormatMark::Underline), "a [word] b");
    assert_eq!(toggle("a `[word]` b", FormatMark::Code), "a [word] b");
    assert_eq!(toggle("a *[word]* b", FormatMark::Italic), "a [word] b");
}

#[test]
fn stars_are_counted_so_bold_and_italic_toggle_apart() {
    assert_eq!(toggle("**[x]**", FormatMark::Italic), "***[x]***", "italic inside bold adds a star");
    assert_eq!(toggle("***[x]***", FormatMark::Italic), "**[x]**");
    assert_eq!(toggle("***[x]***", FormatMark::Bold), "*[x]*");
    assert_eq!(toggle("*[x]*", FormatMark::Bold), "***[x]***");
}

#[test]
fn surrounding_whitespace_stays_outside_the_delimiters() {
    assert_eq!(toggle("[ word ]", FormatMark::Bold), " **[word]** ");
    assert_eq!(tokenize_post_body(&toggle("say [hi ]now", FormatMark::Bold).replace(['[', ']'], ""))[1], BodyToken::Bold(vec![BodyToken::Text("hi".into())]));
}

#[test]
fn an_empty_selection_inserts_a_pair_and_a_second_press_removes_it() {
    assert_eq!(toggle("ab[]cd", FormatMark::Bold), "ab**[]**cd");
    assert_eq!(toggle("ab**[]**cd", FormatMark::Bold), "ab[]cd");
}

#[test]
fn several_lines_are_formatted_line_by_line() {
    assert_eq!(toggle("[one\n\ntwo]", FormatMark::Bold), "[**one**\n\n**two**]");
    assert_eq!(toggle("[**one**\n**two**]", FormatMark::Bold), "[one\ntwo]");
    assert_eq!(toggle("[**one**\ntwo]", FormatMark::Bold), "[**one**\n**two**]", "mixed lines all end up wrapped");
}

#[test]
fn code_over_several_lines_becomes_a_fenced_block_and_back() {
    assert_eq!(toggle("intro\nlet [a = 1;\nlet b] = 2;\nend", FormatMark::Code), "intro\n```\n[let a = 1;\nlet b = 2;]\n```\nend");
    assert_eq!(toggle("[```\nx\n```]", FormatMark::Code), "[x]");
}

#[test]
fn code_around_backticks_uses_a_longer_fence() {
    assert_eq!(toggle("[a`b]", FormatMark::Code), "``[a`b]``");
    assert_eq!(toggle("[`x]", FormatMark::Code), "`` [`x] ``");
}

#[test]
fn wrapped_text_parses_back_to_the_mark() {
    let cases = [
        (FormatMark::Bold, BodyToken::Bold(vec![BodyToken::Text("hi".into())])),
        (FormatMark::Italic, BodyToken::Italic(vec![BodyToken::Text("hi".into())])),
        (FormatMark::Underline, BodyToken::Underline(vec![BodyToken::Text("hi".into())])),
        (FormatMark::Spoiler, BodyToken::Spoiler(vec![BodyToken::Text("hi".into())])),
        (FormatMark::Code, BodyToken::Code("hi".into())),
    ];
    for (mark, token) in cases {
        let edit = toggle_mark("hi", 0..2, mark);
        assert_eq!(tokenize_post_body(&edit.apply("hi")), vec![token], "{mark:?}");
    }
}

#[test]
fn out_of_range_or_reversed_selections_are_clamped() {
    let reversed = Range { start: 9, end: 1 };
    assert_eq!(toggle_mark("abc", reversed, FormatMark::Bold).apply("abc"), "a**bc**");
    assert_eq!(toggle_mark("é", 1..1, FormatMark::Bold).selection, 2..2, "a split character is not cut");
    assert_eq!(FormatMark::ALL.len(), 5);
    assert_eq!(FormatMark::Code.id(), "code");
}
