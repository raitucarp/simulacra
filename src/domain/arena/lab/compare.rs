//! Comparing the answers, pure:
//!
//! - **Disagreements:** cells where the agents' stances differ.
//! - **Agreement between agents:** for each pair, the share of cells both answered alike.
//! - **What flips answers:** for each variable, over every agent and every pair of cells
//!   that differ only in that variable, how often the agent's stance changed. The variable
//!   with the most flips is the one the agents' answers turn on.

use super::record::{Experiment, Stance};
use super::scenario::cells;
use crate::domain::ProfileId;

/// How often two agents agreed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairAgreement {
    pub first: ProfileId,
    pub second: ProfileId,
    pub agreed: usize,
    pub compared: usize,
}

/// How often changing one variable changed an answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VariableFlips {
    pub variable: usize,
    pub flips: usize,
    pub comparisons: usize,
}

fn stance(experiment: &Experiment, agent: &ProfileId, cell: usize) -> Option<Stance> {
    experiment.answer(agent, cell).map(|answer| answer.stance)
}

/// Cells whose answered stances are not all the same.
pub fn disagreements(experiment: &Experiment) -> Vec<usize> {
    (0..cells(&experiment.scenario).len())
        .filter(|cell| {
            let mut stances = experiment.agents.iter().filter_map(|agent| stance(experiment, agent, *cell));
            stances.next().is_some_and(|first| stances.any(|other| other != first))
        })
        .collect()
}

/// Agreement of every pair of agents, in agent order.
pub fn pairwise(experiment: &Experiment) -> Vec<PairAgreement> {
    let count = cells(&experiment.scenario).len();
    let agents = &experiment.agents;
    (0..agents.len())
        .flat_map(|i| (i + 1..agents.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            let both: Vec<(Stance, Stance)> = (0..count).filter_map(|cell| Some((stance(experiment, &agents[i], cell)?, stance(experiment, &agents[j], cell)?))).collect();
            let agreed = both.iter().filter(|(a, b)| a == b).count();
            PairAgreement { first: agents[i].clone(), second: agents[j].clone(), agreed, compared: both.len() }
        })
        .collect()
}

/// Pairs of cells that differ only in `variable`.
fn neighbours(grid: &[Vec<usize>], variable: usize) -> Vec<(usize, usize)> {
    let differs_only_in = |a: &[usize], b: &[usize]| a.iter().zip(b).enumerate().all(|(index, (x, y))| (index == variable) != (x == y));
    (0..grid.len()).flat_map(|i| (i + 1..grid.len()).map(move |j| (i, j))).filter(|(i, j)| differs_only_in(&grid[*i], &grid[*j])).collect()
}

/// Flips per variable, the variable that flips most first.
pub fn flips(experiment: &Experiment) -> Vec<VariableFlips> {
    let grid = cells(&experiment.scenario);
    let mut counts: Vec<VariableFlips> = (0..experiment.scenario.variables.len())
        .map(|variable| {
            let pairs = neighbours(&grid, variable);
            let observed: Vec<bool> = experiment
                .agents
                .iter()
                .flat_map(|agent| pairs.iter().filter_map(move |(a, b)| Some(stance(experiment, agent, *a)? != stance(experiment, agent, *b)?)))
                .collect();
            VariableFlips { variable, flips: observed.iter().filter(|flipped| **flipped).count(), comparisons: observed.len() }
        })
        .collect();
    counts.sort_by(|a, b| b.flips.cmp(&a.flips).then(a.variable.cmp(&b.variable)));
    counts
}
