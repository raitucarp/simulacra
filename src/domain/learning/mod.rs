//! Learning tools on top of the review deck (sb E2), pure side.
//!
//! - [`bloom`]: Bloom's levels (remember → create), the ladder the tutor climbs.
//! - [`concept`]: concept names as keys and whether a text mentions one.
//! - [`tutor`]: the Socratic tutor: its dialogue state machine, level progression, prompts,
//!   reply parsing and the post-anchored review items it offers for the gaps.
//! - [`goals`]: learning goals, their linked material, progress metrics and forecast.
//! - [`weekly`]: the weekly review digest, next actions and the reminder rule.
//! - [`gaps`]: the knowledge-gap radar: concept evidence, dimension scores and ranking.
//! - [`palace`]: memory palaces: rooms, loci, placements and walk-throughs graded with FSRS.
//! - [`records`]: every goal, session and palace as loaded from the vault.
//!
//! The effectful side lives at the edges: the tutor's requests go through the post actions
//! runner (`agent::learning`), the tables are `storage::db::learning*`, and the Review page's
//! Learn tab draws everything (`ui::features::review::learning`).

pub mod bloom;
pub mod concept;
pub mod gaps;
pub mod goals;
pub mod palace;
pub mod records;
pub mod tutor;
pub mod weekly;

#[cfg(test)]
mod tests;

pub use bloom::BloomLevel;
pub use gaps::{GapDimension, GapEntry, GapInputs, GapReport};
pub use goals::{Forecast, Goal, GoalId, GoalLink, GoalProgress, GoalTask};
pub use records::LearningRecords;
pub use palace::{plan_walk, Locus, Palace, PalaceId, PlacedTarget, Placement, Room, RoomKind, Walk, WalkScope, WalkStop};
pub use tutor::{Assessment, Exchange, SessionId, TutorPhase, TutorQuestion, TutorSession, TutorSource, TutorSummary, Verdict};
pub use weekly::{NextAction, WeeklyDigest};

/// Prefix of generated learning record ids (goals, sessions, palaces, rooms, tasks).
const ID_SEPARATOR: &str = "_";

/// A fresh random id starting with `prefix`.
pub fn fresh_id(prefix: &str) -> String {
    format!("{prefix}{ID_SEPARATOR}{}", uuid::Uuid::new_v4().simple())
}
