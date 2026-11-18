use crate::domain::media::{detect_urls, display_domain, post_links};

#[test]
fn finds_urls_in_prose_and_trims_trailing_punctuation() {
    let text = "Read https://example.org/a/b?x=1. Then (see http://foo.test/path), and https://bar.io!";
    assert_eq!(detect_urls(text), vec!["https://example.org/a/b?x=1", "http://foo.test/path", "https://bar.io/"]);
}

#[test]
fn keeps_balanced_brackets_and_drops_unbalanced_ones() {
    let text = "Wiki https://en.wikipedia.org/wiki/Rust_(programming_language) and [https://a.org/x]";
    assert_eq!(detect_urls(text), vec!["https://en.wikipedia.org/wiki/Rust_(programming_language)", "https://a.org/x"]);
}

#[test]
fn scheme_is_case_insensitive_and_ports_survive() {
    let found = detect_urls("HTTPS://Example.COM:8443/Path and Http://127.0.0.1:9000/page");
    assert_eq!(found, vec!["https://example.com:8443/Path", "http://127.0.0.1:9000/page"]);
}

#[test]
fn ignores_non_web_schemes_glued_words_and_bare_schemes() {
    for text in ["ftp://files.example.org/x", "mailto:a@b.c", "javascript:alert(1)", "xhttps://glued.example", "https:// nothing", "https://", ""] {
        assert!(detect_urls(text).is_empty(), "{text:?}");
    }
}

#[test]
fn deduplicates_and_survives_multibyte_text() {
    let text = "思考 https://例え.jp/ページ 和 https://例え.jp/ページ — «https://a.org/»";
    let found = detect_urls(text);
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found[0].starts_with("https://xn--"), "hosts are punycoded: {found:?}");
    assert_eq!(found[1], "https://a.org/");
}

#[test]
fn stops_at_markup_and_quotes() {
    assert_eq!(detect_urls("<a href=\"https://q.org/x\">x</a>"), vec!["https://q.org/x"]);
    assert_eq!(detect_urls("`https://code.org/y`"), vec!["https://code.org/y"]);
}

#[test]
fn post_links_put_the_attached_link_first_without_duplicates() {
    let links = post_links(Some("https://attached.org/a"), "also https://other.org and https://attached.org/a");
    assert_eq!(links, vec!["https://attached.org/a", "https://other.org/"]);
    assert_eq!(post_links(Some("not a url"), "plain"), Vec::<String>::new());
    assert_eq!(post_links(None, "https://x.org/1 https://x.org/2").len(), 2);
}

#[test]
fn caps_links_per_post() {
    let text: String = (0..20).map(|i| format!("https://site{i}.org ")).collect();
    assert_eq!(detect_urls(&text).len(), crate::domain::media::links::MAX_LINKS_PER_POST);
}

#[test]
fn file_names_come_from_the_last_path_segment() {
    use crate::domain::media::link_file_name;
    assert_eq!(link_file_name("https://cdn.org/talks/clip.mp4?dl=1").as_deref(), Some("clip.mp4"));
    assert_eq!(link_file_name("https://cdn.org/dir/").as_deref(), Some("dir"));
    assert_eq!(link_file_name("https://cdn.org/"), None);
}

#[test]
fn display_domain_drops_www_and_lowercases() {
    assert_eq!(display_domain("https://WWW.Example.org/a"), "example.org");
    assert_eq!(display_domain("http://127.0.0.1:8000/x"), "127.0.0.1");
    assert_eq!(display_domain("garbage"), "garbage");
}
