//! The post lifecycle on the scheduler heartbeat (Y2): due scheduled posts are published, due
//! posts archived (Y6) and posts due to delete themselves deleted (Y7) on every tick (and once
//! at startup), whether or not agents post on their own; to-dos that came due are reminded of
//! (D10).

use super::super::view::AppView;
use crate::agent::schedule::Clock;
use crate::domain::preferences::ToastKind;
use crate::state::store::{DueReminder, LifecycleReport};
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// One lifecycle tick at `clock`'s time: carries out what is due and says so. While a post
    /// counts down to deleting itself, every tick re-renders so its chip stays current.
    pub(in crate::ui::features::shell) fn post_lifecycle_tick(&mut self, clock: &dyn Clock, window: &mut Window, cx: &mut Context<Self>) {
        match self.state.lifecycle_heartbeat(clock) {
            Ok(report) if !report.is_empty() => self.report_lifecycle(&report, window, cx),
            Ok(_) if self.state.posts.iter().any(|post| post.lifecycle.delete_at.is_some()) => cx.notify(),
            Ok(_) => {}
            Err(err) => eprintln!("simulacra: post lifecycle tick failed: {err}"),
        }
    }

    /// Forgets deleted posts, toasts what a tick changed and re-renders.
    fn report_lifecycle(&mut self, report: &LifecycleReport, window: &mut Window, cx: &mut Context<Self>) {
        self.forget_deleted_posts(&report.deleted);
        let changes = [(&report.published, "post.scheduled.published"), (&report.archived, "post.auto_archived"), (&report.deleted, "post.auto_deleted")];
        for (ids, key) in changes {
            if !ids.is_empty() {
                let text = crate::i18n::t_count(key, ids.len());
                self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
            }
        }
        self.remind(&report.reminders, window, cx);
        cx.notify();
    }

    /// Reminds the user of to-dos that came due (D10): one by name, several by count.
    fn remind(&mut self, reminders: &[DueReminder], window: &mut Window, cx: &mut Context<Self>) {
        let today = crate::time::local_now().date();
        let text = match reminders {
            [] => return,
            [one] if one.is_overdue(today) => t!("todo.remind.overdue", item = one.text, since = crate::ui::features::post::todo_due_label(one.due, today)).to_string(),
            [one] => t!("todo.remind.today", item = one.text).to_string(),
            many => crate::i18n::t_count("todo.remind.many", many.len()),
        };
        self.toast(ToastKind::DueReminder, Notification::info(text), window, cx);
    }
}
