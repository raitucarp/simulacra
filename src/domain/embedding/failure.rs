//! Why embedding stopped, as the user is told it (each kind has a localized notice).

/// A classified embedding failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailureKind {
    /// No key is configured for a provider that needs one (nothing was sent).
    MissingKey,
    /// The provider rejected the key (invalid, expired or without access).
    BadKey,
    /// Rate limit or quota exhausted after every retry.
    Quota,
    /// The daily AI budget in Settings › AI & Models is used up (nothing was sent).
    Budget,
    /// The provider or local server could not be reached.
    Network,
    /// The model id is not served by this provider.
    ModelNotFound,
    /// The provider failed on its side (5xx) after every retry.
    Server,
    /// The request was refused as malformed, or the reply could not be read.
    InvalidResponse,
}

impl FailureKind {
    /// Stable key (the locale suffix of the notice).
    pub const fn key(self) -> &'static str {
        match self {
            Self::MissingKey => "missing_key",
            Self::BadKey => "bad_key",
            Self::Quota => "quota",
            Self::Budget => "budget",
            Self::Network => "network",
            Self::ModelNotFound => "model_not_found",
            Self::Server => "server",
            Self::InvalidResponse => "invalid_response",
        }
    }

    /// Whether waiting and sending again can help.
    pub const fn is_retryable(self) -> bool {
        matches!(self, Self::Quota | Self::Network | Self::Server)
    }
}
