//! A locked post as exported text (X6): a fenced block tagged [`LOCK_FENCE_TAG`] holding the
//! envelope as JSON, ciphertext only. An import reads it back into a locked post, which
//! opens with the same password.

use super::PostLock;

/// The language tag of the fenced block.
pub const LOCK_FENCE_TAG: &str = "post-lock";
const FENCE: &str = "```";

/// `lock` as a fenced block.
pub fn lock_block(lock: &PostLock) -> String {
    let json = serde_json::to_string(lock).unwrap_or_default();
    format!("{FENCE}{LOCK_FENCE_TAG}\n{json}\n{FENCE}")
}

/// The lock in `text` when it is exactly one such block (surrounding space allowed).
pub fn read_lock_block(text: &str) -> Option<PostLock> {
    let body = text.trim().strip_prefix(FENCE)?.strip_prefix(LOCK_FENCE_TAG)?.strip_suffix(FENCE)?;
    serde_json::from_str(body.trim()).ok()
}
