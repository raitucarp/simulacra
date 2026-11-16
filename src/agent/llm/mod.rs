//! Multi-provider LLM inference engine supporting Google Gemini, OpenAI, Anthropic, OpenRouter, and Groq.
//!
//! - [`provider`]: the [`LlmProvider`] enum and its metadata table.
//! - [`settings`]: [`LlmSettings`] and its SQLite persistence.
//! - [`client`]: [`LlmClient`] and provider dispatch; one backend module per wire protocol.

mod anthropic;
mod client;
mod gemini;
mod openai;
mod provider;
mod settings;

#[cfg(test)]
mod tests;

pub use client::LlmClient;
pub use provider::LlmProvider;
pub use settings::LlmSettings;
