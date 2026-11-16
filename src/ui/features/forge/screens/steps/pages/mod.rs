//! The option pages of each step, plus the field-control builders they share.

mod appearance;
mod attributes;
mod birthsign;
mod daily;
mod grimoire;
mod origin;
mod origin_life;
mod photo;
mod portrait;
mod psyche;
mod relationships;
mod review;
mod review_card;
mod trait_block;
mod voice;

use super::super::common::{labeled, message};
use crate::state::forge::{DraftEdit, Severity, TextField, ValidationIssue};
use crate::ui::features::forge::text::tr_with;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::ForgeTextInput;
use gpui_kit::*;

const TEXT_ID_PREFIX: &str = "forge-text";
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
    let args: Vec<(&str, String)> = issue.args.iter().map(|(name, value)| (*name, value.clone())).collect();
    tr_with(&issue.message_key(), &args)
}

impl ForgeView {
    /// Issues reported on the field with serde name `path`.
    pub(super) fn field_issues(&self, path: &str) -> Vec<&ValidationIssue> {
        self.issues.iter().filter(|issue| issue.field == path).collect()
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

    /// A captioned text input with its issue lines underneath.
    pub(super) fn text_row(&self, field: TextField, handle: &ForgeHandle) -> Div {
        labeled(field.path(), self.text_input(field, handle), &self.theme).children(self.issue_lines(field.path()))
    }
}
