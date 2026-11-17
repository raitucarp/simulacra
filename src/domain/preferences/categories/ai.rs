//! AI & Models: request timeout and retries, the daily budget and "ask before spending".
//! The provider, key and default model keep their own rows (`LlmSettings`).

use super::super::bounds::Bounds;
use serde::{Deserialize, Serialize};

/// Seconds one text request may take.
pub const REQUEST_TIMEOUT_SECONDS: Bounds = Bounds::new(10, 300, 10);
/// Extra attempts after a failed request.
pub const MAX_RETRIES: Bounds = Bounds::new(0, 5, 1);
/// Requests per day; 0 means no limit.
pub const DAILY_REQUESTS: Bounds = Bounds::new(0, 10_000, 50);
/// Tokens per day; 0 means no limit.
pub const DAILY_TOKENS: Bounds = Bounds::new(0, 5_000_000, 50_000);
const DEFAULT_TIMEOUT_SECONDS: u32 = 60;
const DEFAULT_RETRIES: u32 = 2;

/// Model request preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AiPrefs {
    /// Consumed by a later wave.
    pub request_timeout_seconds: u32,
    /// Consumed by a later wave.
    pub max_retries: u32,
    /// Requests per day (0 = no limit). Consumed by a later wave.
    pub daily_request_budget: u32,
    /// Tokens per day (0 = no limit). Consumed by a later wave.
    pub daily_token_cap: u32,
    /// Confirm before a paid request the user started. Consumed by a later wave.
    pub ask_before_spending: bool,
}

impl Default for AiPrefs {
    fn default() -> Self {
        Self {
            request_timeout_seconds: DEFAULT_TIMEOUT_SECONDS,
            max_retries: DEFAULT_RETRIES,
            daily_request_budget: 0,
            daily_token_cap: 0,
            ask_before_spending: true,
        }
    }
}

impl AiPrefs {
    /// Clamps every limit into range.
    pub fn validated(self) -> Self {
        Self {
            request_timeout_seconds: REQUEST_TIMEOUT_SECONDS.clamp_u32(self.request_timeout_seconds),
            max_retries: MAX_RETRIES.clamp_u32(self.max_retries),
            daily_request_budget: DAILY_REQUESTS.clamp_u32(self.daily_request_budget),
            daily_token_cap: DAILY_TOKENS.clamp_u32(self.daily_token_cap),
            ..self
        }
    }
}
