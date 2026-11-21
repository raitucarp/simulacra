//! The post lifecycle on the scheduler heartbeat (Y2): due scheduled posts are published and
//! due posts archived (Y6) on every tick (and once at startup), whether or not agents post
//! on their own.

use super::super::view::AppView;
use crate::agent::schedule::Clock;
use crate::domain::preferences::ToastKind;
use crate::state::store::LifecycleReport;
use gpui_kit::component::notification::Notification;
use gpui_kit::*;

impl AppView {
    /// One lifecycle tick at `clock`'s time: publishes what is due and says so.
    pub(in crate::ui::features::shell) fn post_lifecycle_tick(&mut self, clock: &dyn Clock, window: &mut Window, cx: &mut Context<Self>) {
        match self.state.lifecycle_heartbeat(clock) {
            Ok(report) if !report.is_empty() => self.report_lifecycle(&report, window, cx),
            Ok(_) => {}
            Err(err) => eprintln!("simulacra: post lifecycle tick failed: {err}"),
        }
    }

    /// Toasts what a tick changed and re-renders.
    fn report_lifecycle(&mut self, report: &LifecycleReport, window: &mut Window, cx: &mut Context<Self>) {
        for (ids, key) in [(&report.published, "post.scheduled.published"), (&report.archived, "post.auto_archived")] {
            if !ids.is_empty() {
                let text = crate::i18n::t_count(key, ids.len());
                self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
            }
        }
        cx.notify();
    }
}
