//! Icon set downloads in the shell (S13): one at a time, on a background thread, with its
//! progress shown in Settings › Symbols and a Cancel that stops it between chunks. A finished
//! download installs the set into the icon store; removing deletes it from there.

use super::super::view::AppView;
use crate::domain::icons::IconSet;
use crate::domain::preferences::ToastKind;
use crate::icons::{fetch_package_blocking, FetchError, IconStore, PackageSource, Progress, PACKAGE_VERSION};
use crate::ui::features::settings::{action_set, SetDownload, SymbolSetAction};
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use rust_i18n::t;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// How often the settings window redraws a running download's progress.
const PROGRESS_TICK: Duration = Duration::from_millis(250);

/// The download under way.
pub struct DownloadJob {
    pub set: &'static IconSet,
    pub progress: Arc<Mutex<Progress>>,
    pub cancel: Arc<AtomicBool>,
    /// Redraws the progress while the download runs.
    _tick: Task<()>,
}

impl DownloadJob {
    /// What the settings pane shows of it.
    pub fn view(&self) -> SetDownload {
        let progress = self.progress.lock().map(|p| *p).unwrap_or_default();
        let fraction = progress.total.filter(|total| *total > 0).map(|total| progress.received as f32 / total as f32);
        SetDownload { set: self.set.id.to_string(), fraction }
    }
}

/// Why a download did not install its set.
enum Failure {
    Fetch(FetchError),
    Install(String),
}

impl AppView {
    /// Downloads, removes or cancels from Settings › Symbols.
    pub(in crate::ui::features::shell) fn on_symbol_set_action(&mut self, action: SymbolSetAction, window: &mut Window, cx: &mut Context<Self>) {
        match (&action, action_set(&action)) {
            (SymbolSetAction::Download(_), Some(set)) if self.symbols.download.is_none() => self.start_download(set, window, cx),
            (SymbolSetAction::Remove(_), Some(set)) => self.remove_set(set, window, cx),
            (SymbolSetAction::Cancel, _) => {
                if let Some(job) = &self.symbols.download {
                    job.cancel.store(true, Ordering::Relaxed);
                }
            }
            _ => {}
        }
        cx.notify();
    }

    fn start_download(&mut self, set: &'static IconSet, window: &mut Window, cx: &mut Context<Self>) {
        let (progress, cancel) = (Arc::new(Mutex::new(Progress::default())), Arc::new(AtomicBool::new(false)));
        let (store, shared, stop) = (IconStore::new(self.state.symbols_dir()), progress.clone(), cancel.clone());
        let work = cx.background_executor().spawn(async move {
            let package = fetch_package_blocking(&PackageSource::pinned(), &stop, |p| *shared.lock().unwrap_or_else(|e| e.into_inner()) = p).map_err(Failure::Fetch)?;
            store.install(&package, set, PACKAGE_VERSION, jiff::Timestamp::now()).map_err(|e| Failure::Install(format!("{e:?}")))
        });
        cx.spawn_in(window, async move |this, cx| {
            let result = work.await;
            let _ = this.update_in(cx, |this, window, cx| this.finish_download(result.map(|_| ()), window, cx));
        })
        .detach();
        let tick = cx.spawn_in(window, async move |this, cx| loop {
            cx.background_executor().timer(PROGRESS_TICK).await;
            if !this.update(cx, |this, cx| { cx.notify(); this.symbols.download.is_some() }).unwrap_or(false) {
                break;
            }
        });
        self.symbols.error = None;
        self.symbols.download = Some(DownloadJob { set, progress, cancel, _tick: tick });
    }

    fn finish_download(&mut self, result: Result<(), Failure>, window: &mut Window, cx: &mut Context<Self>) {
        self.symbols.download = None;
        let toast = match result {
            Ok(()) => Some(t!("symbols.sets.installed").to_string()),
            Err(Failure::Fetch(FetchError::Cancelled)) => Some(t!("symbols.sets.cancelled").to_string()),
            Err(Failure::Fetch(FetchError::Integrity)) => self.symbol_failure(t!("symbols.sets.integrity").to_string()),
            Err(Failure::Fetch(other)) => self.symbol_failure(format!("{other:?}")),
            Err(Failure::Install(reason)) => self.symbol_failure(reason),
        };
        self.refresh_symbol_library(cx);
        if let Some(text) = toast {
            self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
        }
        cx.notify();
    }

    /// Keeps the failure for the settings pane; nothing to toast.
    fn symbol_failure(&mut self, reason: String) -> Option<String> {
        self.symbols.error = Some(t!("symbols.sets.failed", reason = reason).to_string());
        None
    }

    fn remove_set(&mut self, set: &IconSet, window: &mut Window, cx: &mut Context<Self>) {
        match IconStore::new(self.state.symbols_dir()).remove(set.id) {
            Ok(_) => self.toast(ToastKind::Confirmation, Notification::success(t!("symbols.sets.removed").to_string()), window, cx),
            Err(err) => self.symbols.error = Some(t!("symbols.sets.failed", reason = err.to_string()).to_string()),
        }
        self.refresh_symbol_library(cx);
    }
}
