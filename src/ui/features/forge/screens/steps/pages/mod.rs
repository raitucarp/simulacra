//! The option pages of each step, plus the field-control builders they share.

mod appearance;
mod attributes;
mod avatar_kind;
mod birthsign;
mod daily;
mod embodiment;
mod engine;
mod form_editor;
mod form_options;
mod grimoire;
mod origin;
mod origin_life;
mod photo;
mod portrait;
mod psyche;
mod relationships;
mod review;
mod review_card;
mod rhetoric;
mod trait_block;
mod voice;

use super::super::common::message;
use crate::state::forge::{DraftEdit, IssueKind, Severity, TextField, ValidationIssue, ARG_VALUE};
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::ForgeTextInput;
use crate::state::forge::suggest::Param;
use gpui_kit::*;

const TEXT_ID_PREFIX: &str = "forge-text";
/// Shown in place of a custom tool's name while it is still blank.
const UNNAMED_TOOL_KEY: &str = "forge.issue.unnamed_tool";
/// Rows (min, max) of long free-text fields.
const LONG_TEXT_ROWS: (usize, usize) = (3, 8);
/// Free-text fields edited in a multi-line area.
const LONG_TEXT_FIELDS: [TextField; 5] = [
    TextField::Bio,
    TextField::DailyRoutine,
    TextField::CustomMentionInstruction,
    TextField::SystemPrompt,
    TextField::PromptTemplate,
];

/// The translated message of a validation issue, with its arguments filled in.
pub(super) fn issue_text(issue: &ValidationIssue) -> String {
    tr_with(&issue.message_key(), &issue_args(issue, &tr(UNNAMED_TOOL_KEY)))
}

/// The arguments of `issue`, with a blank custom tool name shown as `unnamed`.
pub(super) fn issue_args(issue: &ValidationIssue, unnamed: &str) -> Vec<(&'static str, String)> {
    let names_tool = matches!(issue.kind, IssueKind::EngineToolName | IssueKind::EngineToolParams);
    let shown = |name: &str, value: &str| match names_tool && name == ARG_VALUE && value.trim().is_empty() {
        true => unnamed.to_string(),
        false => value.to_string(),
    };
    issue.args.iter().map(|(name, value)| (*name, shown(name, value))).collect()
}

impl ForgeView {
    /// Issues reported on the field with serde name `path`.
    pub(super) fn field_issues(&self, path: &str) -> Vec<&ValidationIssue> {
        self.issues.iter().filter(|issue| issue.field == path && self.reveals(issue)).collect()
    }

    /// Red / yellow lines under a field for its issues.
    pub(super) fn issue_lines(&self, path: &str) -> Vec<Div> {
        let theme = &self.theme;
        self.field_issues(path)
            .into_iter()
            .map(|issue| {
                let color = if issue.severity == Severity::Error { theme.accent_danger } else { theme.warning };
                message(issue_text(issue), color, theme)
            })
            .collect()
    }

    /// A text input bound to `field` (multi-line for long fields).
    pub(super) fn text_input(&self, field: TextField, handle: &ForgeHandle) -> ForgeTextInput {
        let id = SharedString::from(format!("{TEXT_ID_PREFIX}-{}", field.path()));
        let value = field.get(self.draft.profile()).clone();
        let edit = handle.edit(move |text: String| DraftEdit::Text(field, text));
        let seal = handle.seal();
        let input = ForgeTextInput::new(id, value, &self.theme, move |text, w, cx| edit(text, w, cx))
            .invalid(self.field_issues(field.path()).iter().any(|issue| issue.severity == Severity::Error))
            .on_blur(move |w, cx| seal(w, cx));
        if LONG_TEXT_FIELDS.contains(&field) {
            input.multiline(LONG_TEXT_ROWS.0, LONG_TEXT_ROWS.1)
        } else {
            input
        }
    }

    /// The text input of `field` with its example chips (the name has its own ideas row).
    pub(super) fn text_with_examples(&self, field: TextField, input: impl IntoElement, handle: &ForgeHandle) -> Div {
        let examples = (field != TextField::Name).then(|| self.text_examples(field, handle)).flatten();
        div().flex().flex_col().gap(self.theme.spacing_xs).child(input).children(examples)
    }

    /// A captioned text input with example chips, "Why?" / "Suggest" and its issue lines.
    pub(super) fn text_row(&self, field: TextField, handle: &ForgeHandle) -> Div {
        let control = self.text_with_examples(field, self.text_input(field, handle), handle);
        self.hinted(Param::Text(field), control, handle).children(self.issue_lines(field.path()))
    }
}
