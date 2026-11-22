use super::*;

fn n(text: &str) -> ComposeNode {
    ComposeNode::new(text)
}

fn tree(text: &str, children: Vec<ComposeNode>) -> ComposeNode {
    n(text).with_children(children)
}

#[test]
fn every_non_empty_line_is_a_post() {
    assert_eq!(split_compose("one\ntwo\nthree"), vec![n("one"), n("two"), n("three")]);
}

#[test]
fn blank_and_whitespace_lines_are_ignored() {
    let text = "\n\none\n   \n\t\n\ntwo\n\n";
    assert_eq!(split_compose(text), vec![n("one"), n("two")]);
    assert!(split_compose("").is_empty());
    assert!(split_compose(" \n\t\n").is_empty());
}

#[test]
fn bullet_markers_are_stripped_and_can_be_mixed() {
    let text = "- dash\n* star\nplain\n-\ttabbed\n*   wide";
    let expected = vec![n("dash"), n("star"), n("plain"), n("tabbed"), n("wide")];
    assert_eq!(split_compose(text), expected);
}

#[test]
fn markers_without_whitespace_and_numbers_stay_text() {
    let text = "-5 degrees\n**bold** start\n1. first\n2) second\n---\n*";
    let expected = vec![n("-5 degrees"), n("**bold** start"), n("1. first"), n("2) second"), n("---")];
    assert_eq!(split_compose(text), expected);
}

#[test]
fn empty_bullets_are_ignored() {
    assert_eq!(split_compose("- \n*\n-   \nreal"), vec![n("real")]);
}

#[test]
fn only_the_first_marker_is_stripped() {
    assert_eq!(split_compose("- - inner\n* - mixed"), vec![n("- inner"), n("- mixed")]);
}

#[test]
fn two_spaces_nest_as_a_reply() {
    let text = "- root\n  - reply\n    - deeper\n  - second reply\n- next root";
    let expected = vec![
        tree("root", vec![tree("reply", vec![n("deeper")]), n("second reply")]),
        n("next root"),
    ];
    assert_eq!(split_compose(text), expected);
    assert_eq!(draft_count(&split_compose(text)), 5);
}

#[test]
fn tabs_count_as_one_level_each() {
    let text = "root\n\treply\n\t\tdeeper\n\t  also deeper";
    let expected = vec![tree("root", vec![tree("reply", vec![n("deeper"), n("also deeper")])])];
    assert_eq!(split_compose(text), expected);
}

#[test]
fn indentation_works_without_bullets_and_with_mixed_bullets() {
    let text = "topic\n  * point\n  - counterpoint\n    plain detail";
    let expected = vec![tree("topic", vec![n("point"), tree("counterpoint", vec![n("plain detail")])])];
    assert_eq!(split_compose(text), expected);
}

#[test]
fn jumping_several_levels_in_is_still_a_direct_reply() {
    let text = "root\n      far in\nback";
    assert_eq!(split_compose(text), vec![tree("root", vec![n("far in")]), n("back")]);
}

#[test]
fn dedenting_between_levels_attaches_to_the_nearest_shallower_line() {
    // Levels 0, 3, 1: the level-1 line is a reply to the root, next to the level-3 line.
    let text = "root\n      deep\n  middle\n    under middle";
    let expected = vec![tree("root", vec![n("deep"), tree("middle", vec![n("under middle")])])];
    assert_eq!(split_compose(text), expected);
}

#[test]
fn odd_spaces_round_down_and_leading_indent_on_first_line_is_a_root() {
    assert_eq!(indent_level("   x"), 1);
    assert_eq!(indent_level(" x"), 0);
    assert_eq!(indent_level("\t \tx"), 2);
    let text = "    starts indented\n  shallower\nroot";
    assert_eq!(split_compose(text), vec![n("starts indented"), n("shallower"), n("root")]);
}

#[test]
fn indented_line_after_a_blank_line_still_nests() {
    let text = "root\n\n  reply after blank";
    assert_eq!(split_compose(text), vec![tree("root", vec![n("reply after blank")])]);
}

#[test]
fn crlf_line_endings_are_handled() {
    let text = "- root\r\n  - reply\r\n\r\nnext\r\n";
    assert_eq!(split_compose(text), vec![tree("root", vec![n("reply")]), n("next")]);
}

