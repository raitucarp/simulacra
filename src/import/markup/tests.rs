use super::*;

fn names(src: &str) -> Vec<String> {
    tokenize(src)
        .map(|token| match token {
            Token::Open(tag) => format!("<{}>", tag.name),
            Token::Close(name) => format!("</{name}>"),
            Token::Text(text) => text.to_string(),
        })
        .collect()
}

#[test]
fn tokenizes_tags_text_and_skips_declarations() {
    let src = "<?xml version=\"1.0\"?><!DOCTYPE html><!-- note --><p class='a'>Hi <b>there</b></p>";
    assert_eq!(names(src), ["<p>", "Hi ", "<b>", "there", "</b>", "</p>"]);
}

#[test]
fn script_and_style_bodies_are_swallowed() {
    let src = "<style>p > a { color: red }</style><p>x</p><script>if (a < b) {}</script>";
    assert_eq!(names(src), ["<style>", "</style>", "<p>", "x", "</p>", "<script>", "</script>"]);
}

#[test]
fn stray_angle_brackets_stay_text() {
    let joined: String = names("a < b and c > d <3").concat();
    assert_eq!(joined, "a < b and c > d <3");
}

#[test]
fn attributes_are_found_by_local_or_full_name() {
    let tag = Tag::parse(r#"a epub:type="noteref" xlink:href='ch1.xhtml#n1' hidden data-x=1 /"#);
    assert!(tag.self_closing);
    assert_eq!(tag.attr("href").as_deref(), Some("ch1.xhtml#n1"));
    assert_eq!(tag.attr("epub:type").as_deref(), Some("noteref"));
    assert_eq!(tag.attr("data-x").as_deref(), Some("1"));
    assert_eq!(tag.attr("hidden").as_deref(), Some(""));
    assert_eq!(tag.attr("missing"), None);
    assert!(Tag::parse("w:p").is("p"));
}

#[test]
fn cdata_is_text() {
    assert_eq!(names("<a><![CDATA[x < y]]></a>"), ["<a>", "x < y", "</a>"]);
}

#[test]
fn entities_decode_named_and_numeric_references() {
    assert_eq!(decode_entities("Tom &amp; Jerry &#8212; &#x263A; &hellip;"), "Tom & Jerry — ☺ …");
    assert_eq!(decode_entities("AT&T &unknown; & alone"), "AT&T &unknown; & alone");
    assert!(matches!(decode_entities("plain"), std::borrow::Cow::Borrowed(_)));
}

#[test]
fn closes_matches_local_names() {
    let tokens: Vec<Token> = tokenize("<w:p></w:p>").collect();
    assert!(tokens[1].closes("p"));
    assert!(!tokens[0].closes("p"));
}
