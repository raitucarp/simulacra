use super::{png_bytes, scratch_media_dir};
use crate::domain::media::catalog::ImageScan;
use crate::domain::media::ImageSize;
use crate::media::images::THUMB_MAX_EDGE;
use crate::media::{scan_image, store_attachment_bytes, store_attachment_file};

#[test]
fn attachments_are_stored_once_by_content_with_their_size() {
    let media = scratch_media_dir("attach");
    let bytes = png_bytes(64, 48);
    let first = store_attachment_bytes(&bytes, &media).expect("stored");
    let second = store_attachment_bytes(&bytes, &media).expect("stored again");
    assert_eq!(first, second);
    assert_eq!(first.size, Some(ImageSize::new(64, 48)));
    assert!(first.path.ends_with(".png") && first.path.contains("attachments"));
    let source = media.join("picked.png");
    std::fs::write(&source, &bytes).expect("write source");
    assert_eq!(store_attachment_file(&source, &media).expect("from file").path, first.path);
}

#[test]
fn non_images_are_refused() {
    let media = scratch_media_dir("refuse");
    assert!(store_attachment_bytes(b"plain text, not an image", &media).is_err());
    assert!(store_attachment_bytes(b"", &media).is_err());
    assert!(store_attachment_file(&media.join("missing.png"), &media).is_err());
}

#[test]
fn small_images_need_no_thumbnail() {
    let media = scratch_media_dir("small");
    let source = media.join("small.png");
    std::fs::write(&source, png_bytes(320, 200)).expect("write");
    match scan_image(&source.display().to_string(), &media) {
        ImageScan::Known(facts) => assert_eq!((facts.size, facts.thumb_path), (ImageSize::new(320, 200), None)),
        ImageScan::Unreadable => panic!("readable"),
    }
}

#[test]
fn large_images_get_a_cached_thumbnail() {
    let media = scratch_media_dir("large");
    let source = media.join("wide.png").display().to_string();
    std::fs::write(&source, png_bytes(2600, 1300)).expect("write");
    let ImageScan::Known(facts) = scan_image(&source, &media) else { panic!("readable") };
    assert_eq!(facts.size, ImageSize::new(2600, 1300));
    let thumb = facts.thumb_path.clone().expect("thumbnail");
    let (width, height) = image::image_dimensions(&thumb).expect("thumbnail decodes");
    assert_eq!((width, height), (THUMB_MAX_EDGE, THUMB_MAX_EDGE / 2));
    let modified = std::fs::metadata(&thumb).and_then(|m| m.modified()).expect("mtime");
    let ImageScan::Known(again) = scan_image(&source, &media) else { panic!("readable") };
    assert_eq!(again.thumb_path.as_deref(), Some(thumb.as_str()), "the cached thumbnail is reused");
    assert_eq!(std::fs::metadata(&thumb).and_then(|m| m.modified()).expect("mtime"), modified);
}

#[test]
fn missing_and_corrupt_files_are_unreadable() {
    let media = scratch_media_dir("corrupt");
    let corrupt = media.join("corrupt.png");
    std::fs::write(&corrupt, b"\x89PNG\r\n\x1a\n truncated").expect("write");
    assert_eq!(scan_image(&corrupt.display().to_string(), &media), ImageScan::Unreadable);
    assert_eq!(scan_image(&media.join("nope.png").display().to_string(), &media), ImageScan::Unreadable);
    assert_eq!(scan_image(&media.display().to_string(), &media), ImageScan::Unreadable);
}
