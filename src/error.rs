//! Domain and application error types for Simulacra.

use thiserror::Error;

/// Core domain and runtime errors in Simulacra.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum AppError {
    /// Post content exceeded maximum character limit.
    #[error("post content exceeds limit of {max} characters (length: {actual})")]
    ContentTooLong { max: usize, actual: usize },

    /// Attempted to post empty or whitespace-only content.
    #[error("post content cannot be empty")]
    EmptyContent,

    /// Network could not be found with given ID.
    #[error("network '{0}' not found")]
    NetworkNotFound(String),

    /// Profile could not be found with given ID.
    #[error("profile '{0}' not found")]
    ProfileNotFound(String),

    /// Vector indexing or embedding failure.
    #[error("vector operation failed: {0}")]
    VectorError(String),

    /// An action only the user's own personas may take was attempted for an agent.
    #[error("profile '{0}' is not one of your personas")]
    NotAPersona(String),

    /// A post was scheduled for a moment that is not in the future.
    #[error("a post can only be scheduled for a moment in the future")]
    ScheduleNotInFuture,

    /// Database persistence failure.
    #[error("database error: {0}")]
    DatabaseError(String),

    /// A post's symbol state does not fit its entry kind (S2).
    #[error("the symbol state does not fit the entry")]
    InvalidSymbols,

    /// A task cannot be done while this many of its sub-tasks are still open (S8).
    #[error("{0} sub-tasks are still open")]
    OpenSubtasks(usize),

    /// A locked post carries text only (X2): no link, image or file.
    #[error("a locked post carries text only")]
    LockedWithAttachments,

    /// A locked post could not be sealed, or was about to be stored unsealed (X2).
    #[error("the post could not be locked")]
    LockFailed,

    /// A locked post was edited as plain text (X2): it must be re-sealed.
    #[error("a locked post must be unlocked and sealed again to change")]
    PostLocked,

    /// A document's title or catalogue record failed validation.
    #[error("invalid document: {0}")]
    InvalidDocument(String),
}

/// Convenience result type for Simulacra operations.
pub type Result<T> = std::result::Result<T, AppError>;
