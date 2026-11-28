//! The live suite, by tier (pick tiers with `SIMULACRA_LIVE_TIERS`, default `a,b,d`):
//!
//! - [`tier_a`], [`tier_a_embed`], [`tier_a_thinking`]: text through every app path (chat,
//!   structured output, engine jobs), embeddings, and the thinking tools (post actions with
//!   their parsers, the argument map, the tutor, the arena).
//! - [`tier_b`]: tools and transports (tool loop, emit tool, Google Search grounding, a custom
//!   raw request, Gemini's OpenAI-compatible endpoint).
//! - [`tier_c`]: media, opt-in because it costs more (images, an illustrated post, audio).
//! - [`tier_d`]: real error classification and redaction (fake key, unknown model).
//!
//! [`harness`] reads the configuration and runs each case; [`report`] collects one row per
//! case into `target/live-report.md`; [`fixtures`] builds agents, posts, plans and stores in
//! memory (never from the user's database).

mod fixtures;
mod harness;
mod report;
mod tier_a;
mod tier_a_embed;
mod tier_a_thinking;
mod tier_b;
mod tier_c;
mod tier_d;
