//! The add / edit form: its input entities, choice values, open sections (progressive
//! disclosure) and the problems of the last save attempt.

pub mod draft;
pub mod fields;
#[cfg(test)]
mod tests;

use crate::domain::document::intake::{FieldProblem, RecordField};
use draft::EditorDraft;
use fields::TextField;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::{App, AppContext, Context, Entity, Window};
use rust_i18n::t;
use std::collections::{HashMap, HashSet};

/// Rows of the abstract and the text areas (minimum, maximum before scrolling).
const ABSTRACT_ROWS: (usize, usize) = (3, 8);
const CONTENT_ROWS: (usize, usize) = (6, 18);

/// A collapsible group of the form beyond the basics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorSection {
    Publication,
    Classification,
    Abstract,
}

impl EditorSection {
    pub const ALL: [Self; 3] = [Self::Publication, Self::Classification, Self::Abstract];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Publication => "publication",
            Self::Classification => "classification",
            Self::Abstract => "abstract",
        }
    }

    /// The section holding the field a problem points at (none for the basics).
    pub const fn of_field(field: RecordField) -> Option<Self> {
        match field {
            RecordField::Title | RecordField::Date => None,
            RecordField::Language | RecordField::Identifier | RecordField::Pages => Some(Self::Publication),
        }
    }
}

/// Input entities and choices of the form.
pub struct EditorState {
    pub inputs: HashMap<TextField, Entity<InputState>>,
    pub description: Entity<TextareaState>,
    pub content: Entity<TextareaState>,
    /// The values the inputs do not hold (kind, scheme, classification choices).
    pub choices: EditorDraft,
    pub open: HashSet<EditorSection>,
    pub problems: Vec<FieldProblem>,
}

fn textarea<V: 'static>(rows: (usize, usize), placeholder: String, window: &mut Window, cx: &mut Context<V>) -> Entity<TextareaState> {
    cx.new(|cx| TextareaState::new(window, cx).placeholder(placeholder).auto_grow(rows.0, rows.1))
}

impl EditorState {
    pub fn new<V: 'static>(window: &mut Window, cx: &mut Context<V>) -> Self {
        let inputs = TextField::ALL
            .into_iter()
            .map(|field| (field, cx.new(|cx| InputState::new(window, cx).placeholder(field.placeholder()))))
            .collect();
        Self {
            inputs,
            description: textarea(ABSTRACT_ROWS, t!("vault.hint.description").to_string(), window, cx),
            content: textarea(CONTENT_ROWS, t!("vault.hint.content").to_string(), window, cx),
            choices: EditorDraft::default(),
            open: HashSet::new(),
            problems: Vec::new(),
        }
    }

    pub fn input(&self, field: TextField) -> &Entity<InputState> {
        &self.inputs[&field]
    }

    /// The form's current values.
    pub fn read(&self, cx: &App) -> EditorDraft {
        let mut draft = EditorDraft {
            description: self.description.read(cx).value().to_string(),
            content: self.content.read(cx).value().to_string(),
            ..self.choices.clone()
        };
        for field in TextField::ALL {
            field.set(&mut draft, self.input(field).read(cx).value().to_string());
        }
        draft
    }

    /// Fills the form with `draft` and closes every section that has nothing in it.
    pub fn load<V: 'static>(&mut self, draft: EditorDraft, window: &mut Window, cx: &mut Context<V>) {
        for field in TextField::ALL {
            let value = field.get(&draft).to_string();
            self.input(field).update(cx, |input, cx| input.set_value(value, window, cx));
        }
        let (description, content) = (draft.description.clone(), draft.content.clone());
        self.description.update(cx, |input, cx| input.set_value(description, window, cx));
        self.content.update(cx, |input, cx| input.set_value(content, window, cx));
        self.choices = draft;
        self.open.clear();
        self.problems.clear();
    }

    /// Opens the sections holding `problems` so every message is visible.
    pub fn show_problems(&mut self, problems: Vec<FieldProblem>) {
        self.open.extend(problems.iter().filter_map(|p| EditorSection::of_field(p.field())));
        self.problems = problems;
    }

    /// The first problem of `field`.
    pub fn problem(&self, field: RecordField) -> Option<FieldProblem> {
        self.problems.iter().copied().find(|p| p.field() == field)
    }
}
