//! The settings window's command buttons, routed by [`SettingsCommand`] id.

use super::super::view::AppView;
use crate::domain::preferences::{Preferences, ToastKind};
use crate::ui::features::settings::SettingsCommand;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;
use std::path::{Path, PathBuf};

/// Prefix of the notes of commands that later waves complete (`prefs.pending.<id>`).
const PENDING_PREFIX: &str = "prefs.pending";
/// Folder of the vault backups, next to the database.
const BACKUP_DIR: &str = "backups";
/// `simulacra-20261117-210455.db`
const BACKUP_FILE_FORMAT: &str = "simulacra-%Y%m%d-%H%M%S.db";

/// The system file manager command for a folder.
#[cfg(target_os = "windows")]
const FILE_MANAGER: &str = "explorer";
#[cfg(target_os = "macos")]
const FILE_MANAGER: &str = "open";
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
const FILE_MANAGER: &str = "xdg-open";

impl AppView {
    /// Runs one settings command.
    pub(crate) fn run_settings_command(&mut self, command: SettingsCommand, window: &mut Window, cx: &mut Context<Self>) {
        match command {
            SettingsCommand::ReplayWelcome => self.replay_welcome(window, cx),
            SettingsCommand::OpenDatabaseFolder => self.open_folder(self.state.vault_dir(), window, cx),
            SettingsCommand::OpenMediaFolder => self.open_folder(Some(self.state.media_dir()), window, cx),
            SettingsCommand::Backup => self.backup_vault(window, cx),
            SettingsCommand::ClearCaches => self.clear_caches(window, cx),
            SettingsCommand::ResetAll => self.reset_all_settings(window, cx),
            SettingsCommand::OpenKeymap => {
                self.chrome.is_settings_open = false;
                self.toggle_shortcut_sheet(window, cx);
            }
            SettingsCommand::ReindexEmbeddings | SettingsCommand::Export => {
                let key = format!("{PENDING_PREFIX}.{}", command.id());
                let note = Notification::info(t!(key.as_str()).to_string());
                window.push_notification(note, cx);
            }
        }
        cx.notify();
    }

    /// Opens `folder` (created when missing) in the system file manager.
    fn open_folder(&self, folder: Option<PathBuf>, window: &mut Window, cx: &mut App) {
        let opened = folder.ok_or_else(|| t!("prefs.ui.no_vault").to_string()).and_then(|folder| {
            std::fs::create_dir_all(&folder).map_err(|err| err.to_string())?;
            std::process::Command::new(FILE_MANAGER).arg(&folder).spawn().map(|_| ()).map_err(|err| err.to_string())
        });
        if let Err(reason) = opened {
            window.push_notification(Notification::error(t!("prefs.ui.open_failed", reason = reason).to_string()), cx);
        }
    }

    /// Writes a consistent copy of the vault into `backups/` beside it.
    fn backup_vault(&self, window: &mut Window, cx: &mut App) {
        let (Some(db), Some(dir)) = (self.state.db.as_ref(), self.state.vault_dir()) else {
            window.push_notification(Notification::error(t!("prefs.ui.no_vault").to_string()), cx);
            return;
        };
        let path = dir.join(BACKUP_DIR).join(chrono::Local::now().format(BACKUP_FILE_FORMAT).to_string());
        let target = path.clone();
        let written = std::fs::create_dir_all(dir.join(BACKUP_DIR))
            .map_err(|err| err.to_string())
            .and_then(|()| db.read(move |db| db.backup_to(&target)).map_err(|err| err.to_string()));
        match written {
            Ok(()) => self.toast(ToastKind::Confirmation, Notification::success(saved_to(&path)), window, cx),
            Err(reason) => window.push_notification(Notification::error(t!("prefs.ui.backup_failed", reason = reason).to_string()), cx),
        }
    }

    /// Forgets every cached illustration and avatar raster (they are redrawn on demand).
    fn clear_caches(&self, window: &mut Window, cx: &mut App) {
        let stale = crate::ui::illustration::forget_illustrations("");
        let count = stale.len();
        stale.into_iter().for_each(|image| image.remove_asset(cx));
        let note = Notification::success(t!("prefs.ui.caches_cleared", count = count).to_string());
        self.toast(ToastKind::Confirmation, note, window, cx);
    }

    /// First click asks for confirmation; the second restores every preference.
    fn reset_all_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.settings_ui.confirm_reset {
            self.settings_ui.confirm_reset = true;
            return;
        }
        self.apply_preferences(Preferences::default(), window, cx);
        self.settings_ui.sync_inputs(self.state.preferences(), window, cx);
        let note = Notification::success(t!("prefs.ui.reset_done").to_string());
        self.toast(ToastKind::Confirmation, note, window, cx);
    }
}

/// "Backup saved to <path>".
fn saved_to(path: &Path) -> String {
    t!("prefs.ui.backup_saved", path = path.display().to_string()).to_string()
}
