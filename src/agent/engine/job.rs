//! Per-agent engine generations: an [`EngineJob`] runs on a worker thread and yields a
//! [`SimulationOutcome`] (the heuristic reply plus the failure when the engine fails).

use super::outcome::SimulationOutcome;
use super::prompts::PromptContext;
use super::reply::{build_followup, build_reply};
use super::DialecticEngine;
use crate::agent::runtime::{run_engine, EnginePlan, EngineResult};
use crate::agent::tools::ToolContext;
use crate::domain::image_prompt::subject_from_config;
use crate::domain::{AuthoredPost, NetworkId, Profile};
use tokio::sync::oneshot;

/// Name of the dedicated OS thread that runs background agent simulations.
const AGENT_SIM_THREAD_NAME: &str = "simulacra-agent-sim";

/// Everything one generation needs, owned so it can move to the worker thread.
pub struct EngineJob {
    pub parent: AuthoredPost,
    pub agent: Profile,
    pub network: NetworkId,
    pub context: PromptContext,
    pub plan: EnginePlan,
    /// Snapshot for granted tools (`None` when the agent has no tools).
    pub tools: Option<ToolContext>,
    /// Uniform draw in `0..100` for chance-triggered images.
    pub roll: u8,
}

impl DialecticEngine {
    /// Runs `job` on a dedicated worker thread; the receiver errors only if the thread
    /// cannot be spawned.
    pub fn spawn_engine_simulation(job: EngineJob) -> oneshot::Receiver<SimulationOutcome> {
        let (tx, rx) = oneshot::channel();
        let _ = std::thread::Builder::new()
            .name(AGENT_SIM_THREAD_NAME.into())
            .spawn(move || {
                let _ = tx.send(Self::run_engine_job(job));
            });
        rx
    }

    /// Runs `job` on a fresh current-thread Tokio runtime (blocking the caller).
    pub fn run_engine_job(mut job: EngineJob) -> SimulationOutcome {
        job.plan.image_subject = subject_from_config(&job.agent.config);
        let system = Self::system_prompt_in(&job.agent, &job.context);
        let user = Self::persona_user_prompt(&job.parent, &job.agent);
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build();
        let result = match runtime {
            Ok(rt) => rt.block_on(run_engine(&job.plan, &system, &user, job.tools.clone(), job.roll)),
            Err(e) => Err(e.to_string()),
        };
        match result {
            Ok(result) => Self::engine_outcome(job, result),
            Err(raw) => {
                let failure = job.plan.transport.describe_failure(&raw, &job.plan.credentials.secrets());
                let text = Self::generate_heuristic_text(&job.parent, &job.agent);
                let reply = build_reply(&job.parent, job.agent, job.network, text);
                SimulationOutcome { reply, thread: Vec::new(), llm_error: Some(failure), gate_key: job.plan.gate_key, image_error: None }
            }
        }
    }

    /// The reply (with extras and image) and its chained thread parts.
    fn engine_outcome(job: EngineJob, result: EngineResult) -> SimulationOutcome {
        let author = job.agent.id.clone();
        let mut reply = build_reply(&job.parent, job.agent, job.network, result.draft.body());
        reply.image_path = result.image_path.clone();
        reply.extras = result.extras();
        let mut thread = Vec::new();
        for part in result.draft.thread_parts.iter().filter(|part| !part.trim().is_empty()) {
            let previous = thread.last().unwrap_or(&reply);
            thread.push(build_followup(previous, author.clone(), part.trim().to_string()));
        }
        SimulationOutcome { reply, thread, llm_error: None, gate_key: job.plan.gate_key, image_error: result.image_error }
    }
}
