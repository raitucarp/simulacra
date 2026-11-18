//! Edge tests of post media: preview fetching against the local mock server only, and image
//! storage, scanning and thumbnails in a scratch media folder.

mod fetch;
mod images;

use std::path::PathBuf;

/// A fresh, empty media folder for one test.
fn scratch_media_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("simulacra-media-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch media dir");
    dir
}

/// PNG bytes of a `width` x `height` gradient.
fn png_bytes(width: u32, height: u32) -> Vec<u8> {
    let image = image::RgbImage::from_fn(width, height, |x, y| image::Rgb([(x % 256) as u8, (y % 256) as u8, 128]));
    let mut out = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(image).write_to(&mut out, image::ImageFormat::Png).expect("encode png");
    out.into_inner()
}
