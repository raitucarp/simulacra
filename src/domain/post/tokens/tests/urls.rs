//! L1: web addresses written in a body are link tokens, kept exactly as written.

use super::*;

fn url(value: &str, trailing: &str) -> BodyToken {
    BodyToken::Url { url: value.to_string(), trailing: trailing.to_string() }
}

#[test]
fn a_url_is_a_token_with_its_trailing_punctuation() {
    assert_eq!(tokenize_post_body("see https://a.org/x."), vec![text("see "), url("https://a.org/x", ".")]);
    assert_eq!(tokenize_post_body("HTTPS://A.org/Path?q=1"), vec![url("HTTPS://A.org/Path?q=1", "")]);
    assert_eq!(tokenize_post_body("(https://en.wikipedia.org/wiki/Rust_(language))"), vec![text("("), url("https://en.wikipedia.org/wiki/Rust_(language)", ""), text(")")]);
}

#[test]
fn markup_inside_a_url_stays_part_of_it() {
    assert_eq!(tokenize_post_body("https://a.org/snake__case__name"), vec![url("https://a.org/snake__case__name", "")]);
    assert_eq!(tokenize_post_body("https://a.org/#frag"), vec![url("https://a.org/#frag", "")]);
}

#[test]
fn marks_around_a_url_still_apply() {
    assert_eq!(tokenize_post_body("**https://a.org**"), vec![bold(vec![url("https://a.org", "")])]);
    assert_eq!(tokenize_post_body("*go https://a.org/x*"), vec![italic(vec![text("go "), url("https://a.org/x", "")])]);
}

#[test]
fn only_whole_web_addresses_are_links() {
    for source in ["xhttps://a.org", "http://", "https:// a.org", "ftp://a.org", "mailto:a@b.org"] {
        assert!(!tokenize_post_body(source).iter().any(|t| matches!(t, BodyToken::Url { .. })), "{source}");
    }
}

#[test]
fn the_visible_text_of_a_link_is_what_was_written() {
    let source = "read https://a.org/a_b, then #tag";
    assert_eq!(visible_text(&tokenize_post_body(source)), source);
    assert_eq!(visible_text_len(source), source.chars().count());
    assert!(has_rich_markup("plain https://a.org") && !has_rich_markup("plain a.org"));
}
