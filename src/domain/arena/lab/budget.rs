//! The lab's own budget preview, before anything is sent: one request per agent and cell,
//! with an estimate of the tokens, checked against what today's shared AI budget has left.
//! A grid larger than [`MAX_LAB_REQUESTS`] is refused outright.

use super::prompt::{cell_prompt_estimate, CELL_ANSWER_TOKENS};
use super::scenario::{cells, Scenario};
use crate::domain::embedding::{BudgetLimits, DailyUsage};

/// Requests one run may make at most.
pub const MAX_LAB_REQUESTS: u32 = 48;
/// Estimated tokens of the agent's dossier added to each request.
pub const DOSSIER_TOKEN_ESTIMATE: u32 = 1_200;

/// What a run would cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunPreview {
    pub cells: u32,
    pub agents: u32,
    pub requests: u32,
    pub tokens: u32,
    /// Requests and tokens today's budget still allows (`None` = no limit).
    pub requests_left: Option<u32>,
    pub tokens_left: Option<u32>,
}

impl RunPreview {
    /// Whether the run fits the lab's cap and today's budget.
    pub fn fits(&self) -> bool {
        let within = |need: u32, left: Option<u32>| left.is_none_or(|left| need <= left);
        self.requests > 0 && self.requests <= MAX_LAB_REQUESTS && within(self.requests, self.requests_left) && within(self.tokens, self.tokens_left)
    }

    pub fn over_cap(&self) -> bool {
        self.requests > MAX_LAB_REQUESTS
    }
}

/// What's left of `cap` after `used` (`None` when `cap` is 0, meaning no limit).
fn left(cap: u32, used: u32) -> Option<u32> {
    (cap > 0).then(|| cap.saturating_sub(used))
}

/// The preview of running `scenario` with `agents` agents, `pending` of the cells' requests
/// still to make (all of them for a fresh run), against `usage` and `limits` on `today`.
pub fn preview(scenario: &Scenario, agents: usize, pending: usize, usage: &DailyUsage, limits: BudgetLimits, today: &str) -> RunPreview {
    let cells = cells(scenario).len() as u32;
    let requests = pending as u32;
    let per_request = cell_prompt_estimate(scenario).saturating_add(DOSSIER_TOKEN_ESTIMATE).saturating_add(CELL_ANSWER_TOKENS);
    let spent = usage.on(today);
    RunPreview {
        cells,
        agents: agents as u32,
        requests,
        tokens: per_request.saturating_mul(requests),
        requests_left: left(limits.daily_requests, spent.requests),
        tokens_left: left(limits.daily_tokens, spent.tokens),
    }
}
