//! Placeholder author for posts whose profile can no longer be resolved.

use super::Profile;
use std::sync::OnceLock;

const UNKNOWN_AUTHOR_ID: &str = "unknown";
const UNKNOWN_AUTHOR_NAME: &str = "Author";
const UNKNOWN_AUTHOR_HANDLE: &str = "@unknown";

impl Profile {
    /// Shared placeholder shown for posts whose author cannot be resolved.
    pub fn unknown() -> &'static Profile {
        static UNKNOWN: OnceLock<Profile> = OnceLock::new();
        UNKNOWN.get_or_init(|| Profile::new_primary(UNKNOWN_AUTHOR_ID, UNKNOWN_AUTHOR_NAME, UNKNOWN_AUTHOR_HANDLE))
    }
}
