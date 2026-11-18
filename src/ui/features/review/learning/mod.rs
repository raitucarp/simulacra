//! The Learn tab of the Review page (sb E2): four learning tools behind one sub-switch, so
//! the page's tab row keeps four entries (Today, Learn, Statistics, Items) and still fits a
//! compact window.
//!
//! - [`tutor`]: the Socratic tutor (start on a post, thread, document or concept; the
//!   dialogue; its summary and the review items it offers for the gaps).
//! - [`goals`]: learning goals with their progress and forecast, and the weekly review.
//! - [`gaps`]: the knowledge-gap radar and the weakest concepts with one-click actions.
//! - [`palace`]: memory palaces: the editor and the walk-through.
//!
//! The shell computes [`LearningData`] (progress, the radar, the weekly digest) and runs the
//! tutor's requests; this module renders and reports [`LearningEvent`]s.

mod data;
mod events;
mod fields;
pub mod gaps;
pub mod goals;
mod items;
pub mod palace;
mod picker;
mod render;
mod state;
pub mod tutor;

#[cfg(test)]
mod tests;

pub use data::{GoalView, LearningData, TutorStatus};
pub use events::LearningEvent;
pub use fields::LearningFields;
pub use state::{LearnTool, LearningUi};
