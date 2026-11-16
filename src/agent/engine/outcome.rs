//! The result of one agent generation: the reply plus any LLM failure behind it.

use crate::agent::llm::LlmFailure;
use crate::domain::Post;

/// A generated reply and, when the LLM could not be used, why the heuristic
/// fallback text was substituted.
#[derive(Debug, Clone)]
pub struct SimulationOutcome {
    pub reply: Post,
    /// `Some` when the reply text is the offline heuristic fallback.
    pub llm_error: Option<LlmFailure>,
}
