//! L11: relinking an attached file that moved: to the renamed copy the scan found, or to a
//! file the user picks. The new file's reference is read off the UI thread, saved on the post
//! (no new version) and both paths are scanned again.

use super::super::view::AppView;
use crate::domain::media::FileRef;
use crate::domain::preferences::ToastKind;
use crate::ui::features::media::RelinkFile;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;
use std::path::PathBuf;

impl AppView {
    /// Points a post's file at `request.to`, or at a file the user picks.
    pub(crate) fn relink_file(&mut self, request: RelinkFile, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(path) = request.to.clone() {
            return self.relink_to(request, PathBuf::from(path), window, cx);
        }
        let title = t!("media.file.relink_title").to_string();
        cx.spawn_in(window, async move |this, cx| {
            let Some(picked) = rfd::AsyncFileDialog::new().set_title(&title).pick_file().await else { return };
            let _ = this.update_in(cx, |this, window, cx| this.relink_to(request, picked.path().to_path_buf(), window, cx));
        })
        .detach();
    }

    /// Reads the file at `path` in the background, then relinks to it.
    fn relink_to(&mut self, request: RelinkFile, path: PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        let task = cx.background_executor().spawn(async move { crate::media::files::read_file_ref(&path) });
        cx.spawn_in(window, async move |this, cx| {
            let read = task.await;
            let _ = this.update_in(cx, |this, window, cx| this.finish_relink(request, read, window, cx));
        })
        .detach();
    }

    fn finish_relink(&mut self, request: RelinkFile, read: Result<FileRef, String>, window: &mut Window, cx: &mut Context<Self>) {
        let old = self.state.get_post(&request.post_id).and_then(|post| post.extras.files.get(request.index)).map(|file| file.path.clone());
        let relinked = read.and_then(|file| {
            let (path, name) = (file.path.clone(), file.name().to_string());
            self.state.relink_file(&request.post_id, request.index, file).map_err(|err| err.to_string())?;
            Ok((path, name))
        });
        match relinked {
            Ok((path, name)) => {
                let media = std::mem::take(&mut self.state.media);
                self.state.media = old.iter().fold(media.without_file(&path), |media, old| media.without_file(old));
                self.media.watched = None;
                self.toast(ToastKind::Confirmation, Notification::success(t!("media.file.relinked", name = name).to_string()), window, cx);
            }
            Err(reason) => window.push_notification(Notification::error(t!("media.file.relink_failed", reason = reason).to_string()), cx),
        }
        cx.notify();
    }
}
