use crate::domain::media::catalog::ImageScan;
use crate::domain::media::{ImageFacts, ImageSize, LinkPreview, MediaCatalog, PreviewEntry, PreviewLookup};
use crate::storage::db::Database;

const NOW: i64 = 1_800_000_000;

fn preview(title: &str) -> LinkPreview {
    LinkPreview { title: title.into(), image_url: Some("https://a.org/i.png".into()), duration_secs: Some(61), is_video: true, ..LinkPreview::default() }
}

#[test]
fn previews_round_trip_and_upsert_by_url() {
    let db = Database::in_memory().unwrap();
    let ready = PreviewEntry::ready("https://a.org/", preview("First"), Some("media/previews/a.jpg".into()), NOW).with_icon(Some("media/previews/i.png".into()));
    db.save_link_preview(&ready).unwrap();
    db.save_link_preview(&PreviewEntry::failed("https://dead.org/", NOW)).unwrap();
    let mut loaded = db.load_link_previews().unwrap();
    loaded.sort_by(|a, b| a.url.cmp(&b.url));
    assert_eq!(loaded, vec![ready, PreviewEntry::failed("https://dead.org/", NOW)]);
    let refreshed = PreviewEntry::ready("https://a.org/", preview("Second"), None, NOW + 5);
    db.save_link_preview(&refreshed).unwrap();
    let loaded = db.load_link_previews().unwrap();
    assert_eq!(loaded.len(), 2);
    assert!(loaded.contains(&refreshed));
}

#[test]
fn image_scans_round_trip_and_clear() {
    let db = Database::in_memory().unwrap();
    let known = ImageScan::Known(ImageFacts { size: ImageSize::new(1600, 900), thumb_path: Some("t.jpg".into()) });
    db.save_media_image("a.png", &known, NOW).unwrap();
    db.save_media_image("b.png", &ImageScan::Unreadable, NOW).unwrap();
    let catalog = MediaCatalog::from_rows(db.load_link_previews().unwrap(), db.load_media_images().unwrap());
    assert_eq!(catalog.image("a.png"), Some(&known));
    assert_eq!(catalog.image("b.png"), Some(&ImageScan::Unreadable));
    db.save_link_preview(&PreviewEntry::ready("https://a.org/", preview("A"), None, NOW)).unwrap();
    db.clear_media_cache().unwrap();
    assert!(db.load_media_images().unwrap().is_empty() && db.load_link_previews().unwrap().is_empty());
}

#[test]
fn corrupt_preview_json_loads_as_a_fallback_link() {
    let db = Database::in_memory().unwrap();
    db.conn
        .execute("INSERT INTO link_previews (url, preview_json, fetched_at, expires_at) VALUES ('https://x.org/', '{not json', 1, 2)", [])
        .unwrap();
    let catalog = MediaCatalog::from_rows(db.load_link_previews().unwrap(), vec![]);
    assert_eq!(catalog.preview("https://x.org/"), PreviewLookup::Unavailable);
}

#[test]
fn media_tables_are_created_idempotently_on_reopen() {
    let path = std::env::temp_dir().join(format!("simulacra-media-schema-{}.db", std::process::id()));
    let _ = std::fs::remove_file(&path);
    Database::open(&path).unwrap().save_link_preview(&PreviewEntry::failed("https://x.org/", NOW)).unwrap();
    let reopened = Database::open(&path).unwrap();
    assert_eq!(reopened.load_link_previews().unwrap().len(), 1);
    drop(reopened);
    let _ = std::fs::remove_file(&path);
}
