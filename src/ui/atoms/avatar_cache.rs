//! Per-path memo of avatar image resolution, so rendering never stats the file system.
//!
//! The first render of a path checks whether the file exists and keeps the shared
//! `Arc<Path>` image source (or the miss); later frames reuse it. Entries are keyed by the
//! trimmed path, so changing a profile's avatar path resolves afresh; [`forget_avatar`]
//! drops an entry whose file may have changed in place.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

thread_local! {
    /// Resolved avatar sources by trimmed path (`None` = no readable file).
    static AVATARS: RefCell<HashMap<String, Option<Arc<Path>>>> = RefCell::new(HashMap::new());
    /// File-system checks performed (observed by tests).
    static LOOKUPS: RefCell<usize> = const { RefCell::new(0) };
}

/// The image source for `path`, or `None` when it is blank or no file exists there.
pub fn resolve_avatar(path: &str) -> Option<Arc<Path>> {
    let key = path.trim();
    if key.is_empty() {
        return None;
    }
    AVATARS.with(|cache| {
        if let Some(hit) = cache.borrow().get(key) {
            return hit.clone();
        }
        LOOKUPS.with(|n| *n.borrow_mut() += 1);
        let resolved = Path::new(key).exists().then(|| Arc::<Path>::from(Path::new(key)));
        cache.borrow_mut().insert(key.to_string(), resolved.clone());
        resolved
    })
}

/// Forgets the cached resolution of `path` (call after its file was replaced or created).
pub fn forget_avatar(path: &str) {
    AVATARS.with(|cache| cache.borrow_mut().remove(path.trim()));
}

/// Number of file-system checks so far on this thread.
pub fn avatar_lookups() -> usize {
    LOOKUPS.with(|n| *n.borrow())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_renders_check_the_file_system_once_per_path() {
        let file = std::env::temp_dir().join(format!("simulacra_avatar_{}.png", uuid::Uuid::new_v4()));
        let path = file.to_string_lossy().to_string();
        let before = avatar_lookups();
        assert!(resolve_avatar(&path).is_none());
        std::fs::write(&file, b"png").unwrap();
        assert!(resolve_avatar(&format!(" {path} ")).is_none(), "misses are cached too");
        assert_eq!(avatar_lookups(), before + 1);

        forget_avatar(&path);
        assert!(resolve_avatar(&path).is_some(), "a forgotten path resolves afresh");
        for _ in 0..100 {
            assert!(resolve_avatar(&path).is_some());
        }
        assert_eq!(avatar_lookups(), before + 2);
        assert!(resolve_avatar("   ").is_none());
        let _ = std::fs::remove_file(&file);
    }
}
