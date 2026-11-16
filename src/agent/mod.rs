//! Autonomous agent dialectic engine module.

pub mod engine;
pub mod llm;
pub mod schedule;

pub use engine::{DialecticEngine, SimulationOutcome};
pub use llm::{LlmClient, LlmFailure, LlmProvider, LlmSettings};
