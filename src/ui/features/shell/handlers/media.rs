//! Post media handlers (sb C3): the lightbox (open, step, close), files dropped while the
//! composer is open (attached instead of imported) and clearing the media cache.

use super::super::view::AppView;
use crate::domain::media::{step_index, Step};
use crate::ui::features::media::GalleryOpen;
use gpui_kit::*;
use std::path::PathBuf;

impl AppView {
    /// Opens the lightbox on one image of a post and gives it the keyboard.
    pub(crate) fn open_lightbox(&mut self, open: GalleryOpen, window: &mut Window, cx: &mut Context<Self>) {
        if open.images.is_empty() {
            return;
        }
        let index = open.index.min(open.images.len() - 1);
        self.media.lightbox = Some(GalleryOpen { index, ..open });
        window.focus(&self.media.lightbox_focus, cx);
        cx.notify();
    }

    pub(crate) fn close_lightbox(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.media.lightbox = None;
        window.focus(&self.palette.shell_focus, cx);
        cx.notify();
    }

    /// Shows the previous or next image, wrapping around.
    pub(crate) fn step_lightbox(&mut self, step: Step, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(open) = self.media.lightbox.as_mut() {
            open.index = step_index(open.index, open.images.len(), step);
            cx.notify();
        }
    }

    /// Files dropped on the window: attached to the open composer, otherwise imported.
    pub(crate) fn drop_files(&mut self, paths: Vec<PathBuf>, window: &mut Window, cx: &mut Context<Self>) {
        if !self.composer_modal.read(cx).is_open() {
            return self.import_paths(paths, window, cx);
        }
        self.import.dragging = false;
        self.composer_modal.update(cx, |panel, cx| panel.attach_paths(paths, window, cx));
        cx.notify();
    }

    /// Forgets every cached link preview and image scan; the shown posts fetch them again.
    pub(crate) fn clear_media_cache(&mut self) {
        self.state.media = std::mem::take(&mut self.state.media).cleared();
        self.media.watched = None;
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.clear_media_cache();
        }
    }
}
