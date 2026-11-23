//! Post actions (sb C2), effectful side: sends an action prompt through the configured
//! [`LlmClient`](crate::agent::LlmClient) with the request timeout, retries with backoff on
//! transient failures, and stops at once when the user cancels. Prompts and parsing are pure
//! (`domain::post_actions`).

mod cancel;
mod runner;
mod transcribe; // N11

#[cfg(test)]
mod tests;
#[cfg(test)]
mod thinking_tests;
#[cfg(test)]
mod depth_tests; // N5
#[cfg(test)]
mod transcribe_tests; // N11
#[cfg(test)]
mod recheck_tests; // N13

pub use cancel::CancelToken;
pub use runner::{run_action, run_action_blocking, ActionFailure, ActionJob};
pub use transcribe::{run_transcription, run_transcription_blocking, TranscribeJob};
