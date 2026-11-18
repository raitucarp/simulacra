//! Tests of the Arena page's pure form state and its wording.

use super::panel::{DebateDraft, LabDraft};
use super::text::*;
use crate::domain::arena::debate::{setup_problem, Criterion, DebateFormat, Judge, SetupProblem, Side, TurnRole, MAX_DEBATERS, MAX_ROUNDS};
use crate::domain::arena::lab::{PresetKey, ScenarioProblem, Stance, MAX_VARIABLES};
use crate::domain::thinking::argument::Standing;
use crate::domain::ProfileId;

fn agent(id: &str) -> ProfileId {
    ProfileId::new(id)
}

#[test]
fn new_debaters_balance_the_sides_and_socratic_drops_them() {
    let draft = DebateDraft::default().toggled(&agent("a")).toggled(&agent("b")).toggled(&agent("c"));
    assert_eq!(draft.side_of(&agent("a")), Some(Side::For));
    assert_eq!(draft.side_of(&agent("b")), Some(Side::Against));
    assert_eq!(draft.side_of(&agent("c")), Some(Side::For));
    let socratic = draft.with_format(DebateFormat::Socratic);
    assert!(socratic.debaters.iter().all(|debater| debater.side == Side::Open));
    assert_eq!(socratic.with_format(DebateFormat::Oxford).side_of(&agent("b")), Some(Side::Against));
    assert_eq!(draft.toggled(&agent("a")).side_of(&agent("a")), None, "toggling again takes the agent out");
}

#[test]
fn the_draft_caps_debaters_rounds_and_keeps_the_moderator_apart() {
    let full = (0..MAX_DEBATERS + 2).fold(DebateDraft::default(), |draft, i| draft.toggled(&agent(&format!("a{i}"))));
    assert_eq!(full.debaters.len(), MAX_DEBATERS);
    assert_eq!(DebateDraft::default().with_rounds(99).rounds, MAX_ROUNDS);
    assert_eq!(DebateDraft::default().with_rounds(-99).rounds, 1);
    let chaired = full.with_moderator(Some(agent("a0")));
    assert_eq!(chaired.side_of(&agent("a0")), None);
    let small = DebateDraft::default().toggled(&agent("a")).with_moderator(Some(agent("m")));
    assert_eq!(small.toggled(&agent("m")).moderator, None, "a moderator who joins leaves the chair");
    let setup = DebateDraft::default().toggled(&agent("a")).toggled(&agent("b")).with_judge(Judge::Agent(agent("j"))).setup("  Ban cars ");
    assert_eq!(setup.motion, "Ban cars");
    assert_eq!(setup_problem(&setup), None);
    assert_eq!(setup_problem(&DebateDraft::default().setup("x")), Some(SetupProblem::TooFewDebaters));
}

#[test]
fn the_lab_draft_toggles_agents_and_clamps_rows() {
    let draft = LabDraft::default().toggled(&agent("b")).toggled(&agent("a"));
    assert_eq!(draft.present_agents(&[agent("a"), agent("b"), agent("c")]), vec![agent("a"), agent("b")]);
    assert!(draft.toggled(&agent("a")).agents == vec![agent("b")]);
    assert_eq!(draft.with_variables(9).variables, MAX_VARIABLES);
    assert_eq!(draft.with_variables(-9).variables, 1);
}

/// Whether `label` is a translation rather than the missing key echoed back.
fn translated(label: &str) -> bool {
    !label.is_empty() && !label.contains("arena.")
}

#[test]
fn every_label_of_the_page_is_translated() {
    let labels = DebateFormat::ALL.iter().flat_map(|f| [format_label(*f), format_hint(*f)])
        .chain(Side::ALL.iter().map(|s| side_label(*s)))
        .chain(TurnRole::ALL.iter().map(|r| role_label(*r)))
        .chain(Criterion::ALL.iter().map(|c| criterion_label(*c)))
        .chain(Standing::ALL.iter().map(|s| standing_label(*s)))
        .chain(Stance::ALL.iter().map(|s| stance_label(*s)))
        .chain(SetupProblem::ALL.iter().map(|p| setup_problem_text(*p)))
        .chain(ScenarioProblem::ALL.iter().map(|p| scenario_problem_text(*p)))
        .chain(PresetKey::ALL.iter().flat_map(|p| p.locale_keys()).map(|key| tr(&key)));
    for label in labels {
        assert!(translated(&label), "missing translation: {label}");
    }
}
