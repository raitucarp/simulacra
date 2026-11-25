//! L6, L7: a video's poster frame from the OS, never from a bundled codec
//! (`docs/decisions/L6-video-poster-frames.md`): the Shell thumbnail on Windows, Quick Look on
//! macOS, the freedesktop thumbnail cache on Linux. The macOS and Linux routes are compiled on
//! every platform, so every build checks them; the target picks one at run time.

mod freedesktop;
mod quick_look;
#[cfg(target_os = "windows")]
mod windows;

pub use freedesktop::{cached_thumbnail, thumbnail_name};
pub use quick_look::{quick_look_args, quick_look_output, ran_to_success, QUICK_LOOK_LIMITS};

use crate::domain::media::FileRef;
use crate::import::parse::sha256_hex;
use image::DynamicImage;
use std::path::Path;

/// Longest edge asked of the OS.
pub const POSTER_EDGE: u32 = 640;
/// Folder under the media folder where the OS tool writes before the poster is kept.
const SCRATCH_DIR: &str = "posters-scratch";
const KEY_HASH_CHARS: usize = 24;

/// The saved poster of the video `file`, made once per version of the file; `None` when the
/// OS has none. Blocking: run it on a thread of its own.
pub fn poster_for(file: &FileRef, media_dir: &Path) -> Option<String> {
    let key = poster_key(file);
    if let Some(saved) = crate::media::images::saved_poster(&key, media_dir) {
        return Some(saved);
    }
    let poster = os_poster(Path::new(&file.path), &media_dir.join(SCRATCH_DIR))?;
    crate::media::images::save_poster(poster, &key, media_dir).ok()
}

/// A name that changes whenever the file is replaced.
fn poster_key(file: &FileRef) -> String {
    let stamp = file.modified.map_or(0, |at| at.as_nanosecond());
    sha256_hex(format!("{}|{}|{stamp}", file.path, file.size).as_bytes()).chars().take(KEY_HASH_CHARS).collect()
}

/// The OS's poster frame of the video at `path`.
fn os_poster(path: &Path, scratch: &Path) -> Option<DynamicImage> {
    #[cfg(target_os = "windows")]
    let poster = windows::shell_thumbnail(path, POSTER_EDGE).map(DynamicImage::ImageRgba8);
    #[cfg(target_os = "macos")]
    let poster = quick_look::quick_look(path, POSTER_EDGE, scratch);
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let poster = freedesktop::cache_home().and_then(|cache| cached_thumbnail(path, &cache)).and_then(|png| image::open(png).ok());
    let _ = scratch;
    poster
}

#[cfg(test)]
mod tests;
