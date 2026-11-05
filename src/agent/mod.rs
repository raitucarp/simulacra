//! Autonomous agent dialectic engine module.

pub mod engine;
pub mod llm;

pub use engine::DialecticEngine;
pub use llm::{LlmClient, LlmProvider, LlmSettings};
