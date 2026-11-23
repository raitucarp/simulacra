//! Password-locked posts (X2). A locked post's text is sealed with the shared
//! [`crypto`](crate::domain::crypto) seal (Argon2id key with a random salt, XChaCha20-Poly1305
//! with a random nonce) and kept as a [`PostLock`] in `extras.lock`; `content` holds only
//! [`LOCKED_PLACEHOLDER`], so the full-text index, the meaning index, previews and every
//! agent prompt see the placeholder and never the text.
//!
//! - **Associated data:** the envelope format tag, so a post lock never opens as anything
//!   else. It names no post id: an exported lock still opens once imported under a new id.
//! - **Secrets:** the password lives in a [`LockPassword`] (zeroized when dropped, redacted in
//!   `Debug`); opened text is zeroized when dropped. Neither is ever stored or logged.

mod block; // X6
mod check; // X3
mod envelope;

pub use block::{lock_block, read_lock_block, LOCK_FENCE_TAG};
pub use check::{lock_problem, LockProblem};
pub use envelope::{PostLock, LOCK_FORMAT};

use crate::domain::crypto::KdfParams;
use crate::domain::Post;
use crate::error::{AppError, Result};
use zeroize::Zeroizing;

/// What a locked post's `content` holds in place of its text.
pub const LOCKED_PLACEHOLDER: &str = "\u{1F512}";
/// Shortest password a post can be locked with (the snapshot passphrase's minimum).
pub const MIN_LOCK_PASSWORD_CHARS: usize = crate::domain::data::snapshot::MIN_PASSPHRASE_CHARS;

/// A lock password: zeroized when dropped and never printed.
#[derive(Clone, PartialEq, Eq)]
pub struct LockPassword(Zeroizing<String>);

impl LockPassword {
    pub fn new(password: impl Into<String>) -> Self {
        Self(Zeroizing::new(password.into()))
    }

    /// The password itself, for sealing and opening only.
    pub fn expose(&self) -> &str {
        &self.0
    }

    /// Whether it is long enough to lock a post with.
    pub fn meets_minimum(&self) -> bool {
        self.0.chars().count() >= MIN_LOCK_PASSWORD_CHARS
    }
}

impl std::fmt::Debug for LockPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LockPassword(..)")
    }
}

/// Why a locked post did not open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnlockError {
    /// The password is wrong (or the sealed text was changed).
    WrongPassword,
    /// The stored lock cannot be read (unknown format, damaged fields, absurd costs).
    Damaged,
}

fn associated_data() -> Vec<u8> {
    format!("post-lock/{LOCK_FORMAT}").into_bytes()
}

impl Post {
    /// Whether the post's text is sealed.
    pub fn is_locked(&self) -> bool {
        self.extras.lock.is_some()
    }

    /// Seals the post's text under `password` with `kdf` costs and leaves the placeholder in
    /// `content`; the mentions and wikilinks read from the text go with it. Text only: a
    /// post with a link, image or file is refused.
    pub fn lock_with(&mut self, password: &LockPassword, kdf: KdfParams) -> Result<()> {
        if self.has_attachments() {
            return Err(AppError::LockedWithAttachments);
        }
        if self.is_locked() {
            return Err(AppError::PostLocked);
        }
        let text = Zeroizing::new(std::mem::replace(&mut self.content, LOCKED_PLACEHOLDER.to_string()));
        self.extras.lock = Some(PostLock::seal(text.as_bytes(), password.expose(), kdf, &associated_data()).map_err(|_| AppError::LockFailed)?);
        self.mentions.clear();
        let kept = [self.reply_to_id.clone(), self.quote_id.clone(), self.repost_id.clone()];
        self.linked_posts.retain(|id| kept.contains(&Some(id.clone())));
        self.extras.defines = None; // N17: a definition would speak past the lock
        self.extras.mind_change = None; // N19: so would the old view
        Ok(())
    }

    /// The text of a locked post, opened with `password` (zeroized when dropped).
    pub fn unlock(&self, password: &LockPassword) -> std::result::Result<Zeroizing<String>, UnlockError> {
        let lock = self.extras.lock.as_ref().ok_or(UnlockError::Damaged)?;
        open_text(lock, password)
    }

    /// Whether the post is safe to store: unlocked, or locked with only the placeholder left.
    pub fn is_sealed_or_plain(&self) -> bool {
        !self.is_locked() || (self.content == LOCKED_PLACEHOLDER && self.mentions.is_empty())
    }
}

/// `lock`'s text, opened with `password`.
pub fn open_text(lock: &PostLock, password: &LockPassword) -> std::result::Result<Zeroizing<String>, UnlockError> {
    let bytes = lock.open(password.expose(), &associated_data())?;
    std::str::from_utf8(&bytes).map(|text| Zeroizing::new(text.to_string())).map_err(|_| UnlockError::Damaged)
}

/// `text` sealed under `password` (a version of a locked post, X5).
pub fn seal_text(text: &str, password: &LockPassword, kdf: KdfParams) -> Result<PostLock> {
    PostLock::seal(text.as_bytes(), password.expose(), kdf, &associated_data()).map_err(|_| AppError::LockFailed)
}

#[cfg(test)]
mod tests;
