//! The file side of the media pump (L7): scans of the files attached to the shown posts. Each
//! runs on a thread of its own, like a preview fetch: a poster from the OS can take a moment,
//! and the Windows Shell joins COM on the thread that asks it.

use super::super::view::AppView;
use crate::domain::media::files::FileScan;
use crate::domain::media::FileRef;
use crate::media::files::scan_file;
use gpui_kit::*;
use std::path::PathBuf;

/// Files scanned at the same time, at most.
const MAX_FILE_SCANS: usize = 2;
const FILE_THREAD_NAME: &str = "simulacra-file-scan";

impl AppView {
    /// Starts scans of the `files` on screen that have none yet.
    pub(in crate::ui::features::shell) fn start_file_scans(&mut self, files: &[FileRef], window: &mut Window, cx: &mut Context<Self>) {
        let room = MAX_FILE_SCANS.saturating_sub(self.state.media.files_in_flight());
        let due = self.state.media.files_due(files.iter().map(|file| file.path.as_str()), room);
        if due.is_empty() {
            return;
        }
        self.state.media = std::mem::take(&mut self.state.media).files_started(&due);
        let media_dir = self.state.media_dir();
        for file in due.iter().filter_map(|path| files.iter().find(|file| &file.path == path)) {
            self.start_file_scan(file.clone(), media_dir.clone(), window, cx);
        }
    }

    fn start_file_scan(&mut self, file: FileRef, media_dir: PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        let (reply, answer) = tokio::sync::oneshot::channel();
        let path = file.path.clone();
        let spawned = std::thread::Builder::new().name(FILE_THREAD_NAME.into()).spawn(move || {
            let _ = reply.send(scan_file(&file, &media_dir));
        });
        if spawned.is_err() {
            return;
        }
        cx.spawn_in(window, async move |this, cx| {
            let Ok(scan) = answer.await else { return };
            let _ = this.update(cx, |this, cx| this.finish_file_scan(path, scan, cx));
        })
        .detach();
    }

    fn finish_file_scan(&mut self, path: String, scan: FileScan, cx: &mut Context<Self>) {
        self.state.media = std::mem::take(&mut self.state.media).with_file(path, scan);
        cx.notify();
    }
}
