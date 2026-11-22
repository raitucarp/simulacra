//! Whether the composer may lock a thread with the typed password (X3): it is typed twice,
//! long enough, and the thread carries text only.

use super::MIN_LOCK_PASSWORD_CHARS;

/// Why the thread cannot be locked yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockProblem {
    /// A link, image or file is attached (a locked post carries text only).
    Attachments,
    /// Shorter than [`MIN_LOCK_PASSWORD_CHARS`].
    TooShort,
    /// The two fields differ.
    Mismatch,
}

impl LockProblem {
    /// Stable key (locale keys `composer.lock.problem.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Attachments => "attachments",
            Self::TooShort => "too_short",
            Self::Mismatch => "mismatch",
        }
    }
}

/// The first problem with locking under `password` (typed again as `repeat`).
pub fn lock_problem(password: &str, repeat: &str, has_media: bool) -> Option<LockProblem> {
    if has_media {
        Some(LockProblem::Attachments)
    } else if password.chars().count() < MIN_LOCK_PASSWORD_CHARS {
        Some(LockProblem::TooShort)
    } else if password != repeat {
        Some(LockProblem::Mismatch)
    } else {
        None
    }
}
