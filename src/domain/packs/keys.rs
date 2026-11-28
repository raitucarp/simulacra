//! Pack keys: the Classic asset path without its root and extension (`empty/no_posts` for
//! `illustrations/empty/no_posts.svg`), plus the safety rules for ids, keys and file paths.

/// Root and extension of a Classic asset path.
const ASSET_ROOT: &str = "illustrations/";
const ASSET_EXT: &str = ".svg";
/// Path separator of keys and pack files.
const SEPARATOR: char = '/';
/// Prefixes of new art that has no Classic template yet (journey, visual guides).
pub const NEW_ART_PREFIXES: [&str; 2] = ["journey/", "guides/"];
/// Families drawn as 256×256 layers composed at runtime.
const LAYERED_PREFIXES: [&str; 2] = ["portrait/", "forms/"];
/// Image formats a pack file may use.
const FILE_EXTENSIONS: [&str; 2] = ["webp", "png"];

/// The key of a Classic asset path (`None` for anything else).
pub fn key_of_asset_path(path: &str) -> Option<&str> {
    let key = path.strip_prefix(ASSET_ROOT)?.strip_suffix(ASSET_EXT)?;
    is_safe_key(key).then_some(key)
}

/// The Classic asset path of `key`.
pub fn asset_path_of_key(key: &str) -> String {
    format!("{ASSET_ROOT}{key}{ASSET_EXT}")
}

/// Whether `key` belongs to new art (no Classic template required).
pub fn is_new_art_key(key: &str) -> bool {
    NEW_ART_PREFIXES.iter().any(|prefix| key.starts_with(prefix))
}

/// Whether `key` is a layer of a composed family (portraits, forms).
pub fn is_layered_key(key: &str) -> bool {
    LAYERED_PREFIXES.iter().any(|prefix| key.starts_with(prefix))
}

/// A plain name: lowercase letters, digits, `-` and `_`.
fn is_plain(segment: &str) -> bool {
    !segment.is_empty() && segment.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

/// Whether `id` can name a pack (and its folder).
pub fn is_safe_id(id: &str) -> bool {
    is_plain(id)
}

/// Whether `key` is a relative path of plain names with a folder (`empty/no_posts`).
pub fn is_safe_key(key: &str) -> bool {
    key.contains(SEPARATOR) && key.split(SEPARATOR).all(is_plain)
}

/// Whether `file` is a relative path of plain names ending in an image extension, so it can
/// never climb out of its pack folder.
pub fn is_safe_file(file: &str) -> bool {
    let Some((path, ext)) = file.rsplit_once('.') else { return false };
    FILE_EXTENSIONS.contains(&ext) && path.split(SEPARATOR).all(is_plain)
}
