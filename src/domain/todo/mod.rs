//! To-dos written inside a post body, parsed into a tree with byte spans.
//!
//! ```text
//! [ ] open   [x] done   [>] migrated   [<] scheduled   [~] dropped   [/] delegated
//! ^fri  ^2026-12-01  ^+3d     due (raw phrase; `^next month` is one token, see [`due`])
//! ^^daily  ^^mon,wed  ^^15th  repeat (raw rule)
//! !  !!  !!!                  priority 1..3
//! &home                       context
//! ~25m  ~2h  ~1h30m           estimate
//! @name                       delegate
//! ```
//!
//! - A to-do line is optional indentation, an optional `-`/`*` bullet, a box and its text.
//!   The box must be followed by whitespace or the end of the line (`[x]done` is not one).
//! - Indentation (two spaces or a tab per level) nests an item under the closest item above
//!   it at a lower level. Other lines are ignored and do not break nesting; lines inside
//!   ```` ``` ```` fences are never items.
//! - Tokens are whole whitespace-separated words, so `a&b`, `x@y.z`, `wow!` and `a^b` stay
//!   text. Single-valued tokens (due, repeat, priority, estimate, delegate) take the first
//!   occurrence and later ones stay text. Contexts collect without duplicates.
//! - `text` is the item without box and tokens, words joined by single spaces.
//!
//! Spans are byte ranges into the parsed text, so items and tokens can later be mapped onto
//! the post body tokens.

mod line;
pub mod due; // D3: due phrases as days
pub mod syntax; // D1: the help card rows
mod tokens;
#[cfg(test)]
mod tests;

pub use tokens::{parse_estimate, TokenValue};

use crate::domain::symbols::EntryState;
use std::ops::Range;

/// A byte range into the parsed text.
pub type Span = Range<usize>;

/// What a to-do token sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TodoTokenKind {
    Due,
    Repeat,
    Priority,
    Context,
    Estimate,
    Delegate,
}

/// One recognised token and where it sits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoToken {
    pub kind: TodoTokenKind,
    pub span: Span,
}

/// The values the tokens of one item set.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TodoMeta {
    /// Raw due phrase without `^` (`fri`, `2026-12-01`, `+3d`); resolved to a date later.
    pub due: Option<String>,
    /// Raw repeat rule without `^^` (`daily`, `mon,wed`, `15th`).
    pub repeat: Option<String>,
    /// 1 to 3.
    pub priority: Option<u8>,
    pub contexts: Vec<String>,
    pub estimate_minutes: Option<u32>,
    pub delegate: Option<String>,
}

/// One to-do and the to-dos nested under it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub state: EntryState,
    pub text: String,
    /// The whole line, without its line ending.
    pub line_span: Span,
    /// The `[ ]` box.
    pub marker_span: Span,
    pub tokens: Vec<TodoToken>,
    pub meta: TodoMeta,
    pub children: Vec<TodoItem>,
}

impl TodoItem {
    /// The day the item is due, reading its phrase from `written`, the day the post was
    /// written in the user's time zone (D3); `None` without a due phrase or with one that is
    /// not a date.
    pub fn due_date(&self, written: jiff::civil::Date) -> Option<jiff::civil::Date> {
        due::resolve_due(self.meta.due.as_deref()?, written)
    }
}

/// Parses every to-do line of `text` into a tree (see the module rules).
pub fn parse_todos(text: &str) -> Vec<TodoItem> {
    let mut roots = Vec::new();
    let mut open: Vec<(usize, TodoItem)> = Vec::new();
    let mut in_fence = false;
    let mut offset = 0;
    for raw in text.split_inclusive('\n') {
        let line = raw.trim_end_matches(['\n', '\r']);
        let line_offset = offset;
        offset += raw.len();
        if line.trim_start().starts_with(FENCE) {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        let Some((level, item)) = line::parse_line(line, line_offset) else { continue };
        close_to(level, &mut open, &mut roots);
        open.push((level, item));
    }
    close_to(0, &mut open, &mut roots);
    roots
}

/// Every item of a tree, depth first in reading order.
pub fn flatten_todos(items: &[TodoItem]) -> Vec<&TodoItem> {
    let mut out = Vec::new();
    for item in items {
        out.push(item);
        out.extend(flatten_todos(&item.children));
    }
    out
}

const FENCE: &str = "```";

/// Closes every open item at `level` or deeper, attaching each to its parent (or the roots).
fn close_to(level: usize, open: &mut Vec<(usize, TodoItem)>, roots: &mut Vec<TodoItem>) {
    while open.last().is_some_and(|(open_level, _)| *open_level >= level) {
        let Some((_, item)) = open.pop() else { break };
        match open.last_mut() {
            Some((_, parent)) => parent.children.push(item),
            None => roots.push(item),
        }
    }
}