#[test]
fn unicode_text_is_kept_intact() {
    let text = "- 日本語のポスト\n  - réponse ✨ 🚀\n\u{3000}全角スペース";
    let expected = vec![tree("日本語のポスト", vec![n("réponse ✨ 🚀")]), n("全角スペース")];
    assert_eq!(split_compose(text), expected, "full-width space is trimmed, not indentation");
}

#[test]
fn trailing_whitespace_is_trimmed() {
    assert_eq!(split_compose("- hello   \t\n"), vec![n("hello")]);
}

#[test]
fn strip_bullet_reports_markers() {
    assert_eq!(strip_bullet("- a"), Some("a"));
    assert_eq!(strip_bullet("*\tb"), Some("b"));
    assert_eq!(strip_bullet("-"), Some(""));
    assert_eq!(strip_bullet("-a"), None);
    assert_eq!(strip_bullet("+ a"), None);
    assert_eq!(strip_bullet(""), None);
}

#[test]
fn the_preview_lists_posts_in_thread_order_with_depth_and_line() {
    let text = "First\n\n  - a reply\n        - jumped in\n* second root\n";
    let lines = compose_lines(text);
    let got: Vec<(usize, usize, &str)> = lines.iter().map(|l| (l.line, l.depth, l.text.as_str())).collect();
    assert_eq!(got, vec![(0, 0, "First"), (2, 1, "a reply"), (3, 2, "jumped in"), (4, 0, "second root")]);
    assert_eq!(lines.len(), draft_count(&split_compose(text)), "the preview counts what the splitter makes");
    assert_eq!(lines[0].chars, 5);
    assert!(compose_lines("  \n-\n").is_empty());
}

#[test]
fn a_post_over_the_limit_is_flagged_by_visible_length() {
    let long = "x".repeat(12);
    let lines = compose_lines(&format!("**{long}**\nshort"));
    assert_eq!(lines[0].chars, 12, "markup does not count");
    assert!(lines[0].is_over(10) && !lines[0].is_over(12));
    assert!(!lines[1].is_over(10));
}

#[test]
fn splitting_a_long_line_keeps_its_indent_bullet_and_neighbours() {
    let text = "Intro\n  - One two three. Four five six.\nOutro";
    let split = split_line(text, 1, 16);
    assert_eq!(split, "Intro\n  - One two three.\n  - Four five six.\nOutro");
    let depths: Vec<usize> = compose_lines(&split).iter().map(|l| l.depth).collect();
    assert_eq!(depths, vec![0, 1, 1, 0], "both pieces reply where the line did");
    assert_eq!(split_line("Short", 0, 16), "Short", "a line that fits is left alone");
    assert_eq!(split_line("Short", 7, 16), "Short", "no such line");
    assert_eq!(split_line("alpha beta gamma\r\nnext", 0, 11), "alpha beta\r\ngamma\r\nnext");
    assert_eq!(split_line("\tplain words here", 0, 11), "\tplain words\n\there");
}

#[test]
fn each_post_names_the_post_it_replies_to() {
    let parents: Vec<Option<usize>> = compose_lines("A\n  - a1\n    - a1x\n  - a2\nB\nC\n  - c1").iter().map(|l| l.parent).collect();
    assert_eq!(parents, vec![None, Some(0), Some(1), Some(0), Some(0), Some(4), Some(5)], "main-thread posts chain; indented ones reply to the line above them");
}

#[test]
fn shorthand_is_read_per_line_after_the_bullet() {
    use crate::domain::symbols::{EntrySymbol, Signifier};
    let lines = read_shorthand(compose_lines("Plans\n  - . Call the bakery\n  - * o Market at nine\n  - just a thought\n.\n"));
    let entries: Vec<Option<EntrySymbol>> = lines.iter().map(|l| l.symbols.entry).collect();
    assert_eq!(entries, vec![None, Some(EntrySymbol::Task), Some(EntrySymbol::Event), None, None]);
    assert_eq!(lines[1].text, "Call the bakery");
    assert_eq!(lines[1].chars, "Call the bakery".chars().count(), "the length counts the text after the markers");
    assert_eq!(lines[2].symbols.signifiers, vec![Signifier::Priority]);
    assert_eq!(lines[4].text, ".", "a marker with nothing after it stays text");
    assert_eq!(lines.iter().map(|l| l.parent).collect::<Vec<_>>(), vec![None, Some(0), Some(0), Some(0), Some(0)], "reading markers keeps the tree");
}
