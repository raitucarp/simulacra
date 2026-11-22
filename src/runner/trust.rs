//! What the runner checks before it starts anything (P7): the runtime's executable is still one
//! a probe would pick, and a cache kept between runs is the user's own.

use super::probe::is_executable;
use crate::domain::code::DetectedRuntime;
use std::path::{Path, PathBuf};

/// The only executables taken on Windows.
#[cfg(windows)]
const EXECUTABLE_EXTENSION: &str = "exe";

/// Whether `found` still names an executable of its runtime: an absolute path to an executable
/// file named like one its probe looks for. A stale, edited or restored cache names no other
/// program.
pub fn is_runtime_executable(found: &DetectedRuntime) -> bool {
    let path = Path::new(&found.path);
    let name = program_name(path).unwrap_or_default();
    path.is_absolute() && found.runtime.probe().executables.contains(&name.as_str()) && is_executable(path)
}

/// The name a probe looks for: on Windows the file's stem, and only of an `.exe`.
#[cfg(windows)]
fn program_name(path: &Path) -> Option<String> {
    path.extension().filter(|ext| ext.eq_ignore_ascii_case(EXECUTABLE_EXTENSION))?;
    path.file_stem().map(|stem| stem.to_string_lossy().to_lowercase())
}

#[cfg(not(windows))]
fn program_name(path: &Path) -> Option<String> {
    path.file_name().map(|name| name.to_string_lossy().into_owned())
}

/// `dir` as a cache only this user can write, made when missing; `None` when it is not the
/// user's own (Unix: someone else made it, or others may write it), and nothing is then cached
/// between runs.
pub fn private_cache(dir: &Path) -> Option<PathBuf> {
    std::fs::create_dir_all(dir).ok()?;
    let meta = std::fs::symlink_metadata(dir).ok()?;
    (meta.is_dir() && is_private(dir, &meta)).then(|| dir.to_path_buf())
}

#[cfg(unix)]
fn is_private(dir: &Path, meta: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};
    // SAFETY: `getuid` has no preconditions and cannot fail.
    let me = unsafe { libc::getuid() };
    let _ = std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700));
    let parent_ok = dir.parent().and_then(|parent| std::fs::symlink_metadata(parent).ok()).is_some_and(|parent| parent.uid() == me && parent.mode() & 0o022 == 0);
    meta.uid() == me && parent_ok
}

#[cfg(not(unix))]
fn is_private(_dir: &Path, _meta: &std::fs::Metadata) -> bool {
    true // the temp folder is the user's own on Windows
}
