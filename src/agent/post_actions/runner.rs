//! Runs one action request: timeout per attempt, retries with backoff on transient failures,
//! cancel at any point (during a request or a backoff wait).

use super::cancel::CancelToken;
use crate::agent::LlmClient;
use crate::domain::post_actions::{ActionFailureKind, ActionPrompt, RequestPolicy};

/// Everything one run needs; moves to the background executor.
pub struct ActionJob {
    pub client: LlmClient,
    pub prompt: ActionPrompt,
    pub policy: RequestPolicy,
    pub cancel: CancelToken,
}

/// Why a run ended without text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionFailure {
    pub kind: ActionFailureKind,
    /// Short provider message with every key redacted (empty for local reasons).
    pub detail: String,
}

impl ActionFailure {
    pub fn local(kind: ActionFailureKind) -> Self {
        Self { kind, detail: String::new() }
    }
}

/// [`run_action`] on a private single-thread runtime (the HTTP client needs one), for a
/// background thread.
pub fn run_action_blocking(job: &ActionJob) -> Result<String, ActionFailure> {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build();
    match runtime {
        Ok(runtime) => runtime.block_on(run_action(job)),
        Err(err) => Err(ActionFailure { kind: ActionFailureKind::Network, detail: err.to_string() }),
    }
}

/// Sends the job's prompt, retrying retryable failures up to the policy's limit.
pub async fn run_action(job: &ActionJob) -> Result<String, ActionFailure> {
    if !job.client.has_api_key() {
        return Err(ActionFailure::local(ActionFailureKind::NoKey));
    }
    let mut attempt = 0;
    loop {
        match attempt_once(job).await {
            Err(failure) if failure.kind.is_retryable() && attempt < job.policy.max_retries => {
                tokio::select! {
                    _ = tokio::time::sleep(job.policy.backoff(attempt)) => attempt += 1,
                    _ = job.cancel.cancelled() => return Err(ActionFailure::local(ActionFailureKind::Cancelled)),
                }
            }
            outcome => return outcome,
        }
    }
}

/// One request, bounded by the policy timeout and the cancel flag.
async fn attempt_once(job: &ActionJob) -> Result<String, ActionFailure> {
    if job.cancel.is_cancelled() {
        return Err(ActionFailure::local(ActionFailureKind::Cancelled));
    }
    let request = tokio::time::timeout(job.policy.timeout, job.client.generate(&job.prompt.system, &job.prompt.user));
    tokio::select! {
        outcome = request => match outcome {
            Ok(Ok(text)) if !text.trim().is_empty() => Ok(text),
            Ok(Ok(_)) => Err(ActionFailure::local(ActionFailureKind::Empty)),
            Ok(Err(raw)) => Err(ActionFailure { kind: ActionFailureKind::classify(&raw), detail: job.client.describe_failure(&raw).message }),
            Err(_) => Err(ActionFailure::local(ActionFailureKind::Timeout)),
        },
        _ = job.cancel.cancelled() => Err(ActionFailure::local(ActionFailureKind::Cancelled)),
    }
}
