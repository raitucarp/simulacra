//! Localized wording of the Arena page: formats, sides, roles, rubric criteria, standings,
//! stances, setup and scenario problems, and dates.

use crate::domain::arena::debate::{Criterion, DebateFormat, SetupProblem, Side, TurnRole};
use crate::domain::arena::lab::{ScenarioProblem, Stance};
use crate::domain::thinking::argument::Standing;
use jiff::Timestamp;
use rust_i18n::t;


/// A translated key.
pub fn tr(key: &str) -> String {
    t!(key).to_string()
}

fn keyed(prefix: &str, key: &str) -> String {
    tr(&format!("{prefix}.{key}"))
}

pub fn format_label(format: DebateFormat) -> String {
    keyed("arena.format", format.key())
}

pub fn format_hint(format: DebateFormat) -> String {
    keyed("arena.format_hint", format.key())
}

pub fn side_label(side: Side) -> String {
    keyed("arena.side", side.key())
}

pub fn role_label(role: TurnRole) -> String {
    keyed("arena.role", role.key())
}

pub fn criterion_label(criterion: Criterion) -> String {
    keyed("arena.criterion", criterion.key())
}

pub fn standing_label(standing: Standing) -> String {
    keyed("arena.standing", standing.key())
}

pub fn stance_label(stance: Stance) -> String {
    keyed("arena.stance", stance.key())
}

pub fn setup_problem_text(problem: SetupProblem) -> String {
    keyed("arena.problem", problem.key())
}

pub fn scenario_problem_text(problem: ScenarioProblem) -> String {
    keyed("arena.lab_problem", problem.key())
}

/// A time in the user's local zone.
pub fn stamp(at: Timestamp) -> String {
    crate::ui::format::date_time(at)
}
