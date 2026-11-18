//! Post actions (sb C2), effectful side: sends an action prompt through the configured
//! [`LlmClient`](crate::agent::LlmClient) with the request timeout, retries with backoff on
//! transient failures, and stops at once when the user cancels. Prompts and parsing are pure
//! (`domain::post_actions`).

mod cancel;
mod runner;

#[cfg(test)]
mod tests;

pub use cancel::CancelToken;
pub use runner::{run_action, run_action_blocking, ActionFailure, ActionJob};
