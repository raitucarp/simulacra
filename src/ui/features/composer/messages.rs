//! Localized explanations for drafts that cannot be published, and the notice line showing them.

use crate::domain::{DraftIssue, ThreadProblem};
use crate::error::AppError;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{div, Div, ParentElement, Styled};
use rust_i18n::t;

/// Human (1-based) position of a draft in the thread.
fn ordinal(index: usize) -> usize {
    index + 1
}

/// Why the thread cannot be posted yet, e.g. "Post 2 is 12 characters over the limit".
pub fn thread_problem_message(problem: &ThreadProblem) -> String {
    let index = ordinal(problem.index);
    match problem.issue {
        DraftIssue::Empty => t!("composer.error_empty", index = index).to_string(),
        DraftIssue::TooLong { max, .. } => t!(
            "composer.error_too_long",
            index = index,
            over = problem.issue.overflow(),
            max = max
        )
        .to_string(),
    }
}

/// Explains a store rejection of a single-post submission.
pub fn publish_error_message(error: &AppError) -> String {
    match error {
        AppError::EmptyContent => {
            thread_problem_message(&ThreadProblem { index: 0, issue: DraftIssue::Empty })
        }
        AppError::ContentTooLong { max, actual } => {
            let issue = DraftIssue::TooLong { actual: *actual, max: *max };
            thread_problem_message(&ThreadProblem { index: 0, issue })
        }
        AppError::InvalidSymbols => t!("symbols.error.not_allowed").to_string(), // S7
        AppError::OpenSubtasks(open) => crate::i18n::t_count("symbols.error.open_children", *open), // S8
        other => t!("composer.error_publish", reason = other.to_string()).to_string(),
    }
}

/// Message for a blocked submission, hiding the "empty" nag on an untouched single draft.
pub fn visible_problem_message(problem: Option<ThreadProblem>, draft_count: usize) -> Option<String> {
    let problem = problem?;
    let pristine = draft_count <= 1 && problem.issue == DraftIssue::Empty;
    (!pristine).then(|| thread_problem_message(&problem))
}

/// Danger-tinted notice line (icon + text) shown under a composer's editor.
pub(super) fn render_notice(notice: String, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .text_size(theme.font_size_xs)
        .text_color(theme.accent_danger)
        .child(Icon::new(LucideIcon::Info).size(theme.font_size_sm))
        .child(div().flex_1().min_w_0().child(notice))
}
