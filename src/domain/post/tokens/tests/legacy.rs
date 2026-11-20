//! The tokens bodies had before marks keep coming out unchanged.

use super::*;

#[test]
fn wikilinks_hashtags_and_text_are_unchanged() {
    let tokens = tokenize_post_body("See [[ post_1 ]]， and #virtue! end [[open");
    assert_eq!(
        tokens,
        vec![
            text("See "),
            BodyToken::Wikilink { target: "post_1".to_string(), trailing: "，".to_string() },
            text(" and "),
            BodyToken::Hashtag { tag: "#virtue".to_string(), trailing: "!".to_string() },
            text(" end [[open"),
        ]
    );
}

#[test]
fn a_leading_hash_is_a_hashtag_or_text_never_a_heading() {
    assert_eq!(
        tokenize_post_body("#title here"),
        vec![BodyToken::Hashtag { tag: "#title".to_string(), trailing: String::new() }, text(" here")]
    );
    assert_eq!(tokenize_post_body("# not a heading"), vec![text("# not a heading")]);
    assert_eq!(tokenize_post_body("a#b # c"), vec![text("a#b # c")]);
}

#[test]
fn line_breaks_stay_in_text() {
    assert_eq!(tokenize_post_body("one\ntwo\n\nthree"), vec![text("one\ntwo\n\nthree")]);
}

#[test]
fn hashtags_and_wikilinks_work_inside_marks() {
    let tag = BodyToken::Hashtag { tag: "#tag".to_string(), trailing: String::new() };
    assert_eq!(tokenize_post_body("**#tag**"), vec![bold(vec![tag.clone()])]);
    assert_eq!(tokenize_post_body("__#snake_case__"), vec![BodyToken::Underline(vec![BodyToken::Hashtag {
        tag: "#snake_case".to_string(),
        trailing: String::new(),
    }])]);
    let link = BodyToken::Wikilink { target: "a__b".to_string(), trailing: String::new() };
    assert_eq!(tokenize_post_body("*[[a__b]]*"), vec![italic(vec![link])]);
    assert_eq!(wikilink_targets(&tokenize_post_body("x **[[p1]] and *[[p2]]***")), vec!["p1", "p2"]);
}

#[test]
fn mark_and_variant_helpers_agree() {
    for mark in [Mark::Bold, Mark::Italic, Mark::Underline, Mark::Spoiler] {
        let token = mark.wrap(vec![text("x")]);
        assert_eq!(token.as_mark(), Some((mark, &[text("x")][..])));
        let source = format!("{0}x{0}", mark.delimiter());
        assert_eq!(tokenize_post_body(&source), vec![token]);
    }
    assert_eq!(text("x").as_mark(), None);
    assert!(has_rich_markup("**a**") && has_rich_markup("a\nb") && !has_rich_markup("plain words"));
}
