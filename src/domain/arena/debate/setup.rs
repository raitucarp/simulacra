//! Whether a setup can start, and the first thing that stops it.

use super::model::{DebateFormat, DebateSetup, Judge, Side};
use crate::domain::document::keyed::keyed_enum;

/// Debaters a debate takes.
pub const MIN_DEBATERS: usize = 2;
pub const MAX_DEBATERS: usize = 4;
/// Rounds a debate may have.
pub const MIN_ROUNDS: u8 = 1;
pub const MAX_ROUNDS: u8 = 5;
pub const DEFAULT_ROUNDS: u8 = 2;
/// Characters of a motion at most.
pub const MAX_MOTION_CHARS: usize = 300;

keyed_enum! {
    /// Why a setup cannot start (the key names its message).
    SetupProblem {
        NoMotion => "no_motion",
        MotionTooLong => "motion_too_long",
        TooFewDebaters => "too_few_debaters",
        TooManyDebaters => "too_many_debaters",
        /// Oxford and Lincoln–Douglas need someone on each side.
        OneSided => "one_sided",
        /// Lincoln–Douglas is one against one.
        NotOneOnOne => "not_one_on_one",
        /// A Socratic round has no sides.
        SidesInSocratic => "sides_in_socratic",
        RoundsOutOfRange => "rounds_out_of_range",
        /// The same agent twice, or the moderator also debating.
        Repeated => "repeated",
    }
}

/// The first problem of `setup`, or `None` when it can start.
pub fn setup_problem(setup: &DebateSetup) -> Option<SetupProblem> {
    let count = setup.debaters.len();
    let sided = !setup.on_side(Side::For).is_empty() && !setup.on_side(Side::Against).is_empty();
    let open = setup.debaters.iter().all(|debater| debater.side == Side::Open);
    let mut speakers = setup.speakers();
    speakers.sort();
    let repeated = speakers.windows(2).any(|pair| pair[0] == pair[1]);
    let checks = [
        (setup.motion.trim().is_empty(), SetupProblem::NoMotion),
        (setup.motion.trim().chars().count() > MAX_MOTION_CHARS, SetupProblem::MotionTooLong),
        (count < MIN_DEBATERS, SetupProblem::TooFewDebaters),
        (count > MAX_DEBATERS, SetupProblem::TooManyDebaters),
        (repeated, SetupProblem::Repeated),
        (!(MIN_ROUNDS..=MAX_ROUNDS).contains(&setup.rounds), SetupProblem::RoundsOutOfRange),
        (setup.format == DebateFormat::LincolnDouglas && count != MIN_DEBATERS, SetupProblem::NotOneOnOne),
        (setup.format != DebateFormat::Socratic && !sided, SetupProblem::OneSided),
        (setup.format == DebateFormat::Socratic && !open, SetupProblem::SidesInSocratic),
    ];
    checks.into_iter().find(|(failed, _)| *failed).map(|(_, problem)| problem)
}

/// The judge that can score: an agent judge that also debates falls back to the global model.
pub fn effective_judge(setup: &DebateSetup) -> Judge {
    match &setup.judge {
        Judge::Agent(agent) if setup.side_of(agent).is_some() => Judge::GlobalModel,
        judge => judge.clone(),
    }
}
