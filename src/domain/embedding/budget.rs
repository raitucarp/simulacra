//! The daily AI budget for embedding requests (Settings › AI & Models › Budget): requests and
//! estimated tokens counted per local day; `0` means no limit.

use crate::domain::preferences::AiPrefs;
use serde::{Deserialize, Serialize};

/// Characters per estimated token.
const CHARS_PER_TOKEN: usize = 4;

/// The daily caps (`0` = no limit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetLimits {
    pub daily_requests: u32,
    pub daily_tokens: u32,
}

impl BudgetLimits {
    pub fn from_prefs(ai: &AiPrefs) -> Self {
        Self { daily_requests: ai.daily_request_budget, daily_tokens: ai.daily_token_cap }
    }
}

/// What was spent on one day (`day` is `YYYY-MM-DD`, local).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DailyUsage {
    pub day: String,
    pub requests: u32,
    pub tokens: u32,
}

impl DailyUsage {
    /// The usage counted on `today` (another day's usage starts over).
    pub fn on(&self, today: &str) -> Self {
        if self.day == today {
            return self.clone();
        }
        Self { day: today.to_string(), requests: 0, tokens: 0 }
    }

    /// Whether `requests` more requests of `tokens` tokens fit in `limits` on `today`.
    pub fn allows(&self, limits: BudgetLimits, today: &str, requests: u32, tokens: u32) -> bool {
        let usage = self.on(today);
        let fits = |used: u32, more: u32, cap: u32| cap == 0 || used.saturating_add(more) <= cap;
        fits(usage.requests, requests, limits.daily_requests) && fits(usage.tokens, tokens, limits.daily_tokens)
    }

    /// The usage after spending `requests` and `tokens` on `today`.
    pub fn recorded(&self, today: &str, requests: u32, tokens: u32) -> Self {
        let usage = self.on(today);
        Self { requests: usage.requests.saturating_add(requests), tokens: usage.tokens.saturating_add(tokens), ..usage }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Parses stored JSON; anything unreadable counts as nothing spent.
    pub fn from_json(raw: &str) -> Self {
        serde_json::from_str(raw).unwrap_or_default()
    }
}

/// A rough token count of `texts` (characters / 4, at least one per text).
pub fn estimate_tokens<'a>(texts: impl IntoIterator<Item = &'a str>) -> u32 {
    let total: usize = texts.into_iter().map(|text| text.chars().count().div_ceil(CHARS_PER_TOKEN).max(1)).sum();
    u32::try_from(total).unwrap_or(u32::MAX)
}
