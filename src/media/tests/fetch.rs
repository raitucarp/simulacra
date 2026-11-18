use super::{png_bytes, scratch_media_dir};
use crate::agent::mock_server::{MockReply, MockServer};
use crate::domain::media::preview::FAILURE_TTL_SECS;
use crate::media::fetch::MAX_PAGE_BYTES;
use crate::media::{fetch_preview_blocking, PreviewJob};

const NOW: i64 = 1_800_000_000;
const HTML: &str = "text/html; charset=utf-8";
const PAGE: &str = r#"<html><head><title>Fallback</title>
<meta property="og:title" content="Mock essay">
<meta property="og:description" content="Served from 127.0.0.1">
<meta property="og:image" content="/cover.png">
</head><body>ignored</body></html>"#;

fn job(server: &MockServer, path: &str, name: &str) -> PreviewJob {
    PreviewJob { url: format!("{}{path}", server.url), allow_private: true, media_dir: scratch_media_dir(name) }
}

#[test]
fn reads_the_page_then_saves_its_image() {
    let server = MockServer::start(vec![MockReply::raw(200, HTML, PAGE), MockReply::raw(200, "image/png", png_bytes(900, 600))]);
    let job = job(&server, "/essay", "page");
    let entry = fetch_preview_blocking(&job, NOW);
    let preview = entry.preview.clone().expect("preview");
    assert_eq!((preview.title.as_str(), preview.description.as_str()), ("Mock essay", "Served from 127.0.0.1"));
    assert_eq!(preview.image_url, Some(format!("{}/cover.png", server.url)));
    let saved = entry.image_path.clone().expect("saved image");
    let (width, height) = image::image_dimensions(&saved).expect("decodable");
    assert!(width <= crate::media::images::PREVIEW_MAX_EDGE && width * 2 == height * 3, "{width}x{height}");
    assert!(entry.is_fresh(NOW) && entry.fetched_at == NOW);
    assert_eq!(entry.icon_path, None, "the default icon was not served");
}

#[test]
fn saves_the_site_icon() {
    let page = "<head><title>Iconic</title><link rel=\"icon\" href=\"/i.png\"></head>";
    let server = MockServer::start(vec![MockReply::raw(200, HTML, page), MockReply::raw(200, "image/png", png_bytes(128, 128))]);
    let entry = fetch_preview_blocking(&job(&server, "/", "icon"), NOW);
    let icon = entry.icon_path.expect("icon saved");
    assert_eq!(image::image_dimensions(&icon).expect("icon decodes"), (64, 64));
    assert_eq!(server.requests()[1].path, "/i.png");
}

#[test]
fn requests_carry_no_cookies_referrer_or_user_details() {
    let server = MockServer::start(vec![MockReply::raw(200, HTML, PAGE), MockReply::raw(200, "image/png", png_bytes(4, 4))]);
    fetch_preview_blocking(&job(&server, "/essay", "headers"), NOW);
    let requests = server.requests();
    assert_eq!(requests.len(), 2);
    for request in &requests {
        assert_eq!(request.method, "GET");
        assert!(request.header("cookie").is_none() && request.header("referer").is_none() && request.header("authorization").is_none());
        let agent = request.header("user-agent").unwrap_or_default();
        assert!(agent.contains("SimulacraLinkPreview") && !agent.contains(&whoami()), "{agent}");
    }
}

/// The OS user name, which must never appear in a request.
fn whoami() -> String {
    std::env::var("USERNAME").or_else(|_| std::env::var("USER")).unwrap_or_else(|_| "\u{0}".into())
}

#[test]
fn follows_redirects_within_the_cap_and_fails_past_it() {
    let redirect = || MockReply::raw(302, HTML, "").with_header("Location", "/next");
    let server = MockServer::start(vec![redirect(), redirect(), MockReply::raw(200, HTML, "<title>Arrived</title>")]);
    let entry = fetch_preview_blocking(&job(&server, "/start", "redirect"), NOW);
    assert_eq!(entry.preview.map(|p| p.title).as_deref(), Some("Arrived"));
    let hops = crate::media::fetch::MAX_REDIRECTS + 2;
    let server = MockServer::start((0..hops).map(|_| redirect()).collect());
    let entry = fetch_preview_blocking(&job(&server, "/loop", "redirect-loop"), NOW);
    assert!(!entry.is_ready());
    assert!(server.requests().len() <= crate::media::fetch::MAX_REDIRECTS + 1);
}

#[test]
fn private_addresses_and_other_schemes_are_refused_without_a_request() {
    let server = MockServer::start(vec![MockReply::raw(200, HTML, PAGE)]);
    let mut private = job(&server, "/essay", "private");
    private.allow_private = false;
    assert!(!fetch_preview_blocking(&private, NOW).is_ready());
    for url in ["ftp://127.0.0.1/x", "file:///etc/passwd", "javascript:alert(1)", "not a url"] {
        let other = PreviewJob { url: url.into(), ..private.clone() };
        assert!(!fetch_preview_blocking(&other, NOW).is_ready(), "{url}");
    }
    assert!(server.requests().is_empty());
}

#[test]
fn errors_and_non_pages_are_cached_as_short_lived_failures() {
    let server = MockServer::start(vec![MockReply::raw(404, HTML, "<title>Missing</title>"), MockReply::raw(200, "application/pdf", "%PDF-1.4")]);
    for (path, name) in [("/gone", "gone"), ("/paper.pdf", "pdf")] {
        let entry = fetch_preview_blocking(&job(&server, path, name), NOW);
        assert!(!entry.is_ready(), "{path}");
        assert_eq!(entry.expires_at, NOW + FAILURE_TTL_SECS);
    }
}

#[test]
fn oversized_pages_are_read_up_to_the_cap() {
    let mut page = String::from("<html><head><meta property=\"og:title\" content=\"Huge page\"></head><body>");
    page.push_str(&"x".repeat(MAX_PAGE_BYTES * 2));
    let server = MockServer::start(vec![MockReply::raw(200, HTML, page)]);
    let entry = fetch_preview_blocking(&job(&server, "/huge", "huge"), NOW);
    assert_eq!(entry.preview.map(|p| p.title).as_deref(), Some("Huge page"));
}

#[test]
fn a_link_to_an_image_previews_the_image_itself() {
    let server = MockServer::start(vec![MockReply::raw(200, "image/png", png_bytes(40, 30)), MockReply::raw(200, "image/png", png_bytes(40, 30))]);
    let entry = fetch_preview_blocking(&job(&server, "/photo.png", "direct"), NOW);
    assert!(entry.is_ready());
    assert!(entry.image_path.is_some_and(|path| std::path::Path::new(&path).exists()));
}

#[test]
fn an_unreadable_image_still_leaves_the_text_preview() {
    let server = MockServer::start(vec![MockReply::raw(200, HTML, PAGE), MockReply::raw(200, "image/png", "not really a png")]);
    let entry = fetch_preview_blocking(&job(&server, "/essay", "bad-image"), NOW);
    assert!(entry.is_ready() && entry.image_path.is_none());
}
