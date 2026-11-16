//! Autonomous dialectic reasoning engine for human-like network agents.
//!
//! Generates context-aware philosophical, technical, and dialectic syntheses
//! responding to network thoughts and grounding documents while honoring each
//! agent's full psychological, linguistic, and behavioral configuration.
//!
//! - [`prompts`]: system/user prompt assembly.
//! - [`run`]: LLM-backed generation (async, blocking, background thread).
//! - [`heuristic`]: offline canned replies used as the LLM fallback.
//! - [`reply`]: the single reply-`Post` constructor shared by every path.
//! - [`outcome`]: a reply plus the LLM failure (if any) that forced the fallback.

mod heuristic;
mod outcome;
mod prompts;
mod reply;
mod run;
mod voices;

#[cfg(test)]
mod tests;

pub use outcome::SimulationOutcome;

/// Heuristic and LLM-powered Dialectic Reasoner for human-like agent synthesis.
pub struct DialecticEngine;
