//! N4: the finite feed. With a daily budget set, the feed shows at most that many posts a
//! day, counting each post the first time it is shown; once the budget is spent the feed
//! ends with a closing card ("What did you learn today?"), answered as a one-line post.
//! Posts already shown today stay, and the user's own posts never count. The tally and the
//! answer belong to one local day and start afresh the next.

use crate::domain::PostId;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The longest answer to the closing question, in characters.
pub const MAX_ANSWER_CHARS: usize = 140;

/// What the feed showed on one day (persisted as the `feed.day` setting).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FeedDay {
    /// The local day this tally is for (`None`: never used).
    pub day: Option<Date>,
    /// Posts counted against the budget, in the order they were first shown.
    pub seen: Vec<PostId>,
    /// The post answering the closing question, once written.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<PostId>,
}

impl FeedDay {
    /// This tally on `today`: unchanged on the same day, empty on any other.
    #[must_use]
    pub fn on(self, today: Date) -> Self {
        if self.day == Some(today) {
            self
        } else {
            Self { day: Some(today), ..Self::default() }
        }
    }

    /// Counts `shown` (in order) against the budget; `true` when anything new was counted.
    pub fn count(&mut self, shown: impl IntoIterator<Item = PostId>) -> bool {
        let before = self.seen.len();
        for id in shown {
            if !self.seen.contains(&id) {
                self.seen.push(id);
            }
        }
        self.seen.len() != before
    }
}

/// The feed under a budget: the store positions it shows, and whether the budget is spent.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BudgetWindow {
    pub positions: Vec<usize>,
    /// The budget is spent: the feed ends with the closing card.
    pub ended: bool,
}

/// The posts of `order` (each its id, store position and whether it counts) the feed shows
/// under `budget` posts a day (`0`: no budget) given those already `seen` today.
pub fn budget_window<'a>(order: impl IntoIterator<Item = (&'a PostId, usize, bool)>, seen: &HashSet<PostId>, budget: usize) -> BudgetWindow {
    let mut window = BudgetWindow::default();
    let mut spent = seen.len();
    for (id, position, counts) in order {
        if budget == 0 || !counts || seen.contains(id) {
            window.positions.push(position);
        } else if spent < budget {
            spent += 1;
            window.positions.push(position);
        } else {
            window.ended = true;
        }
    }
    window.ended |= budget > 0 && spent >= budget;
    window
}

/// The answer as a one-line post: whitespace collapsed, at most [`MAX_ANSWER_CHARS`]; `None`
/// when nothing is left.
pub fn one_line_answer(text: &str) -> Option<String> {
    let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let line: String = line.chars().take(MAX_ANSWER_CHARS).collect();
    (!line.is_empty()).then_some(line)
}

#[cfg(test)]
mod tests;
