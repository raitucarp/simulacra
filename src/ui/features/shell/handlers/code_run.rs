//! Running a code block (P5): Run checks the post may run code (not locked, its runtime on and
//! found), shows code from anyone but the user's own personas in a confirm first, then runs it
//! off the UI thread in the runner (fresh folder, cleared environment, time limit, output cap,
//! whole process tree ended), one run per block at a time.

use super::super::view::AppView;
use crate::domain::code::DetectedRuntime;
use crate::domain::preferences::ToastKind;
use crate::runner::{run_code, spec_for, Captured, End, Limits};
use crate::ui::features::post::{needs_confirm, RunRequest};
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use rust_i18n::t;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

/// Output a run may print (64 KiB); more stops it.
pub(in crate::ui::features::shell) const OUTPUT_CAP: usize = 64 * 1024;

/// A run waiting on the user's confirm.
pub(super) struct RunAsk {
    pub request: RunRequest,
    pub found: DetectedRuntime,
    /// Who wrote the post.
    pub author: String,
}

impl AppView {
    /// Run was pressed on a block.
    pub(in crate::ui::features::shell) fn on_run_code(&mut self, request: RunRequest, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post_by_id(&request.post) else { return };
        let prefs = &self.state.preferences().code;
        let found = self.code.cache.get(request.runtime).cloned().filter(|_| prefs.allows(request.runtime));
        let Some(found) = found.filter(|_| !post.is_locked()) else { return };
        let author = self.state.author_of(post);
        if !needs_confirm(post, author) {
            return self.start_code_run(request, found, window, cx);
        }
        let ask = RunAsk { request, found, author: author.name.clone() };
        let (view, theme) = (cx.entity().downgrade(), self.theme.clone());
        window.defer(cx, move |window, cx| super::code_confirm::open_run_confirm(view, ask, theme, window, cx));
    }

    /// Runs `request` in `found` (the confirm, if any, was accepted).
    pub(in crate::ui::features::shell) fn start_code_run(&mut self, request: RunRequest, found: DetectedRuntime, window: &mut Window, cx: &mut Context<Self>) {
        let key = (request.post.clone(), request.block);
        if self.code.running.contains_key(&key) {
            return;
        }
        let timeout = Duration::from_secs(u64::from(self.state.preferences().code.timeout_seconds));
        let spec = spec_for(&found, &request.code, Limits { timeout, output_cap: OUTPUT_CAP });
        let cancel = Arc::new(AtomicBool::new(false));
        self.code.running.insert(key.clone(), cancel.clone());
        let task = cx.background_executor().spawn(async move { run_code(&spec, &cancel) });
        cx.spawn_in(window, async move |this, cx| {
            let outcome = task.await;
            let _ = this.update_in(cx, |this, window, cx| this.finish_code_run(key, &found, outcome, window, cx));
        })
        .detach();
        cx.notify();
    }

    fn finish_code_run(&mut self, key: (String, usize), found: &DetectedRuntime, outcome: std::io::Result<Captured>, window: &mut Window, cx: &mut Context<Self>) {
        self.code.running.remove(&key);
        let note = match &outcome {
            Ok(captured) => run_summary(captured),
            Err(err) => t!("code.run.failed", runtime = found.runtime.label(), reason = err.to_string()).to_string(),
        };
        self.toast(ToastKind::Confirmation, Notification::info(note), window, cx);
        cx.notify();
    }
}

/// How a run ended, in one line.
pub(in crate::ui::features::shell) fn run_summary(captured: &Captured) -> String {
    let seconds = format!("{:.1}", captured.duration.as_secs_f64());
    match (captured.end, captured.exit_code) {
        (End::Exited, Some(code)) => t!("code.run.exited", code = code, seconds = seconds).to_string(),
        (End::Exited, None) => t!("code.run.ended", seconds = seconds).to_string(),
        (End::TimedOut, _) => t!("code.run.timed_out", seconds = seconds).to_string(),
        (End::Cancelled, _) => t!("code.run.cancelled").to_string(),
        (End::OutputCapped, _) => t!("code.run.capped").to_string(),
    }
}
