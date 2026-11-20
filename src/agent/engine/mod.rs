//! Autonomous dialectic reasoning engine for human-like network agents.
//!
//! Generates context-aware philosophical, technical, and dialectic syntheses
//! responding to network thoughts and grounding documents while honoring each
//! agent's full psychological, linguistic, and behavioral configuration.
//!
//! - [`prompts`]: system/user prompt assembly.
//! - [`run`]: LLM-backed generation (async, blocking, background thread).
//! - [`job`]: per-agent engine jobs (tools, structured output, images) on a worker thread.
//! - [`kind`]: what a job generates, a reply or a new top-level post from a seed.
//! - [`heuristic`]: offline canned posts used as the LLM fallback.
//! - [`reply`]: the reply, root-post and follow-up constructors shared by every path.
//! - [`outcome`]: a post plus the LLM failure (if any) that forced the fallback.

mod heuristic;
mod job;
mod kind;
mod outcome;
mod prompts;
mod reply;
mod run;
mod voices;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_root;

pub use job::EngineJob;
pub use kind::JobKind;
pub use outcome::SimulationOutcome;
pub use prompts::PromptContext;

/// Heuristic and LLM-powered Dialectic Reasoner for human-like agent synthesis.
pub struct DialecticEngine;
