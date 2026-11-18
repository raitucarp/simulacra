//! The Arena page (sb F1): agents debating a motion, and agents answering thought experiments.
//!
//! - Debate: the setup (motion, format, debaters and sides, rounds, moderator, judge), the
//!   transcript as it is spoken, the judge's rubric scores, the claims that still stand, and
//!   publishing the debate as a thread. Debates are saved and resume where they stopped.
//! - Thought lab: classic presets or an own scenario, the agents to ask, a budget preview,
//!   the grid of answers, the comparison, and a summary post.
//!
//! [`ArenaPanel`] is the page entity; the shell runs every request (through each agent's own
//! engine and the shared daily AI budget) and hands back [`ArenaData`].

pub mod data;
pub mod events;
pub mod kit;
mod panel;
mod render;
pub mod tab;
pub mod text;

#[cfg(test)]
mod tests;

pub use data::{AgentChoice, ArenaData, OpenThread, RunState};
pub use events::ArenaEvent;
pub use panel::{ArenaPanel, DebateDraft, LabDraft};
pub use tab::ArenaTab;
