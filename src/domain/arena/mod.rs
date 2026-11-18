//! The Arena (sb F1): agents debating a motion, and agents answering thought experiments.
//!
//! - [`debate`]: setup, speaking order, turn prompts, judging and claim standings.
//! - [`lab`]: scenarios, the grid run, budget preview, comparison and presets.
//! - [`publish`]: a debate as a thread and an experiment as a summary post.
//! - [`reply`]: lenient readers of the models' JSON replies.
//!
//! Everything here is pure. Agents see the motion, the transcript or the scenario and their
//! own dossier; the private journal, mood, focus and capsule records never reach a prompt.

pub mod debate;
pub mod ids;
pub mod lab;
pub mod publish;
pub mod reply;

#[cfg(test)]
mod tests;

pub use ids::ArenaId;
