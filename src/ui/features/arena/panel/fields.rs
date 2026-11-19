//! The page's text fields: the motion, and the scenario's title, premise, question and up to
//! three variables (a name and its values, separated by `|`).

use super::ArenaPanel;
use crate::domain::arena::lab::{MAX_VARIABLES, VariableDraft};
use gpui_kit::component::input::{InputEvent, InputState, TextareaState};
use gpui_kit::{App, AppContext, Context, Entity, Subscription, Window};
use rust_i18n::t;

/// Rows of the premise (shortest, tallest before scrolling).
const PREMISE_ROWS: (usize, usize) = (3, 10);
const MOTION_ROWS: (usize, usize) = (2, 5);

/// One variable's two fields.
pub struct VariableFields {
    pub name: Entity<InputState>,
    pub values: Entity<InputState>,
}

/// Text fields of the Arena page.
pub struct ArenaFields {
    pub motion: Entity<TextareaState>,
    pub title: Entity<InputState>,
    pub premise: Entity<TextareaState>,
    pub question: Entity<InputState>,
    pub variables: Vec<VariableFields>,
}

fn input(placeholder: &str, window: &mut Window, cx: &mut Context<ArenaPanel>) -> Entity<InputState> {
    let placeholder = t!(placeholder).to_string();
    cx.new(|cx| InputState::new(window, cx).placeholder(placeholder))
}

fn textarea(placeholder: &str, rows: (usize, usize), window: &mut Window, cx: &mut Context<ArenaPanel>) -> Entity<TextareaState> {
    let placeholder = t!(placeholder).to_string();
    cx.new(|cx| TextareaState::new(window, cx).placeholder(placeholder).auto_grow(rows.0, rows.1))
}

impl ArenaFields {
    /// The fields, and the subscriptions that re-render on typing (the setup's problems and
    /// the lab's preview follow the text).
    pub fn new(window: &mut Window, cx: &mut Context<ArenaPanel>) -> (Self, Vec<Subscription>) {
        let variables = (0..MAX_VARIABLES)
            .map(|_| VariableFields { name: input("arena.lab.variable_name_hint", window, cx), values: input("arena.lab.variable_values_hint", window, cx) })
            .collect();
        let fields = Self {
            motion: textarea("arena.debate.motion_hint", MOTION_ROWS, window, cx),
            title: input("arena.lab.title_hint", window, cx),
            premise: textarea("arena.lab.premise_hint", PREMISE_ROWS, window, cx),
            question: input("arena.lab.question_hint", window, cx),
            variables,
        };
        let notify = |_: &mut ArenaPanel, _: Entity<InputState>, event: &InputEvent, cx: &mut Context<ArenaPanel>| {
            if matches!(event, InputEvent::Change) {
                cx.notify();
            }
        };
        let area_notify = |_: &mut ArenaPanel, _: Entity<TextareaState>, event: &InputEvent, cx: &mut Context<ArenaPanel>| {
            if matches!(event, InputEvent::Change) {
                cx.notify();
            }
        };
        let mut subscriptions = vec![cx.subscribe(&fields.motion, area_notify), cx.subscribe(&fields.premise, area_notify), cx.subscribe(&fields.question, notify)];
        subscriptions.extend(fields.variables.iter().flat_map(|variable| [cx.subscribe(&variable.name, notify), cx.subscribe(&variable.values, notify)]));
        (fields, subscriptions)
    }

    /// The trimmed text of `input`.
    pub fn text(input: &Entity<InputState>, cx: &App) -> String {
        input.read(cx).value().trim().to_string()
    }

    /// The text of a multi-line field, trimmed.
    pub fn area_text(input: &Entity<TextareaState>, cx: &App) -> String {
        input.read(cx).value().trim().to_string()
    }

    /// What the first `count` variable rows hold.
    pub fn variable_drafts(&self, count: usize, cx: &App) -> Vec<VariableDraft> {
        self.variables.iter().take(count).map(|variable| VariableDraft { name: Self::text(&variable.name, cx), values: Self::text(&variable.values, cx) }).collect()
    }
}
