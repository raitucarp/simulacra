//! The summary post of an experiment, pure: its title and question, each agent's stances
//! counted over the grid, and the variable the answers turn on. The caller passes the words
//! already translated, so the post is written in the user's language.

use super::compare::flips;
use super::record::{Experiment, Stance};
use crate::domain::ProfileId;

/// Translated words of the summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryWords {
    /// The opening line, title included.
    pub heading: String,
    /// The label of each stance, in `Stance::ALL` order.
    pub stances: Vec<String>,
    /// "Answers turn on: {variable}", with `{variable}` to fill in.
    pub turns_on: String,
    /// Said when no variable changed an answer.
    pub steady: String,
}

const VARIABLE_SLOT: &str = "{variable}";
const PART_SEPARATOR: &str = " · ";

/// One agent's line: `Ada: yes 2 · no 2`.
fn agent_line(experiment: &Experiment, agent: &ProfileId, name: &str, words: &SummaryWords) -> String {
    let counts: Vec<String> = Stance::ALL
        .iter()
        .zip(&words.stances)
        .filter_map(|(stance, label)| {
            let count = experiment.answers.iter().filter(|answer| &answer.agent == agent && answer.stance == *stance).count();
            (count > 0).then(|| format!("{label} {count}"))
        })
        .collect();
    format!("{name}: {}", counts.join(PART_SEPARATOR))
}

/// The summary text; `name` gives an agent's display name.
pub fn summary_text(experiment: &Experiment, words: &SummaryWords, name: &dyn Fn(&ProfileId) -> String) -> String {
    let lines: Vec<String> = experiment.agents.iter().map(|agent| agent_line(experiment, agent, &name(agent), words)).collect();
    let top = flips(experiment).into_iter().next().filter(|top| top.flips > 0);
    let pivot = match top.and_then(|top| experiment.scenario.variables.get(top.variable)) {
        Some(variable) => words.turns_on.replace(VARIABLE_SLOT, &variable.name),
        None => words.steady.clone(),
    };
    format!("{}\n{}\n\n{}\n\n{pivot}", words.heading, experiment.scenario.question.trim(), lines.join("\n"))
}
