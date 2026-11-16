//! User-facing toasts for generation failures (LLM errors and worker failures).
//!
//! The heuristic reply is still inserted; these notices make sure the user knows it
//! was a fallback and why. Messages are already redacted by [`LlmFailure`].

use super::super::view::AppView;
use crate::agent::LlmFailure;
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
    /// Records `failure`, logs it, and shows it as a toast (provider + short message).
    pub(super) fn report_llm_failure(&mut self, failure: LlmFailure, window: &mut Window, cx: &mut Context<Self>) {
        eprintln!("simulacra: {} request failed: {}", failure.provider, failure.message);
        let provider = failure.provider.to_string();
        let title = t!("simulation.llm_failed_title", provider = provider).to_string();
        let note = if failure.missing_key {
            Notification::warning(t!("simulation.llm_missing_key").to_string())
        } else {
            Notification::error(failure.message.clone())
        };
        let note = note.title(title).id1::<LlmFailureNotice>(failure.provider.id_str());
        window.push_notification(note, cx);
        self.scheduler.last_error = Some(failure);
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
