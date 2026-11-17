//! The per-agent engine runtime: resolves an agent's engine into a plan, runs the tool loop
//! and structured output over the right transport, and illustrates the result.
//!
//! - [`plan`]: [`EnginePlan`] from `EngineConfig` + global settings + credentials.
//! - [`transport`]: native provider client or custom raw request.
//! - [`converse`]: the bounded tool loop.
//! - [`structured`] / [`draft`]: JSON prompting, extraction, repair and mapping to a post.
//! - [`execute`]: [`run_engine`], one full generation.

pub mod converse;
pub mod draft;
mod execute;
pub mod plan;
pub mod structured;
pub mod transport;

#[cfg(test)]
mod tests;

pub use draft::PostDraft;
pub use execute::{build_request, run_engine, EngineResult};
pub use plan::{EnginePlan, GLOBAL_GATE_KEY};
pub use transport::{CustomTransport, Transport};
