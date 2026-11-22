//! A post body as the checklist renders it (D5): the plain runs of text between to-do
//! lines, and every to-do in reading order with its depth, its parent and its due day.

use super::due::resolve_due;
use super::state::TodoState;
use super::{flatten_todos, parse_todos, TodoItem};
use crate::domain::symbols::EntryState;
use jiff::civil::Date;

/// One run of a post body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BodyPart {
    /// Text between to-do lines, without the line breaks around it.
    Text(String),
    /// The to-do of this index (into [`todo_entries`]).
    Item(usize),
}

/// One to-do, flat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoEntry {
    /// Place among the body's to-dos, in reading order.
    pub index: usize,
    /// 0 for a top-level item.
    pub depth: usize,
    pub parent: Option<usize>,
    /// The item without its children.
    pub item: TodoItem,
    /// The state it is in.
    pub state: EntryState,
    /// The indices of the items nested right under it.
    pub children: Vec<usize>,
}

impl TodoEntry {
    /// The day it is due, reading its phrase from the day the post was written.
    pub fn due(&self, written: Date) -> Option<Date> {
        resolve_due(self.item.meta.due.as_deref()?, written)
    }

    /// Whether it is past its due day on `today` and still to do.
    pub fn is_overdue(&self, written: Date, today: Date) -> bool {
        !self.state.is_closed() && self.due(written).is_some_and(|due| due < today)
    }
}

/// Every to-do of `content`, flat in reading order, in the states `ticks` put them in (D6).
pub fn todo_entries(content: &str, ticks: &TodoState) -> Vec<TodoEntry> {
    let mut entries = Vec::new();
    for item in parse_todos(content) {
        push_entry(&item, 0, None, &mut entries);
    }
    for entry in &mut entries {
        entry.state = ticks.state_of(entry.index, &entry.item);
    }
    entries
}

fn push_entry(item: &TodoItem, depth: usize, parent: Option<usize>, entries: &mut Vec<TodoEntry>) {
    let index = entries.len();
    if let Some(parent) = parent {
        entries[parent].children.push(index);
    }
    let flat = TodoItem { children: Vec::new(), ..item.clone() };
    entries.push(TodoEntry { index, depth, parent, state: item.state, item: flat, children: Vec::new() });
    for child in &item.children {
        push_entry(child, depth + 1, Some(index), entries);
    }
}

/// `content` cut into runs of text and to-do lines, in order. Blank runs are left out.
pub fn body_parts(content: &str) -> Vec<BodyPart> {
    let items = parse_todos(content);
    let mut parts = Vec::new();
    let mut cursor = 0;
    for (index, item) in flatten_todos(&items).into_iter().enumerate() {
        push_text(&content[cursor..item.line_span.start], &mut parts);
        parts.push(BodyPart::Item(index));
        cursor = item.line_span.end;
    }
    push_text(&content[cursor..], &mut parts);
    parts
}

fn push_text(text: &str, parts: &mut Vec<BodyPart>) {
    let text = text.trim_matches(['\n', '\r']);
    if !text.trim().is_empty() {
        parts.push(BodyPart::Text(text.to_string()));
    }
}
