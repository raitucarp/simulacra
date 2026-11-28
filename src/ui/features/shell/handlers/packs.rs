//! Illustration pack actions from Settings › Appearance › Illustrations (WS5): picking the
//! pack in use, installing a `.simpack` from a file (read and verified on a background
//! thread, every file against its pinned SHA-256, before anything is written) and removing
//! an installed pack after a confirmation.

use super::super::view::AppView;
use crate::domain::packs::{PackError, DEFAULT_PACK_ID, MAX_PACK_BYTES, PACK_ARCHIVE_EXT};
use crate::domain::preferences::ToastKind;
use crate::packs::{read_simpack, InstalledPack, PackStore};
use crate::ui::features::settings::{pack_error_text, PackAction};
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Runs one action of the Illustrations section.
    pub(in crate::ui::features::shell) fn on_pack_action(&mut self, action: PackAction, window: &mut Window, cx: &mut Context<Self>) {
        match action {
            PackAction::Select(id) => self.select_illustration_pack(id, window, cx),
            PackAction::InstallFromFile if !self.packs.installing => self.install_pack_from_file(window, cx),
            PackAction::AskRemove(id) => self.packs.confirm_remove = Some(id),
            PackAction::CancelRemove => self.packs.confirm_remove = None,
            PackAction::ConfirmRemove(id) => self.remove_pack(&id, window, cx),
            PackAction::Download(id) => self.start_pack_download(&id, window, cx),
            PackAction::CancelDownload => self.cancel_pack_download(),
            PackAction::InstallFromFile => {}
        }
        cx.notify();
    }

    /// Makes `id` the pack in use (saved with the preferences, drawn at once).
    fn select_illustration_pack(&mut self, id: String, window: &mut Window, cx: &mut Context<Self>) {
        let mut next = self.state.preferences().clone();
        next.appearance.illustration_pack = id;
        self.packs.error = None;
        self.apply_preferences(next, window, cx);
    }

    fn install_pack_from_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (filter, store) = (t!("packs.file_filter").to_string(), PackStore::new(self.state.packs_dir()));
        self.packs.installing = true;
        cx.spawn_in(window, async move |this, cx| {
            let dialog = rfd::AsyncFileDialog::new().add_filter(filter, &[PACK_ARCHIVE_EXT]);
            let picked = dialog.pick_file().await.map(|file| file.path().to_path_buf());
            let Some(path) = picked else {
                let _ = this.update(cx, |this, cx| {
                    this.packs.installing = false;
                    cx.notify();
                });
                return;
            };
            let work = cx.background_executor().spawn(async move { install_archive(&store, &path) });
            let result = work.await;
            let _ = this.update_in(cx, |this, window, cx| this.finish_pack_install(result, window, cx));
        })
        .detach();
    }

    /// Adopts an install's outcome: reload the packs and say what happened.
    pub(in crate::ui::features::shell) fn finish_pack_install(&mut self, result: Result<InstalledPack, PackError>, window: &mut Window, cx: &mut Context<Self>) {
        self.packs.installing = false;
        match result {
            Ok(pack) => {
                self.packs.error = None;
                self.reload_illustration_packs(cx);
                let note = t!("packs.installed", name = pack.manifest.name.as_str()).to_string();
                self.toast(ToastKind::Confirmation, Notification::success(note), window, cx);
            }
            Err(err) => self.packs.error = Some(t!("packs.failed", reason = pack_error_text(&err)).to_string()),
        }
        cx.notify();
    }

    fn remove_pack(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.packs.confirm_remove = None;
        match PackStore::new(self.state.packs_dir()).remove(id) {
            Ok(_) => {
                self.packs.error = None;
                self.reload_illustration_packs(cx);
                if self.state.preferences().appearance.illustration_pack == id {
                    self.select_illustration_pack(DEFAULT_PACK_ID.to_string(), window, cx);
                }
                self.toast(ToastKind::Confirmation, Notification::success(t!("packs.removed").to_string()), window, cx);
            }
            Err(err) => self.packs.error = Some(t!("packs.failed", reason = pack_error_text(&err)).to_string()),
        }
    }
}

/// Reads, verifies and installs the archive at `path` (a background thread's work).
pub(in crate::ui::features::shell) fn install_archive(store: &PackStore, path: &std::path::Path) -> Result<InstalledPack, PackError> {
    let size = std::fs::metadata(path).map_err(|e| PackError::Io(e.to_string()))?.len();
    if size > MAX_PACK_BYTES {
        return Err(PackError::TooLarge);
    }
    let bytes = std::fs::read(path).map_err(|e| PackError::Io(e.to_string()))?;
    store.install(&read_simpack(&bytes)?)
}
