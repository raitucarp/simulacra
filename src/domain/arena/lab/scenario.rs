//! A thought experiment's scenario: a premise, variables with two to four values each, and a
//! question. The lab runs every combination of values (the grid's cells, in row-major
//! order: the last variable changes fastest).

use crate::domain::document::keyed::keyed_enum;
use serde::{Deserialize, Serialize};

pub const MIN_VALUES: usize = 2;
pub const MAX_VALUES: usize = 4;
pub const MAX_VARIABLES: usize = 3;
/// Separates values typed in one field ("lever | push").
pub const VALUE_SEPARATOR: char = '|';
/// Characters of the premise and of the question at most.
pub const MAX_PREMISE_CHARS: usize = 1_500;
pub const MAX_QUESTION_CHARS: usize = 300;

/// One variable and the values it takes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub values: Vec<String>,
}

/// The scenario.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Scenario {
    pub title: String,
    pub premise: String,
    pub variables: Vec<Variable>,
    pub question: String,
}

keyed_enum! {
    /// Why a scenario cannot run (the key names its message).
    ScenarioProblem {
        NoPremise => "no_premise",
        PremiseTooLong => "premise_too_long",
        NoQuestion => "no_question",
        QuestionTooLong => "question_too_long",
        NoVariables => "no_variables",
        TooManyVariables => "too_many_variables",
        UnnamedVariable => "unnamed_variable",
        TooFewValues => "too_few_values",
        TooManyValues => "too_many_values",
    }
}

/// The values typed in one field, trimmed, blanks and repeats dropped.
pub fn parse_values(text: &str) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    for value in text.split(VALUE_SEPARATOR).map(str::trim).filter(|value| !value.is_empty()) {
        if !values.iter().any(|kept| kept.eq_ignore_ascii_case(value)) {
            values.push(value.to_string());
        }
    }
    values
}

/// The values joined back for editing.
pub fn joined_values(values: &[String]) -> String {
    values.join(&format!(" {VALUE_SEPARATOR} "))
}

/// The first problem of `scenario`, or `None` when it can run.
pub fn scenario_problem(scenario: &Scenario) -> Option<ScenarioProblem> {
    let chars = |text: &str| text.trim().chars().count();
    let values = |test: fn(usize) -> bool| scenario.variables.iter().any(|variable| test(variable.values.len()));
    let checks = [
        (chars(&scenario.premise) == 0, ScenarioProblem::NoPremise),
        (chars(&scenario.premise) > MAX_PREMISE_CHARS, ScenarioProblem::PremiseTooLong),
        (chars(&scenario.question) == 0, ScenarioProblem::NoQuestion),
        (chars(&scenario.question) > MAX_QUESTION_CHARS, ScenarioProblem::QuestionTooLong),
        (scenario.variables.is_empty(), ScenarioProblem::NoVariables),
        (scenario.variables.len() > MAX_VARIABLES, ScenarioProblem::TooManyVariables),
        (scenario.variables.iter().any(|variable| chars(&variable.name) == 0), ScenarioProblem::UnnamedVariable),
        (values(|count| count < MIN_VALUES), ScenarioProblem::TooFewValues),
        (values(|count| count > MAX_VALUES), ScenarioProblem::TooManyValues),
    ];
    checks.into_iter().find(|(failed, _)| *failed).map(|(_, problem)| problem)
}

/// Every cell: one value index per variable, in row-major order.
pub fn cells(scenario: &Scenario) -> Vec<Vec<usize>> {
    scenario.variables.iter().fold(vec![Vec::new()], |cells, variable| {
        cells.iter().flat_map(|cell| (0..variable.values.len()).map(move |value| [cell.as_slice(), &[value]].concat())).collect()
    })
}

/// `(variable, value)` names of `cell`.
pub fn conditions<'a>(scenario: &'a Scenario, cell: &[usize]) -> Vec<(&'a str, &'a str)> {
    scenario.variables.iter().zip(cell).filter_map(|(variable, index)| Some((variable.name.as_str(), variable.values.get(*index)?.as_str()))).collect()
}

/// One variable row as typed: a name and its values in one field.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VariableDraft {
    pub name: String,
    pub values: String,
}

/// The scenario the form holds.
pub fn scenario_from(title: &str, premise: &str, question: &str, variables: &[VariableDraft]) -> Scenario {
    let variables = variables.iter().map(|draft| Variable { name: draft.name.trim().to_string(), values: parse_values(&draft.values) }).collect();
    Scenario { title: title.trim().to_string(), premise: premise.trim().to_string(), variables, question: question.trim().to_string() }
}
