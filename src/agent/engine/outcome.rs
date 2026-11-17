//! The result of one agent generation: the reply plus any LLM failure behind it.

use crate::agent::llm::LlmFailure;
use crate::domain::Post;

/// A generated reply and, when the LLM could not be used, why the heuristic
/// fallback text was substituted.
#[derive(Debug, Clone)]
pub struct SimulationOutcome {
    pub reply: Post,
    /// Follow-up posts chained under `reply` (structured thread parts), in order.
    pub thread: Vec<Post>,
    /// `Some` when the reply text is the offline heuristic fallback.
    pub llm_error: Option<LlmFailure>,
    /// Failure-gate key of the engine that ran (credential, provider or custom URL).
    pub gate_key: String,
    /// Why a triggered image could not be generated (redacted); the post was kept.
    pub image_error: Option<String>,
}
