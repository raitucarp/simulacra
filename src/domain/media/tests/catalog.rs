use crate::domain::media::catalog::ImageScan;
use crate::domain::media::preview::{FAILURE_TTL_SECS, PREVIEW_TTL_SECS};
use crate::domain::media::{ImageFacts, ImageSize, LinkPreview, MediaCatalog, PreviewEntry, PreviewLookup};

const NOW: i64 = 1_800_000_000;
const URL: &str = "https://a.org/";

fn titled(title: &str) -> LinkPreview {
    LinkPreview { title: title.into(), ..LinkPreview::default() }
}

#[test]
fn ready_previews_last_a_week_and_failures_six_hours() {
    let ready = PreviewEntry::ready(URL, titled("A"), None, NOW);
    assert_eq!(ready.expires_at - ready.fetched_at, PREVIEW_TTL_SECS);
    assert!(ready.is_fresh(NOW + PREVIEW_TTL_SECS - 1) && !ready.is_fresh(NOW + PREVIEW_TTL_SECS));
    let failed = PreviewEntry::failed(URL, NOW);
    assert_eq!(failed.expires_at - NOW, FAILURE_TTL_SECS);
    assert!(!failed.is_ready());
}

#[test]
fn an_empty_page_is_cached_like_a_failure() {
    let empty = PreviewEntry::ready(URL, LinkPreview::default(), None, NOW);
    assert!(!empty.is_ready());
    assert_eq!(empty.expires_at - NOW, FAILURE_TTL_SECS);
}

#[test]
fn due_previews_are_missing_or_stale_and_not_in_flight() {
    let stale = PreviewEntry { expires_at: NOW - 1, ..PreviewEntry::ready("https://stale.org/", titled("S"), None, NOW - PREVIEW_TTL_SECS) };
    let fresh = PreviewEntry::ready("https://fresh.org/", titled("F"), None, NOW);
    let catalog = MediaCatalog::from_rows(vec![stale, fresh], vec![]).started(&["https://busy.org/".into()], &[]);
    let urls = ["https://new.org/", "https://stale.org/", "https://fresh.org/", "https://busy.org/", "https://new.org/"];
    assert_eq!(catalog.previews_due(urls, NOW, 10), vec!["https://new.org/", "https://stale.org/"]);
    assert_eq!(catalog.previews_due(urls, NOW, 1), vec!["https://new.org/"]);
}

#[test]
fn lookups_show_stale_cards_loading_and_fallbacks() {
    let catalog = MediaCatalog::default().started(&[URL.into()], &[]);
    assert_eq!(catalog.preview(URL), PreviewLookup::Loading);
    assert_eq!(catalog.preview("https://never.org/"), PreviewLookup::Unavailable);
    let revision = catalog.revision();
    let catalog = catalog.with_preview(PreviewEntry::ready(URL, titled("A"), None, NOW - 2 * PREVIEW_TTL_SECS));
    assert!(matches!(catalog.preview(URL), PreviewLookup::Ready(entry) if entry.preview.as_ref().unwrap().title == "A"));
    assert_eq!(catalog.in_flight(), (0, 0));
    assert!(catalog.revision() > revision);
    let catalog = catalog.with_preview(PreviewEntry::failed("https://dead.org/", NOW));
    assert_eq!(catalog.preview("https://dead.org/"), PreviewLookup::Unavailable);
}

#[test]
fn images_are_scanned_once_and_cleared_on_request() {
    let facts = ImageFacts { size: ImageSize::new(800, 600), thumb_path: Some("t.jpg".into()) };
    let catalog = MediaCatalog::default().started(&[], &["a.png".into()]);
    assert_eq!(catalog.images_due(["a.png", "b.png", "b.png"], 8), vec!["b.png"]);
    let catalog = catalog.with_image("a.png".into(), ImageScan::Known(facts.clone()));
    assert_eq!(catalog.image("a.png"), Some(&ImageScan::Known(facts.clone())));
    assert_eq!(facts.display_path("a.png"), "t.jpg");
    assert!(catalog.images_due(["a.png"], 8).is_empty());
    let cleared = catalog.cleared();
    assert_eq!(cleared.image("a.png"), None);
    assert_eq!(cleared.images_due(["a.png"], 8), vec!["a.png"]);
}
