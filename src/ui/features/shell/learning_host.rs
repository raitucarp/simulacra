//! Learning wiring of the shell (sb E2): the goals, tutor sessions and palaces (loaded once
//! from the vault), the tutor's running request, and the weekly review reminder (the daily
//! reminder's rule on the weekday of Settings › Learning).

use super::view::AppView;
use crate::agent::post_actions::CancelToken;
use crate::domain::learning::weekly::{weekly_review_due, WeeklyReminder};
use crate::domain::learning::LearningRecords;
use crate::state::AppState;
use crate::ui::features::review::learning::TutorStatus;
use jiff::civil::Date;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;

/// The learning records and the tutor's request.
#[derive(Default)]
pub struct LearningHost {
    pub records: LearningRecords,
    pub tutor: TutorStatus,
    /// The running request's cancel flag.
    pub(super) cancel: Option<CancelToken>,
    /// Bumped on every request; a finished request only lands if it is still the latest.
    pub(super) generation: u64,
    /// The day the weekly reminder was last shown on.
    weekly_shown_on: Option<Date>,
}

impl LearningHost {
    /// Loads the records from the vault (none without one).
    pub fn load(state: &AppState) -> Self {
        let records = state.db.as_ref().and_then(|db| db.load_learning().map_err(|err| eprintln!("simulacra: learning records not loaded: {err}")).ok()).unwrap_or_default();
        Self { records, ..Self::default() }
    }

    /// Stops the running request, if any.
    pub(super) fn cancel_running(&mut self) {
        if let Some(cancel) = self.cancel.take() {
            cancel.cancel();
        }
        self.generation += 1;
    }
}

impl AppView {
    /// On the weekly review's day, at or after its hour, once: a nudge to do it.
    pub(super) fn remind_weekly_review(&mut self, today: Date, window: &mut Window, cx: &mut Context<Self>) {
        let prefs = &self.state.preferences().learning;
        let reminder = WeeklyReminder { enabled: prefs.weekly_review_reminder, weekday: prefs.weekly_review_day.weekday(), hour: prefs.weekly_review_hour };
        let now = crate::time::local_now();
        if !weekly_review_due(&reminder, today, today.weekday(), now.hour() as u32, self.review.learning.weekly_shown_on) {
            return;
        }
        self.review.learning.weekly_shown_on = Some(today);
        window.push_notification(Notification::info(rust_i18n::t!("learning.weekly.reminder").to_string()), cx);
    }
}
