//! The debate arena: a motion, two to four agents with sides, a format and rounds, an
//! optional moderator; turns spoken through each agent's own engine and dossier; a judge's
//! rubric scores; and the argument map of the transcript with the claims that still stand.
//!
//! - [`model`]: formats, sides, roles, the setup, slots and turns.
//! - [`setup`]: validation; [`schedule`]: the speaking order of each format.
//! - [`record`]: a saved debate and its phase (a stopped debate resumes at its next slot).
//! - [`transcript`], [`prompt`]: the transcript as quoted data and the turn prompt.
//! - [`judge`]: the rubric, the judge's prompt and the verdict reader.
//! - [`claims`]: the transcript as argument-map input, and claim standings.

pub mod claims;
pub mod judge;
pub mod model;
pub mod prompt;
pub mod record;
pub mod schedule;
pub mod setup;
pub mod transcript;

pub use claims::{claim_standings, turn_posts, ClaimStanding};
pub use judge::{judge_prompt, judge_task, parse_verdict, Criterion, Score, Verdict, JUDGE_ANSWER_TOKENS, MAX_SCORE};
pub use model::{DebateFormat, DebateSetup, Debater, Judge, Side, Slot, Turn, TurnRole};
pub use prompt::{turn_prompt, TURN_ANSWER_TOKENS};
pub use record::{Debate, Phase};
pub use schedule::schedule;
pub use setup::{effective_judge, setup_problem, SetupProblem, DEFAULT_ROUNDS, MAX_DEBATERS, MAX_ROUNDS, MIN_DEBATERS, MIN_ROUNDS};
