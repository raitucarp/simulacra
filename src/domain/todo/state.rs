//! Ticked boxes (D6): a box clicked in a post is kept in `extras.todo_state`, not in the
//! text, so ticking never makes a new version of the post. The text keeps the box as it was
//! written; a tick says what the item is now.
//!
//! Each tick names its item by place (the `index`-th to-do in reading order) and by text.
//! When an edit changes the text, every tick moves to the item with the same text in the new
//! version (the nearest one to its old place), and a tick that now says what the box says is
//! dropped.

use super::{flatten_todos, parse_todos, TodoItem};
use crate::domain::symbols::EntryState;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

fn is_zero(count: &u32) -> bool {
    *count == 0
}

/// What the reader has done to the to-dos of one post.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TodoState {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ticks: Vec<TodoTick>,
}

/// One item's tick.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TodoTick {
    /// The item's place among the post's to-dos, in reading order.
    pub index: usize,
    /// The item's text when it was ticked.
    pub text: String,
    /// The state the item is in now; `None` keeps the written box.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<EntryState>,
    /// The day a repeating item is next due, once it was completed (D8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<Date>,
    /// How many times a repeating item was completed (D8).
    #[serde(skip_serializing_if = "is_zero")]
    pub completions: u32,
}

impl TodoTick {
    /// Whether the tick says nothing the text does not.
    fn is_blank(&self) -> bool {
        Self { index: 0, text: String::new(), ..self.clone() } == Self::default()
    }
}

impl TodoState {
    pub fn is_empty(&self) -> bool {
        self.ticks.is_empty()
    }

    /// The tick of the `index`-th item, if it still has the text `text`.
    pub fn tick(&self, index: usize, text: &str) -> Option<&TodoTick> {
        self.ticks.iter().find(|tick| tick.index == index && tick.text == text)
    }

    /// The tick of `item` (the `index`-th), made when missing.
    pub fn tick_mut(&mut self, index: usize, item: &TodoItem) -> &mut TodoTick {
        let position = match self.ticks.iter().position(|tick| tick.index == index) {
            Some(position) => position,
            None => {
                self.ticks.push(TodoTick { index, ..TodoTick::default() });
                self.ticks.len() - 1
            }
        };
        let tick = &mut self.ticks[position];
        if tick.text != item.text {
            *tick = TodoTick { index, text: item.text.clone(), ..TodoTick::default() };
        }
        tick
    }

    /// The state the `index`-th item is in: its tick's, else its box's.
    pub fn state_of(&self, index: usize, item: &TodoItem) -> EntryState {
        self.tick(index, &item.text).and_then(|tick| tick.state).unwrap_or(item.state)
    }

    /// Drops ticks that say what the text of `content` already does.
    pub fn tidy(&mut self, content: &str) {
        let items = parse_todos(content);
        let flat = flatten_todos(&items);
        self.ticks.retain_mut(|tick| {
            let Some(item) = flat.get(tick.index).filter(|item| item.text == tick.text) else { return false };
            if tick.state == Some(item.state) {
                tick.state = None;
            }
            !tick.is_blank()
        });
        self.ticks.sort_by_key(|tick| tick.index);
    }

    /// These ticks moved from the to-dos of `before` onto those of `after` (an edit).
    pub fn carried(&self, before: &str, after: &str) -> Self {
        let old = parse_todos(before);
        let new = parse_todos(after);
        let (old, new) = (flatten_todos(&old), flatten_todos(&new));
        let mut taken = vec![false; new.len()];
        let mut ticks = Vec::new();
        for tick in &self.ticks {
            if old.get(tick.index).is_none_or(|item| item.text != tick.text) {
                continue;
            }
            let nearest = (0..new.len()).filter(|i| !taken[*i] && new[*i].text == tick.text).min_by_key(|i| i.abs_diff(tick.index));
            if let Some(index) = nearest {
                taken[index] = true;
                ticks.push(TodoTick { index, ..tick.clone() });
            }
        }
        let mut carried = Self { ticks };
        carried.tidy(after);
        carried
    }
}
