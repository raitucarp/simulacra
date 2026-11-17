//! Where generated media (agent images) is stored: a `media` folder next to the database.

use std::path::{Path, PathBuf};

/// Folder next to the database that holds generated media.
pub const MEDIA_DIR_NAME: &str = "media";
/// Media folder (inside the system temp dir) used when the vault is in memory.
const TEMP_MEDIA_DIR_NAME: &str = "simulacra-media";

/// The media folder of the database at `db_path` (a temp folder for an in-memory vault).
pub fn media_dir_for(db_path: &str) -> PathBuf {
    let parent = Path::new(db_path).parent().filter(|_| !db_path.trim().is_empty());
    match parent {
        Some(dir) => dir.join(MEDIA_DIR_NAME),
        None => std::env::temp_dir().join(TEMP_MEDIA_DIR_NAME),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_lives_next_to_the_database_or_in_temp() {
        let dir = Path::new("vault").join("simulacra.db");
        assert_eq!(media_dir_for(&dir.to_string_lossy()), Path::new("vault").join(MEDIA_DIR_NAME));
        assert_eq!(media_dir_for(""), std::env::temp_dir().join(TEMP_MEDIA_DIR_NAME));
    }
}
