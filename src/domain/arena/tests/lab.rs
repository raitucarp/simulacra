use super::{agent, now};
use crate::domain::arena::lab::*;
use crate::domain::arena::ArenaId;
use crate::domain::embedding::{BudgetLimits, DailyUsage};

fn scenario() -> Scenario {
    Scenario {
        title: "Trolley".into(),
        premise: "A runaway trolley.".into(),
        variables: vec![
            Variable { name: "Means".into(), values: vec!["lever".into(), "push".into()] },
            Variable { name: "Count".into(), values: vec!["five".into(), "two".into(), "one".into()] },
        ],
        question: "Do you act?".into(),
    }
}

fn answered(rows: &[(&str, usize, Stance)]) -> Experiment {
    let mut experiment = Experiment::new(ArenaId::new("x"), scenario(), vec![agent("a"), agent("b")], None, now());
    for (who, cell, stance) in rows {
        experiment = experiment.with_answer(CellAnswer { agent: agent(who), cell: *cell, stance: *stance, verdict: "v".into(), reasoning: String::new() }, now());
    }
    experiment
}

#[test]
fn values_parse_and_the_grid_is_row_major() {
    assert_eq!(parse_values(" lever |push|| Lever | "), vec!["lever", "push"]);
    let grid = cells(&scenario());
    assert_eq!(grid.len(), 6);
    assert_eq!(grid[1], vec![0, 1]);
    assert_eq!(grid[3], vec![1, 0]);
    assert_eq!(conditions(&scenario(), &grid[5]), vec![("Means", "push"), ("Count", "one")]);
}

#[test]
fn scenario_problems_are_found() {
    assert_eq!(scenario_problem(&scenario()), None);
    let mut one_value = scenario();
    one_value.variables[0].values.truncate(1);
    assert_eq!(scenario_problem(&one_value), Some(ScenarioProblem::TooFewValues));
    let none = Scenario { variables: Vec::new(), ..scenario() };
    assert_eq!(scenario_problem(&none), Some(ScenarioProblem::NoVariables));
    let blank = Scenario { question: " ".into(), ..scenario() };
    assert_eq!(scenario_problem(&blank), Some(ScenarioProblem::NoQuestion));
}

#[test]
fn the_preview_counts_requests_and_checks_todays_budget() {
    let limits = BudgetLimits { daily_requests: 10, daily_tokens: 0 };
    let usage = DailyUsage { day: "2026-11-18".into(), requests: 5, tokens: 0 };
    let run = preview(&scenario(), 2, 12, &usage, limits, "2026-11-18");
    assert_eq!((run.cells, run.requests, run.requests_left), (6, 12, Some(5)));
    assert!(run.tokens > 12 * CELL_ANSWER_TOKENS);
    assert!(!run.fits(), "12 requests do not fit in the 5 left");
    let tomorrow = preview(&scenario(), 2, 4, &usage, limits, "2026-11-19");
    assert!(tomorrow.fits() && tomorrow.tokens_left.is_none());
    assert!(preview(&scenario(), 9, 54, &usage, BudgetLimits { daily_requests: 0, daily_tokens: 0 }, "x").over_cap());
}

#[test]
fn answers_are_read_leniently() {
    let raw = "Sure!\n{\"stance\": \"Permissible\", \"verdict\": \"Pull it.\", \"reasoning\": \"Fewer die.\"}";
    assert_eq!(parse_cell_answer(raw), Some((Stance::Yes, "Pull it.".into(), "Fewer die.".into())));
    assert_eq!(parse_cell_answer("{\"stance\":\"perhaps\",\"verdict\":\"x\"}"), None);
    assert!(cell_prompt(&scenario(), &[1, 2], "Japanese").contains("- Count: one"));
}

#[test]
fn pending_and_rerun() {
    let experiment = answered(&[("a", 0, Stance::Yes)]);
    assert_eq!(experiment.pending().len(), 11);
    assert_eq!(experiment.pending()[0], (agent("b"), 0));
    assert!(experiment.cleared(now()).answers.is_empty());
}

#[test]
fn comparison_finds_disagreements_agreement_and_the_pivot_variable() {
    use Stance::{No, Yes};
    let rows = [("a", 0, Yes), ("a", 1, Yes), ("a", 3, No), ("a", 4, No), ("b", 0, Yes), ("b", 1, No), ("b", 3, No), ("b", 4, Yes)];
    let experiment = answered(&rows);
    assert_eq!(disagreements(&experiment), vec![1, 4]);
    let pair = &pairwise(&experiment)[0];
    assert_eq!((pair.agreed, pair.compared), (2, 4));
    let ranked = flips(&experiment);
    assert_eq!(ranked[0].variable, 0, "changing the means flips most answers");
    assert_eq!((ranked[0].flips, ranked[0].comparisons), (4, 4));
    assert_eq!((ranked[1].flips, ranked[1].comparisons), (2, 4));
}

#[test]
fn presets_build_from_the_translator() {
    assert_eq!(PresetKey::ALL.len(), 8);
    let scenario = preset_scenario(PresetKey::Trolley, |key| if key.ends_with("_values") { "a | b".into() } else { key.to_string() });
    assert_eq!(scenario.title, "arena.preset.trolley.title");
    assert_eq!(scenario.variables.len(), 2);
    assert_eq!(scenario.variables[1].values, vec!["a", "b"]);
    assert_eq!(scenario_problem(&scenario), None);
    assert_eq!(PresetKey::Newcomb.locale_keys().len(), 7);
}
