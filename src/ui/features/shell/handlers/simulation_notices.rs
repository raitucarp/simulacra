//! User-facing toasts for generation failures (LLM errors and worker failures).
//!
//! The heuristic reply is still inserted; these notices make sure the user knows it
//! was a fallback and why. Messages are already redacted by [`LlmFailure`]. Each distinct
//! failure cause is shown once until the settings change or a generation succeeds.

use super::super::view::AppView;
use crate::agent::runtime::GLOBAL_GATE_KEY;
use crate::agent::schedule::FailureCause;
use crate::agent::LlmFailure;
use chrono::Utc;
use crate::error::AppError;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;

/// Notification id type for LLM failures; one toast per provider replaces the last.
struct LlmFailureNotice;

/// Notification id type for simulation worker failures.
struct WorkerFailureNotice;

/// Notification id type for vault (SQLite) write failures.
struct StorageFailureNotice;

impl AppView {
    /// Records `failure` against the engine `gate_key` and backs its agents off; logs it and
    /// shows it as a toast (provider + short message) only the first time its cause is seen
    /// since the last reset.
    pub(super) fn report_llm_failure(&mut self, gate_key: &str, failure: LlmFailure, window: &mut Window, cx: &mut Context<Self>) {
        let first = self.scheduler.failures.gate_mut(gate_key).record_failure(FailureCause::of(&failure), Utc::now());
        if first {
            eprintln!("simulacra: {} request failed: {}", failure.provider, failure.message);
            let provider = failure.provider.to_string();
            let title = t!("simulation.llm_failed_title", provider = provider).to_string();
            let note = if failure.missing_key {
                Notification::warning(t!("simulation.llm_missing_key").to_string())
            } else {
                Notification::error(failure.message.clone())
            };
            window.push_notification(note.title(title).id1::<LlmFailureNotice>(failure.provider.id_str()), cx);
        }
        self.scheduler.last_error = Some(failure);
    }

    /// Tells the user, once, that autonomous posting waits for an API key.
    pub(super) fn report_scheduler_needs_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let provider = self.state.llm_settings.provider;
        if !self.scheduler.failures.gate_mut(GLOBAL_GATE_KEY).notice(FailureCause::missing_key(provider)) {
            return;
        }
        let title = t!("scheduler.paused_title").to_string();
        let body = t!("scheduler.paused_no_key", provider = provider.to_string()).to_string();
        eprintln!("simulacra: {title}: {body}");
        let note = Notification::warning(body.clone()).title(title).id1::<LlmFailureNotice>(provider.id_str());
        window.push_notification(note, cx);
        self.scheduler.last_error = Some(LlmFailure::new(provider, &body, "", true));
    }

    /// Forgets every shown failure notice and backoff (settings or credentials changed), so
    /// the next failure is reported again.
    pub(super) fn reset_llm_failures(&mut self) {
        self.scheduler.failures.reset_all();
    }

    /// Shows a toast when the background simulation worker could not run at all.
    pub(super) fn report_worker_failure(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let message = t!("simulation.worker_failed").to_string();
        eprintln!("simulacra: {message}");
        window.push_notification(Notification::error(message).id::<WorkerFailureNotice>(), cx);
    }

    /// Logs a failed vault write and shows it as a toast.
    pub(crate) fn report_storage_failure(&mut self, err: &AppError, window: &mut Window, cx: &mut Context<Self>) {
        let message = t!("storage.write_failed", error = err.to_string()).to_string();
        eprintln!("simulacra: {message}");
        window.push_notification(Notification::error(message).id::<StorageFailureNotice>(), cx);
    }
}
