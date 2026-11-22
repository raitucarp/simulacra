//! Running a code block (P5, P6): Run checks the post may run code (not locked, its runtime
//! on and found), shows code from anyone but the user's own personas in a confirm first, then
//! runs it off the UI thread in the runner (fresh folder, cleared environment, time limit,
//! output cap, whole process tree ended), one run per block at a time, and keeps the result on
//! the post. Cancel stops a run.

use super::super::view::AppView;
use crate::domain::code::{kept_output, CodeRun, DetectedRuntime, RunEnd};
use crate::domain::PostId;
use crate::domain::preferences::ToastKind;
use crate::runner::{run_code, spec_for, Captured, End, Limits};
use crate::ui::features::post::{needs_confirm, RunRequest};
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use rust_i18n::t;
use std::sync::atomic::{AtomicBool, Ordering};
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
        // P7: checked again, since code running may have been switched off while a confirm was open.
        if self.code.running.contains_key(&key) || !self.state.preferences().code.allows(found.runtime) {
            return;
        }
        let timeout = Duration::from_secs(u64::from(self.state.preferences().code.timeout_seconds));
        let (runtime, limits) = (found.clone(), Limits { timeout, output_cap: OUTPUT_CAP });
        let cancel = Arc::new(AtomicBool::new(false));
        self.code.running.insert(key.clone(), cancel.clone());
        // P7: the executable is checked and the cache prepared off the UI thread, then it runs.
        let task = cx.background_executor().spawn(async move { spec_for(&runtime, &request.code, limits).and_then(|spec| run_code(&spec, &cancel)) });
        cx.spawn_in(window, async move |this, cx| {
            let outcome = task.await;
            let _ = this.update_in(cx, |this, window, cx| this.finish_code_run(key, &found, outcome, window, cx));
        })
        .detach();
        cx.notify();
    }

    /// The run ended: it is kept on its post (P6) and shown in the block's panel; a runtime
    /// that did not start is also told in a toast.
    fn finish_code_run(&mut self, key: (String, usize), found: &DetectedRuntime, outcome: std::io::Result<Captured>, window: &mut Window, cx: &mut Context<Self>) {
        self.code.running.remove(&key);
        let (post, block) = key;
        if let Err(err) = &outcome {
            let note = t!("code.run.failed", runtime = found.runtime.label(), reason = err.to_string()).to_string();
            self.toast(ToastKind::Confirmation, Notification::error(note), window, cx);
        }
        let _ = self.state.save_code_run(&PostId::new(post), saved_run(block, found, &outcome)); // refused for a locked post
        cx.notify();
    }

    /// Cancel on a running block: the runner stops it with everything it started.
    pub(in crate::ui::features::shell) fn cancel_code_run(&mut self, post: String, block: usize, cx: &mut Context<Self>) {
        if let Some(cancel) = self.code.running.get(&(post, block)) {
            cancel.store(true, Ordering::Relaxed);
        }
        cx.notify();
    }
}

/// The run as it is kept on the post: its output shortened to what a post keeps.
fn saved_run(block: usize, found: &DetectedRuntime, outcome: &std::io::Result<Captured>) -> CodeRun {
    let (end, exit_code, duration, output) = match outcome {
        Ok(captured) => (run_end(captured.end), captured.exit_code, captured.duration, captured.output.clone()),
        Err(err) => (RunEnd::Failed, None, Duration::ZERO, err.to_string()),
    };
    let (output, truncated) = kept_output(&output);
    let duration_ms = u64::try_from(duration.as_millis()).unwrap_or(u64::MAX);
    CodeRun { block, runtime: found.runtime.key().into(), version: found.version.clone(), end, exit_code, duration_ms, ran_at: Some(jiff::Timestamp::now()), output, truncated }
}

fn run_end(end: End) -> RunEnd {
    match end {
        End::Exited => RunEnd::Exited,
        End::TimedOut => RunEnd::TimedOut,
        End::Cancelled => RunEnd::Cancelled,
        End::OutputCapped => RunEnd::OutputCapped,
    }
}
