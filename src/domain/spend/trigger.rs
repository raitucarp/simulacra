//! What a manual agent run (A4) will likely cost, shown before it starts. No price list is
//! kept, so the estimate is in requests and tokens: the prompt's tokens counted the rough way
//! (characters / 4), and at most the reply cap back per request. With tools, the agent may
//! send up to one request per tool step. Without a key or address, nothing is sent: the
//! agent writes from its own phrases.

use crate::domain::embedding::estimate_tokens;

/// The estimate of one run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriggerEstimate {
    /// Requests the run sends at most (0 when no model is set up).
    pub requests: u32,
    /// Tokens sent with the first request (about).
    pub tokens_in: u32,
    /// Tokens the replies may hold at most, over every request.
    pub tokens_out: u32,
}

impl TriggerEstimate {
    /// Whether the run sends anything to a model.
    pub const fn uses_model(self) -> bool {
        self.requests > 0
    }
}

/// The estimate of a run whose prompt is `prompt` (system and user text), with replies capped
/// at `max_tokens`, up to `tool_steps` tool steps (0 without tools), when a model `ready`.
pub fn trigger_estimate(prompt: &[&str], max_tokens: u32, tool_steps: u8, ready: bool) -> TriggerEstimate {
    if !ready {
        return TriggerEstimate { requests: 0, tokens_in: 0, tokens_out: 0 };
    }
    let requests = u32::from(tool_steps.max(1));
    TriggerEstimate { requests, tokens_in: estimate_tokens(prompt.iter().copied()), tokens_out: max_tokens.saturating_mul(requests) }
}
