//! L6 on Linux: the freedesktop.org thumbnail cache. File managers save a PNG of each file
//! they have shown under `$XDG_CACHE_HOME/thumbnails/<size>/`, named by the MD5 of the
//! file's `file://` URI; a poster is read from there when one exists (none is generated).

use md5::{Digest, Md5};
use std::path::{Path, PathBuf};

/// Cache folders, largest thumbnails first.
const SIZES: [&str; 4] = ["xx-large", "x-large", "large", "normal"];
const THUMBNAILS_DIR: &str = "thumbnails";
const CACHE_ENV: &str = "XDG_CACHE_HOME";
const HOME_ENV: &str = "HOME";
const DEFAULT_CACHE: &str = ".cache";
const PNG_EXTENSION: &str = ".png";

/// The cache file name of the file whose URI is `uri` (`file:///home/ann/clip.mp4`).
pub fn thumbnail_name(uri: &str) -> String {
    let digest = Md5::digest(uri.as_bytes());
    let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    format!("{hex}{PNG_EXTENSION}")
}

/// The largest cached thumbnail of the file at the absolute `path`, under `cache`.
pub fn cached_thumbnail(path: &Path, cache: &Path) -> Option<PathBuf> {
    let uri = url::Url::from_file_path(path).ok()?;
    let name = thumbnail_name(uri.as_str());
    SIZES.iter().map(|size| cache.join(THUMBNAILS_DIR).join(size).join(&name)).find(|candidate| candidate.is_file())
}

/// `$XDG_CACHE_HOME`, else `~/.cache`.
#[cfg_attr(any(target_os = "windows", target_os = "macos"), allow(dead_code))]
pub(super) fn cache_home() -> Option<PathBuf> {
    let configured = std::env::var_os(CACHE_ENV).map(PathBuf::from).filter(|dir| dir.is_absolute());
    configured.or_else(|| std::env::var_os(HOME_ENV).map(|home| PathBuf::from(home).join(DEFAULT_CACHE)))
}
