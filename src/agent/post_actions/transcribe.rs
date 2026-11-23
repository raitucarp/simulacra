//! N11: a transcription through the post actions policy: the request timeout per attempt,
//! retries with backoff on transient failures, and the cancel flag at any point.

use super::cancel::CancelToken;
use super::runner::ActionFailure;
use crate::agent::llm::transcribe::AudioClip;
use crate::agent::LlmClient;
use crate::domain::post_actions::{ActionFailureKind, RequestPolicy};

/// Everything one transcription needs; moves to the background executor.
pub struct TranscribeJob {
    pub client: LlmClient,
    pub clip: AudioClip,
    pub policy: RequestPolicy,
    pub cancel: CancelToken,
}

async fn attempt_once(job: &TranscribeJob) -> Result<String, ActionFailure> {
    if job.cancel.is_cancelled() {
        return Err(ActionFailure::local(ActionFailureKind::Cancelled));
    }
    let request = tokio::time::timeout(job.policy.timeout, job.client.transcribe(&job.clip));
    tokio::select! {
        outcome = request => match outcome {
            Ok(Ok(text)) => Ok(text),
            Ok(Err(raw)) => Err(ActionFailure { kind: ActionFailureKind::classify(&raw), detail: job.client.describe_failure(&raw).message }),
            Err(_) => Err(ActionFailure::local(ActionFailureKind::Timeout)),
        },
        _ = job.cancel.cancelled() => Err(ActionFailure::local(ActionFailureKind::Cancelled)),
    }
}

/// Sends the clip, retrying retryable failures up to the policy's limit.
pub async fn run_transcription(job: &TranscribeJob) -> Result<String, ActionFailure> {
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

/// [`run_transcription`] on a private single-thread runtime, for a background thread.
pub fn run_transcription_blocking(job: &TranscribeJob) -> Result<String, ActionFailure> {
    match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(runtime) => runtime.block_on(run_transcription(job)),
        Err(err) => Err(ActionFailure { kind: ActionFailureKind::Network, detail: err.to_string() }),
    }
}
