//! Autonomous agent dialectic engine module.

pub mod credentials;
pub mod custom_request;
pub mod embedding;
pub mod engine;
pub mod image;
pub mod llm;
#[cfg(test)]
pub(crate) mod mock_server;
pub mod post_actions;
pub mod runtime;
pub mod schedule;
pub mod template;
pub mod tools;

pub use engine::{DialecticEngine, EngineJob, PromptContext, SimulationOutcome};
pub use llm::{LlmClient, LlmFailure, LlmProvider, LlmSettings};
