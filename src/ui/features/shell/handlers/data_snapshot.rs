//! "Encrypted snapshot…" in the shell (sb F2): the passphrase is read from its two fields
//! only when sealing, held in zeroizing memory, and the fields are cleared at once; it is
//! never stored, logged or written to the database. The consistent copy is taken on the
//! vault worker, then sealed and written off the UI thread.

use super::super::view::AppView;
use crate::domain::data::snapshot::{meets_minimum, strength, SNAPSHOT_EXTENSION};
use crate::domain::preferences::ToastKind;
use crate::storage::data::create_snapshot;
use crate::ui::features::data::{tr, DataEvent, SnapshotDialog};
use jiff::Timestamp;
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use rust_i18n::t;
use zeroize::Zeroizing;

const FILE_NAME_FORMAT: &str = "simulacra-%Y%m%d-%H%M%S";
/// Name of the plain copy while it waits to be sealed (beside the vault, removed at once).
const COPY_NAME_FORMAT: &str = "simulacra-snapshot-%Y%m%d-%H%M%S.tmp";

impl AppView {
    pub(in crate::ui::features::shell) fn on_snapshot_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            DataEvent::OpenSnapshot => {
                self.chrome.is_settings_open = false;
                self.clear_passphrases(window, cx);
                self.data.snapshot = Some(SnapshotDialog::new(self.data.inputs.passphrase.clone(), self.data.inputs.repeat.clone()));
                self.data.inputs.passphrase.update(cx, |input, cx| input.focus(window, cx));
            }
            DataEvent::CreateSnapshot => self.create_snapshot(window, cx),
            _ => {
                self.clear_passphrases(window, cx);
                self.data.snapshot = None;
            }
        }
    }

    /// Empties every passphrase field.
    pub(in crate::ui::features::shell) fn clear_passphrases(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let inputs = &self.data.inputs;
        for input in [&inputs.passphrase, &inputs.repeat, &inputs.restore_passphrase] {
            input.update(cx, |input, cx| input.set_value("", window, cx));
        }
    }

    /// Re-reads strength and match from the fields (nothing is kept but the verdicts).
    pub(in crate::ui::features::shell) fn refresh_snapshot_checks(&mut self, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.snapshot.as_mut() else { return };
        let passphrase = Zeroizing::new(dialog.passphrase.read(cx).value().to_string());
        let repeat = Zeroizing::new(dialog.repeat.read(cx).value().to_string());
        dialog.strength = strength(&passphrase);
        dialog.long_enough = meets_minimum(&passphrase);
        dialog.matches = !passphrase.is_empty() && *passphrase == *repeat;
        dialog.problem = None;
    }

    /// Reads the passphrase once (both fields must agree), clears the fields, asks where to
    /// save, then copies the vault and seals the copy in the background.
    fn create_snapshot(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.snapshot.as_ref() else { return };
        let passphrase = Zeroizing::new(dialog.passphrase.read(cx).value().to_string());
        let repeat = Zeroizing::new(dialog.repeat.read(cx).value().to_string());
        if dialog.busy || !meets_minimum(&passphrase) || *passphrase != *repeat {
            return;
        }
        self.clear_passphrases(window, cx);
        self.refresh_snapshot_checks(cx);
        self.set_snapshot_busy(true, cx);
        let name = format!("{}.{SNAPSHOT_EXTENSION}", crate::time::local_now().strftime(FILE_NAME_FORMAT));
        let filter = tr("data.snapshot.file_filter");
        cx.spawn_in(window, async move |view, cx| {
            let file = rfd::AsyncFileDialog::new().set_file_name(name).add_filter(filter, &[SNAPSHOT_EXTENSION]).save_file().await;
            let Some(out) = file.map(|f| f.path().to_path_buf()) else {
                let _ = view.update(cx, |this, cx| this.set_snapshot_busy(false, cx));
                return;
            };
            let Ok(Some(copy)) = view.update(cx, |this, cx| this.snapshot_copy(cx)) else { return };
            let target = out.clone();
            let sealed = cx.background_executor().spawn(async move { create_snapshot(&copy, &target, &passphrase, Timestamp::now().as_second()) }).await;
            let _ = view.update_in(cx, |this, window, cx| this.snapshot_finished(sealed.map(|bytes| (out, bytes)), window, cx));
        })
        .detach();
    }

    /// A consistent plain copy of the vault beside it (sealed and removed right after).
    fn snapshot_copy(&mut self, cx: &mut Context<Self>) -> Option<std::path::PathBuf> {
        let (vault, dir) = (self.state.db.clone()?, self.state.vault_dir()?);
        let copy = dir.join(crate::time::local_now().strftime(COPY_NAME_FORMAT).to_string());
        match vault.backup_to(copy.clone()) {
            Ok(()) => Some(copy),
            Err(err) => {
                self.snapshot_failed(err.to_string(), cx);
                None
            }
        }
    }

    fn set_snapshot_busy(&mut self, busy: bool, cx: &mut Context<Self>) {
        if let Some(dialog) = self.data.snapshot.as_mut() {
            dialog.busy = busy;
        }
        cx.notify();
    }

    fn snapshot_failed(&mut self, reason: String, cx: &mut Context<Self>) {
        if let Some(dialog) = self.data.snapshot.as_mut() {
            dialog.problem = Some(t!("data.snapshot.failed", reason = reason).to_string());
        }
        self.set_snapshot_busy(false, cx);
    }

    fn snapshot_finished(&mut self, sealed: crate::error::Result<(std::path::PathBuf, u64)>, window: &mut Window, cx: &mut Context<Self>) {
        match sealed {
            Ok((path, _)) => {
                self.data.snapshot = None;
                let note = Notification::success(t!("data.snapshot.done", path = path.display().to_string()).to_string());
                self.toast(ToastKind::Confirmation, note, window, cx);
                cx.notify();
            }
            Err(err) => self.snapshot_failed(err.to_string(), cx),
        }
    }
}
