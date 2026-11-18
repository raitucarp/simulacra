use crate::domain::media::{decode_html, parse_link_preview};

const ARTICLE: &str = include_str!("fixtures/article.html");
const TWITTER_ONLY: &str = include_str!("fixtures/twitter_only.html");
const VIDEO_PAGE: &str = include_str!("fixtures/video_page.html");
const MALFORMED: &str = include_str!("fixtures/malformed.html");

#[test]
fn reads_opengraph_with_relative_image_and_icon() {
    let preview = parse_link_preview(ARTICLE, "https://notes.example.org/essays/spinoza?ref=feed");
    assert_eq!(preview.title, "On the Ethics of Spinoza & the Geometry of Mind");
    assert_eq!(preview.description, "A long walk through Part III, where affects become geometry.");
    assert_eq!(preview.site_name, "The Field Notes");
    assert_eq!(preview.image_url.as_deref(), Some("https://notes.example.org/images/ethics-cover.jpg"));
    assert_eq!(preview.icon_url.as_deref(), Some("https://notes.example.org/static/icon-32.png"));
    assert!(!preview.is_video);
    assert_eq!(preview.duration_secs, None);
}

#[test]
fn script_bodies_never_leak_tags() {
    let preview = parse_link_preview(ARTICLE, "https://notes.example.org/");
    assert_ne!(preview.title, "script trap");
}

#[test]
fn falls_back_to_twitter_cards_meta_description_and_shortcut_icon() {
    let preview = parse_link_preview(TWITTER_ONLY, "https://example.net/post/1");
    assert_eq!(preview.title, "Card title");
    assert_eq!(preview.description, "Plain meta description");
    assert_eq!(preview.image_url.as_deref(), Some("https://img.example.net/card.png"));
    assert_eq!(preview.icon_url.as_deref(), Some("https://example.net/favicon.png"));
}

#[test]
fn falls_back_to_the_title_element_and_the_default_icon() {
    let html = "<html><head><title>  Just a   title </title></head><body>x</body></html>";
    let preview = parse_link_preview(html, "http://127.0.0.1:8000/deep/page.html");
    assert_eq!(preview.title, "Just a title");
    assert_eq!(preview.icon_url.as_deref(), Some("http://127.0.0.1:8000/favicon.ico"));
    assert!(preview.has_content());
}

#[test]
fn detects_video_pages_and_their_duration() {
    let preview = parse_link_preview(VIDEO_PAGE, "https://videos.example.com/watch/1");
    assert!(preview.is_video);
    assert_eq!(preview.duration_secs, Some(3723));
    assert_eq!(preview.title, "A lecture on monads");
}

#[test]
fn malformed_html_yields_what_it_can_and_rejects_unsafe_urls() {
    let preview = parse_link_preview(MALFORMED, "https://broken.example/");
    assert_eq!(preview.title, "", "an unclosed title and an unclosed meta tag give no title");
    assert_eq!(preview.description, "bare-value");
    assert_eq!(preview.site_name, "It's & ok");
    assert_eq!(preview.image_url, None, "javascript: and data: images are dropped");
    assert_eq!(preview.duration_secs, None);
    assert!(!preview.is_video);
}

#[test]
fn garbage_and_empty_input_never_panic() {
    for html in ["", "<", "<<<>>>", "<meta", "<meta property=\"og:title\" content=\"", "\u{0}\u{1}<head></head>", "<HEAD><META PROPERTY=OG:TITLE CONTENT=Loud></HEAD>"] {
        let _ = parse_link_preview(html, "https://x.org/");
        let _ = parse_link_preview(html, "not a url");
    }
    assert_eq!(parse_link_preview("<HEAD><META PROPERTY=OG:TITLE CONTENT=Loud></HEAD>", "https://x.org/").title, "Loud");
}

#[test]
fn long_titles_and_descriptions_are_clipped() {
    let long = "word ".repeat(300);
    let html = format!("<head><meta property=\"og:title\" content=\"{long}\"><meta name=\"description\" content=\"{long}\"></head>");
    let preview = parse_link_preview(&html, "https://x.org/");
    assert!(preview.title.chars().count() <= 201 && preview.title.ends_with('…'));
    assert!(preview.description.chars().count() <= 401);
}

#[test]
fn decodes_declared_charsets() {
    let latin1 = b"<head><meta charset=\"iso-8859-1\"><title>Caf\xe9</title></head>";
    assert!(decode_html(latin1, None).contains("Café"));
    let header = b"<title>Gr\xfc\xdfe</title>";
    assert!(decode_html(header, Some("text/html; charset=windows-1252")).contains("Grüße"));
    assert!(decode_html("Plain ünïcode".as_bytes(), Some("text/html")).contains("ünïcode"));
    assert!(decode_html(b"\xff broken", Some("text/html; charset=bogus-charset")).contains("broken"));
    assert!(decode_html(b"\xff\xfeH\x00i\x00", None).contains("Hi"), "a byte-order mark wins");
}
