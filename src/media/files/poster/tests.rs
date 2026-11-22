//! L6: the OS poster routes, checked on every platform where they can be (file names and
//! arguments), and the Windows Shell route run for real on a scratch image.

use super::{cached_thumbnail, quick_look_args, quick_look_output, thumbnail_name};
use std::path::{Path, PathBuf};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("simulacra-poster-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

#[test]
fn freedesktop_names_follow_the_specification_example() {
    assert_eq!(thumbnail_name("file:///home/jens/photos/me.png"), "c6ee772d9e49320e97ec29a7eb5b1697.png");
}

#[test]
fn the_largest_cached_thumbnail_is_found() {
    let cache = scratch("cache");
    let video = cache.join("clip one.mp4");
    let name = thumbnail_name(url::Url::from_file_path(&video).expect("absolute").as_str());
    assert_eq!(cached_thumbnail(&video, &cache), None);
    for size in ["normal", "large"] {
        let dir = cache.join("thumbnails").join(size);
        std::fs::create_dir_all(&dir).expect("cache dir");
        std::fs::write(dir.join(&name), b"png").expect("thumbnail");
    }
    assert_eq!(cached_thumbnail(&video, &cache), Some(cache.join("thumbnails").join("large").join(&name)));
}

#[test]
fn quick_look_is_asked_for_one_thumbnail_in_the_scratch_folder() {
    let (video, out) = (Path::new("/Users/ann/clip.mov"), Path::new("/tmp/posters"));
    let args: Vec<String> = quick_look_args(video, 640, out).iter().map(|arg| arg.to_string_lossy().into_owned()).collect();
    assert_eq!(args, ["-t", "-s", "640", "-o", "/tmp/posters", "/Users/ann/clip.mov"]);
    assert_eq!(quick_look_output(video, out), Some(out.join("clip.mov.png")));
}

#[cfg(target_os = "windows")]
#[test]
fn the_shell_thumbnails_a_scratch_image() {
    let dir = scratch("shell");
    let path = dir.join("frame.png");
    let image = image::RgbImage::from_fn(320, 180, |x, _| image::Rgb([(x % 256) as u8, 90, 160]));
    image.save(&path).expect("write png");
    let thumbnail = super::windows::shell_thumbnail(&path, super::POSTER_EDGE).expect("the Shell thumbnails a PNG");
    assert!(thumbnail.width() > 0 && thumbnail.height() > 0);
    let file = crate::media::files::read_file_ref(&path).expect("read");
    let saved = super::poster_for(&file, &dir).expect("saved poster");
    assert!(Path::new(&saved).is_file());
    assert_eq!(super::poster_for(&file, &dir), Some(saved), "made once per version of the file");
}
