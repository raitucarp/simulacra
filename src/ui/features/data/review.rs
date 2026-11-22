//! The monthly migration review sheet (S16): the user's open tasks one at a time, each
//! migrated, scheduled, dropped or kept open; then the tally, with an optional summary post.

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, hint, sheet_frame, SheetParts};
use super::text::tr;
use crate::domain::symbols::ReviewChoice;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const SHEET_ID: &str = "data-review";
const CLOSE_ID: &str = "data-review-close";
const SUMMARY_ID: &str = "data-review-summary";
const CHOICE_ID: &str = "data-review-choice";
/// Lines of a task's text shown before it is cut.
const EXCERPT_LINES: usize = 4;

/// What the sheet shows: the task being looked at (its text) and where the walk stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewView {
    pub current: Option<String>,
    /// 1-based position of the current task, and how many there are.
    pub position: usize,
    pub total: usize,
    /// Migrated, scheduled, dropped and kept so far.
    pub tally: [usize; 4],
}

fn choice_icon(choice: ReviewChoice) -> LucideIcon {
    match choice {
        ReviewChoice::Migrate => LucideIcon::ChevronRight,
        ReviewChoice::Schedule => LucideIcon::ChevronLeft,
        ReviewChoice::Drop => LucideIcon::X,
        ReviewChoice::Keep => LucideIcon::Check,
    }
}

/// The tally as a sentence (`key` is the finished line or the summary post).
pub fn tally_text(key: &str, tally: [usize; 4]) -> String {
    t!(key, migrated = tally[0], scheduled = tally[1], dropped = tally[2], kept = tally[3]).to_string()
}

fn current_task(text: &str, view: &ReviewView, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let buttons = ReviewChoice::ALL.map(|choice| {
        let on_event = on_event.clone();
        Button::new(SharedString::from(format!("{CHOICE_ID}-{}", choice.key()))).ghost().small().icon(choice_icon(choice)).label(tr(&format!("symbols.review.{}", choice.key()))).on_click(move |_, w, cx| on_event(DataEvent::ReviewDecide(choice), w, cx))
    });
    let progress = t!("symbols.review.progress", current = view.position, total = view.total).to_string();
    div().flex().flex_col().gap(theme.spacing_sm)
        .child(div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(progress))
        .child(div().p(theme.spacing_sm).rounded(theme.radius_md).border(theme.border_width_hairline).border_color(theme.border_subtle).text_size(theme.font_size_sm).line_clamp(EXCERPT_LINES).text_color(theme.text_primary).child(text.to_string()))
        .child(div().flex().flex_wrap().gap(theme.spacing_xs).children(buttons))
}

/// The overlay.
pub fn render_review_sheet(view: &ReviewView, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let body = match (&view.current, view.total) {
        (Some(text), _) => current_task(text, view, &on_event, theme),
        (None, 0) => div().child(hint(tr("symbols.review.empty"), theme)),
        (None, _) => div().child(hint(tally_text("symbols.review.done", view.tally), theme)),
    };
    let finished = view.current.is_none() && view.total > 0;
    let (close, closing, summary) = (on_event.clone(), on_event.clone(), on_event);
    let buttons = footer(theme)
        .when(finished, |row| row.child(Button::new(SUMMARY_ID).primary().small().label(tr("symbols.review.post_summary")).on_click(move |_, w, cx| summary(DataEvent::ReviewSummary, w, cx))))
        .child(Button::new(CLOSE_ID).ghost().small().label(tr("symbols.review.close")).on_click(move |_, w, cx| close(DataEvent::CloseReview, w, cx)));
    let parts = SheetParts { id: SHEET_ID, icon: LucideIcon::ListChecks, title: tr("symbols.review.title"), subtitle: Some(tr("symbols.review.subtitle")), body, footer: buttons, on_close: Rc::new(move |w, cx| closing(DataEvent::CloseReview, w, cx)) };
    sheet_frame(parts, theme)
}
