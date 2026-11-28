//! Downloading a listed illustration pack (WS5): one at a time, on a background thread, with
//! progress in Settings › Appearance › Illustrations and a Cancel between chunks. The archive
//! must match the SHA-256 the built-in list pins, and then every file its own pinned
//! checksum, before the pack is installed.

use super::super::view::AppView;
use crate::domain::packs::builtin_downloadable_packs;
use crate::icons::{FetchError, Progress};
use crate::packs::{download_pack_blocking, PackDownloadError, PackStore};
use crate::ui::features::settings::SetDownload;
use gpui_kit::*;
use rust_i18n::t;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// How often the settings window redraws a running download's progress.
const PROGRESS_TICK: Duration = Duration::from_millis(250);

/// The pack download under way.
pub struct PackDownloadJob {
    pub id: String,
    pub progress: Arc<Mutex<Progress>>,
    pub cancel: Arc<AtomicBool>,
    /// Redraws the progress while the download runs.
    _tick: Task<()>,
}

impl PackDownloadJob {
    /// What the settings section shows of it.
    pub fn view(&self) -> SetDownload {
        let progress = self.progress.lock().map(|p| *p).unwrap_or_default();
        let fraction = progress.total.filter(|total| *total > 0).map(|total| progress.received as f32 / total as f32);
        SetDownload { set: self.id.clone(), fraction }
    }
}

impl AppView {
    /// Starts downloading listed pack `id` (nothing while another download runs).
    pub(super) fn start_pack_download(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(pack) = builtin_downloadable_packs().into_iter().find(|pack| pack.id == id) else { return };
        if self.packs.download.is_some() {
            return;
        }
        let (progress, cancel) = (Arc::new(Mutex::new(Progress::default())), Arc::new(AtomicBool::new(false)));
        let (store, shared, stop) = (PackStore::new(self.state.packs_dir()), progress.clone(), cancel.clone());
        let work = cx.background_executor().spawn(async move {
            let verified = download_pack_blocking(&pack, false, &stop, |p| *shared.lock().unwrap_or_else(|e| e.into_inner()) = p)?;
            store.install(&verified).map_err(PackDownloadError::Pack)
        });
        cx.spawn_in(window, async move |this, cx| {
            let result = work.await;
            let _ = this.update_in(cx, |this, window, cx| this.finish_pack_download(result, window, cx));
        })
        .detach();
        let tick = cx.spawn_in(window, async move |this, cx| loop {
            cx.background_executor().timer(PROGRESS_TICK).await;
            if !this.update(cx, |this, cx| { cx.notify(); this.packs.download.is_some() }).unwrap_or(false) {
                break;
            }
        });
        self.packs.error = None;
        self.packs.download = Some(PackDownloadJob { id: id.to_string(), progress, cancel, _tick: tick });
    }

    /// Stops the download under way between chunks.
    pub(super) fn cancel_pack_download(&mut self) {
        if let Some(job) = &self.packs.download {
            job.cancel.store(true, Ordering::Relaxed);
        }
    }

    fn finish_pack_download(&mut self, result: Result<crate::packs::InstalledPack, PackDownloadError>, window: &mut Window, cx: &mut Context<Self>) {
        self.packs.download = None;
        match result {
            Ok(pack) => self.finish_pack_install(Ok(pack), window, cx),
            Err(PackDownloadError::Pack(err)) => self.finish_pack_install(Err(err), window, cx),
            Err(PackDownloadError::Fetch(FetchError::Cancelled)) => self.packs.error = Some(t!("packs.cancelled").to_string()),
            Err(PackDownloadError::Fetch(FetchError::Integrity)) => self.packs.error = Some(t!("packs.failed", reason = t!("packs.error.integrity")).to_string()),
            Err(PackDownloadError::Fetch(other)) => {
                let reason = t!("packs.error.download", detail = format!("{other:?}"));
                self.packs.error = Some(t!("packs.failed", reason = reason).to_string());
            }
        }
        cx.notify();
    }
}
