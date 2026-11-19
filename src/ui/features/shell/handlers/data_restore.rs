//! "Restore snapshot…" in the shell (sb F2): pick a file, verify it with its passphrase
//! (decrypted into a new file beside the vault and checked there), and only after the user
//! confirms, keep the current vault as a `.pre-restore` copy, swap the snapshot in, and
//! restart so every view reloads from the restored store.

use super::super::view::AppView;
use crate::domain::data::snapshot::SNAPSHOT_EXTENSION;
use crate::storage::data::{open_snapshot, pre_restore_path, staged_restore_path, StageFailure, StagedRestore};
use crate::ui::features::data::{format_time, tr, DataEvent, RestoreDialog, RestoreStep};
use gpui_kit::*;
use rust_i18n::t;
use std::path::PathBuf;
use zeroize::Zeroizing;

impl AppView {
    pub(in crate::ui::features::shell) fn on_restore_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            DataEvent::OpenRestore => {
                self.chrome.is_settings_open = false;
                self.clear_passphrases(window, cx);
                self.data.restore = Some(RestoreDialog::new(self.data.inputs.restore_passphrase.clone()));
            }
            DataEvent::PickSnapshotFile => self.pick_snapshot_file(window, cx),
            DataEvent::VerifySnapshot => self.verify_snapshot(window, cx),
            DataEvent::ConfirmRestore => self.confirm_restore(cx),
            _ => self.cancel_restore(window, cx),
        }
    }

    fn with_restore(&mut self, cx: &mut Context<Self>, change: impl FnOnce(&mut RestoreDialog)) {
        if let Some(dialog) = self.data.restore.as_mut() {
            change(dialog);
        }
        cx.notify();
    }

    fn pick_snapshot_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let filter = tr("data.snapshot.file_filter");
        cx.spawn_in(window, async move |view, cx| {
            let Some(file) = rfd::AsyncFileDialog::new().add_filter(filter, &[SNAPSHOT_EXTENSION]).pick_file().await else { return };
            let path = file.path().to_path_buf();
            let _ = view.update(cx, |this, cx| this.with_restore(cx, |dialog| (dialog.file, dialog.problem, dialog.step) = (Some(path), None, RestoreStep::Choose)));
        })
        .detach();
    }

    fn vault_path(&self) -> Option<PathBuf> {
        self.state.db.as_ref().map(|db| PathBuf::from(db.path())).filter(|path| !path.as_os_str().is_empty())
    }

    /// Opens the chosen file with the typed passphrase into a staged copy and checks it.
    fn verify_snapshot(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (Some(dialog), Some(db_path)) = (self.data.restore.as_ref(), self.vault_path()) else { return };
        let Some(file) = dialog.file.clone().filter(|_| !dialog.busy) else { return };
        let passphrase = Zeroizing::new(dialog.passphrase.read(cx).value().to_string());
        self.clear_passphrases(window, cx);
        self.with_restore(cx, |dialog| (dialog.busy, dialog.problem) = (true, None));
        let staged = staged_restore_path(&db_path);
        cx.spawn_in(window, async move |view, cx| {
            let opened = cx.background_executor().spawn(async move { open_snapshot(&file, &passphrase, &staged) }).await;
            let _ = view.update(cx, |this, cx| this.snapshot_verified(opened, cx));
        })
        .detach();
    }

    fn snapshot_verified(&mut self, opened: Result<StagedRestore, StageFailure>, cx: &mut Context<Self>) {
        match opened {
            Ok(staged) => {
                let created_at = jiff::Timestamp::from_second(staged.created_at).ok().map(format_time).unwrap_or_default();
                let summary = staged.summary;
                self.data.staged = Some(staged);
                self.with_restore(cx, |dialog| (dialog.busy, dialog.step) = (false, RestoreStep::Verified { created_at, posts: summary.posts, profiles: summary.profiles, documents: summary.documents }));
            }
            Err(failure) => {
                let problem = match failure {
                    StageFailure::Snapshot(error) => tr(&format!("data.snapshot.error.{}", error.key())),
                    StageFailure::File(reason) => t!("data.restore.file_error", reason = reason).to_string(),
                    StageFailure::NotAVault(reason) => t!("data.restore.not_a_vault", reason = reason).to_string(),
                };
                self.with_restore(cx, |dialog| (dialog.busy, dialog.problem) = (false, Some(problem)));
            }
        }
    }

    /// Keeps the current vault beside it, swaps the verified snapshot in and restarts.
    fn confirm_restore(&mut self, cx: &mut Context<Self>) {
        let (Some(staged), Some(vault), Some(db_path)) = (self.data.staged.clone(), self.state.db.clone(), self.vault_path()) else { return };
        let keep = pre_restore_path(&db_path);
        match vault.restore_database(staged.path.clone(), keep) {
            Ok(()) => {
                let _ = std::fs::remove_file(&staged.path);
                restart_app(cx);
            }
            Err(err) => self.with_restore(cx, |dialog| dialog.problem = Some(t!("data.restore.failed", reason = err.to_string()).to_string())),
        }
    }

    /// Closes the sheet; a staged copy that was never confirmed is deleted.
    fn cancel_restore(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(staged) = self.data.staged.take() {
            let _ = std::fs::remove_file(staged.path);
        }
        self.clear_passphrases(window, cx);
        self.data.restore = None;
    }
}

/// Starts a fresh copy of the app (same environment, so the same vault) and quits this one.
fn restart_app(cx: &mut App) {
    if let Ok(exe) = std::env::current_exe() {
        if let Err(err) = std::process::Command::new(exe).spawn() {
            eprintln!("simulacra: restart after restore failed: {err}");
        }
    }
    cx.quit();
}
