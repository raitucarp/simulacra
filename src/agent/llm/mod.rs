//! Multi-provider LLM inference engine supporting Google Gemini, OpenAI, Anthropic, OpenRouter, and Groq.
//!
//! - [`provider`]: the [`LlmProvider`] enum and its metadata table.
//! - [`settings`]: [`LlmSettings`] and its SQLite persistence.
//! - [`chat`]: provider-neutral messages, tool specs, tool calls and replies.
//! - [`wire`]: per-provider request bodies and reply parsing (pure, golden-tested).
//! - [`client`]: [`LlmClient`] and provider dispatch.
//! - [`capability`]: native web search and structured-output strategy per provider.

mod capability;
pub mod chat;
mod client;
mod failure;
mod provider;
mod settings;
pub mod transcribe; // N11: speech to text
pub mod wire;

#[cfg(test)]
mod tests;

pub use capability::SchemaStrategy;
pub use chat::{ChatMessage, ChatReply, ChatRequest, Role, ToolCall, ToolChoice, ToolSpec};
pub use client::LlmClient;
pub(crate) use client::send_json;
pub use failure::{redact_secrets, LlmFailure, MAX_FAILURE_MESSAGE_CHARS};
pub use provider::LlmProvider;
pub use settings::LlmSettings;
pub(crate) use settings::{DEFAULT_MAX_TOKENS, DEFAULT_TEMPERATURE};
