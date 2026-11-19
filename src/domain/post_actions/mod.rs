//! Post actions (sb C2), pure side: thinking tools a user runs on one thought.
//!
//! - [`kind`]: the [`ActionKind`] enum, its menu groups and the choice each action needs.
//! - [`request`]: an [`ActionRequest`] (kind plus audience, language or concept).
//! - [`context`]: the post, its author and its thread as plain data.
//! - [`concepts`]: concept candidates of a post for "search concept".
//! - [`prompt`]: the system and user prompt of a request.
//! - [`parse`] / [`output`]: the answer as display blocks and as plain text.
//! - [`failure`]: why a request failed, what is retried, and the request policy.
//!
//! The effectful runner (HTTP, timeout, retries, cancel) is `agent::post_actions`.

pub mod concepts;
pub mod context;
pub mod failure;
pub mod kind;
pub mod output;
pub mod parse;
pub mod prompt;
pub mod request;

#[cfg(test)]
mod tests;

pub use concepts::{extract_concepts, MAX_CONCEPTS};
pub use context::{ActionContext, AuthorContext, ThreadLine};
pub use failure::{ActionFailureKind, RequestPolicy};
pub use kind::{ActionGroup, ActionInput, ActionKind};
pub use output::{ActionOutput, ConceptRelation, OutputBlock};
pub use parse::parse_output;
pub use prompt::{build_prompt, ActionPrompt};
pub use request::{ActionRequest, Audience, AudiencePreset, TargetLanguage};
