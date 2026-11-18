//! Runs one Arena request through an agent's engine: the same contract as the post actions
//! runner (timeout per attempt, retries with backoff on transient failures, cancel at any
//! point), but sent through the agent runtime, so each speaker uses its own model and
//! request mode. Arena requests use no tools, images or structured-output mapping: the
//! answer is the model's text.

use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::agent::runtime::{run_engine, EnginePlan};
use crate::domain::post_actions::{ActionFailureKind, ActionPrompt, RequestPolicy};

/// The image roll of Arena requests (images never fire, see [`EngineCall::new`]).
const NO_IMAGE_ROLL: u8 = 0;

/// Everything one call needs; moves to the background executor.
pub struct EngineCall {
    pub plan: EnginePlan,
    pub prompt: ActionPrompt,
    pub policy: RequestPolicy,
    pub cancel: CancelToken,
}

impl EngineCall {
    /// A call of `prompt` through `plan`, stripped to plain text: no tools, no image, no
    /// structured output.
    pub fn new(mut plan: EnginePlan, prompt: ActionPrompt, policy: RequestPolicy, cancel: CancelToken) -> Self {
        plan.tools.clear();
        plan.image = None;
        plan.structured = None;
        Self { plan, prompt, policy, cancel }
    }
}

/// [`run_call`] on a private single-thread runtime, for a background thread.
pub fn run_call_blocking(call: &EngineCall) -> Result<String, ActionFailure> {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build();
    match runtime {
        Ok(runtime) => runtime.block_on(run_call(call)),
        Err(err) => Err(ActionFailure { kind: ActionFailureKind::Network, detail: err.to_string() }),
    }
}

/// Sends the call, retrying retryable failures up to the policy's limit.
pub async fn run_call(call: &EngineCall) -> Result<String, ActionFailure> {
    if !call.plan.is_ready() {
        return Err(ActionFailure::local(ActionFailureKind::NoKey));
    }
    let mut attempt = 0;
    loop {
        match attempt_once(call).await {
            Err(failure) if failure.kind.is_retryable() && attempt < call.policy.max_retries => {
                tokio::select! {
                    _ = tokio::time::sleep(call.policy.backoff(attempt)) => attempt += 1,
                    _ = call.cancel.cancelled() => return Err(ActionFailure::local(ActionFailureKind::Cancelled)),
                }
            }
            outcome => return outcome,
        }
    }
}

/// One request, bounded by the policy timeout and the cancel flag.
async fn attempt_once(call: &EngineCall) -> Result<String, ActionFailure> {
    if call.cancel.is_cancelled() {
        return Err(ActionFailure::local(ActionFailureKind::Cancelled));
    }
    let request = tokio::time::timeout(call.policy.timeout, run_engine(&call.plan, &call.prompt.system, &call.prompt.user, None, NO_IMAGE_ROLL));
    tokio::select! {
        outcome = request => match outcome {
            Ok(Ok(result)) if !result.draft.content.trim().is_empty() => Ok(result.draft.content),
            Ok(Ok(_)) => Err(ActionFailure::local(ActionFailureKind::Empty)),
            Ok(Err(raw)) => Err(failure_of(call, &raw)),
            Err(_) => Err(ActionFailure::local(ActionFailureKind::Timeout)),
        },
        _ = call.cancel.cancelled() => Err(ActionFailure::local(ActionFailureKind::Cancelled)),
    }
}

/// A failure classified from the raw error, with every secret redacted from its detail.
fn failure_of(call: &EngineCall, raw: &str) -> ActionFailure {
    let described = call.plan.transport.describe_failure(raw, &call.plan.credentials.secrets());
    ActionFailure { kind: ActionFailureKind::classify(raw), detail: described.message }
}
