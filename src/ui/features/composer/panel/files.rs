//! L12: any file dropped or pasted into the composer that is not an image becomes a file
//! chip (images keep their own tray). Files are linked where they are, not copied: their
//! reference (size, date, type, hash) is read off the UI thread and goes on the first post.

use super::ComposerModalPanel;
use crate::domain::media::{FileRef, MAX_FILES_PER_POST};
use crate::media::files::read_file_ref;
use gpui_kit::*;
use rust_i18n::t;
use std::path::PathBuf;

/// The files attached so far, and how many are still being read.
#[derive(Debug, Default)]
pub(super) struct ComposerFiles {
    pub(super) attached: Vec<FileRef>,
    pub(super) pending: usize,
}

/// A file that could not be read: its name and why.
type Refused = (String, String);

impl ComposerModalPanel {
    /// Attaches `paths` as files, up to a post's worth.
    pub(super) fn attach_files(&mut self, paths: Vec<PathBuf>, window: &mut Window, cx: &mut Context<Self>) {
        let room = MAX_FILES_PER_POST.saturating_sub(self.files.attached.len() + self.files.pending);
        if paths.len() > room {
            self.error = Some(t!("composer.file_limit", count = MAX_FILES_PER_POST).to_string());
        }
        for path in paths.into_iter().take(room) {
            self.files.pending += 1;
            let name = path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
            let task = cx.background_executor().spawn(async move { read_file_ref(&path).map_err(|reason| (name, reason)) });
            cx.spawn_in(window, async move |this, cx| {
                let read = task.await;
                let _ = this.update(cx, |this, cx| this.finish_file(read, cx));
            })
            .detach();
        }
        cx.notify();
    }

    fn finish_file(&mut self, read: Result<FileRef, Refused>, cx: &mut Context<Self>) {
        self.files.pending = self.files.pending.saturating_sub(1);
        match read {
            Ok(file) if self.is_open && !self.files.attached.iter().any(|known| known.path == file.path) => self.files.attached.push(file),
            Ok(_) => {}
            Err((name, reason)) => self.error = Some(t!("composer.file_failed", name = name, reason = reason).to_string()),
        }
        cx.notify();
    }

    /// Removes the attached file at `index`.
    pub(super) fn remove_file(&mut self, index: usize, _: &mut Window, cx: &mut Context<Self>) {
        if index < self.files.attached.len() {
            self.files.attached.remove(index);
            cx.notify();
        }
    }
}
