//! Images attached in the composer (sb C3): picked, pasted or dropped files are copied into
//! the media folder by content hash off the UI thread, each with a field for its description.
//! Other dropped or pasted files become file chips (L12, `files`).

use super::{ComposerImage, ComposerModalEvent, ComposerModalPanel};
use crate::domain::media::{ImageAttachment, MAX_GALLERY_IMAGES};
use crate::media::{store_attachment_bytes, store_attachment_file};
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use rust_i18n::t;
use std::path::{Path, PathBuf};

/// File extensions attached as images (any other file becomes a file chip, L12).
const IMAGE_EXTENSIONS: [&str; 6] = ["png", "jpg", "jpeg", "webp", "gif", "bmp"];

/// Whether `path` names an image file the composer attaches.
pub(crate) fn is_image_path(path: &Path) -> bool {
    let extension = path.extension().and_then(|ext| ext.to_str()).map(str::to_ascii_lowercase);
    extension.is_some_and(|ext| IMAGE_EXTENSIONS.contains(&ext.as_str()))
}

/// `paths` split into the images (the image tray) and the other files (file chips, L12).
pub(crate) fn split_media(paths: Vec<PathBuf>) -> (Vec<PathBuf>, Vec<PathBuf>) {
    paths.into_iter().partition(|path| is_image_path(path))
}

/// What a paste into a draft carries besides text.
enum PastedMedia {
    Image(Vec<u8>),
    Files(Vec<PathBuf>),
}

fn pasted_media(item: &ClipboardItem) -> Option<PastedMedia> {
    item.entries().iter().find_map(|entry| match entry {
        ClipboardEntry::Image(image) => Some(PastedMedia::Image(image.bytes.clone())),
        ClipboardEntry::ExternalPaths(paths) => (!paths.paths().is_empty()).then(|| PastedMedia::Files(paths.paths().to_vec())),
        ClipboardEntry::String(_) => None,
    })
}

impl ComposerModalPanel {
    /// Where attachments are stored (the vault's media folder).
    pub fn set_media_dir(&mut self, media_dir: PathBuf) {
        self.media_dir = media_dir;
    }

    /// Attaches the image files among `paths`, up to a gallery's worth, and any other files as
    /// file chips (L12).
    pub fn attach_paths(&mut self, paths: Vec<PathBuf>, window: &mut Window, cx: &mut Context<Self>) {
        let (images, others) = split_media(paths);
        if !others.is_empty() {
            self.attach_files(others, window, cx);
        }
        for path in self.within_limit(images, cx) {
            let media_dir = self.media_dir.clone();
            self.store_in_background(move || store_attachment_file(&path, &media_dir), window, cx);
        }
    }

    /// A paste into a draft: an image or files are attached (`true`); text goes through.
    pub(super) fn paste_media(&mut self, item: &ClipboardItem, window: &mut Window, cx: &mut Context<Self>) -> bool {
        match pasted_media(item) {
            Some(PastedMedia::Image(bytes)) => {
                if !self.within_limit(vec![()], cx).is_empty() {
                    let media_dir = self.media_dir.clone();
                    self.store_in_background(move || store_attachment_bytes(&bytes, &media_dir), window, cx);
                }
                true
            }
            Some(PastedMedia::Files(paths)) => {
                self.attach_paths(paths, window, cx);
                true
            }
            None => false,
        }
    }

    /// The part of `items` that still fits, explaining the limit when some do not.
    fn within_limit<T>(&mut self, mut items: Vec<T>, cx: &mut Context<Self>) -> Vec<T> {
        let room = MAX_GALLERY_IMAGES.saturating_sub(self.images.len() + self.pending_images);
        if items.len() > room {
            self.error = Some(t!("composer.media_limit", count = MAX_GALLERY_IMAGES).to_string());
            items.truncate(room);
            cx.notify();
        }
        items
    }

    fn store_in_background(&mut self, store: impl FnOnce() -> Result<ImageAttachment, String> + Send + 'static, window: &mut Window, cx: &mut Context<Self>) {
        self.pending_images += 1;
        let task = cx.background_executor().spawn(async move { store() });
        cx.spawn_in(window, async move |this, cx| {
            let stored = task.await;
            let _ = this.update_in(cx, |this, window, cx| this.finish_attach(stored, window, cx));
        })
        .detach();
        cx.notify();
    }

    fn finish_attach(&mut self, stored: Result<ImageAttachment, String>, window: &mut Window, cx: &mut Context<Self>) {
        self.pending_images = self.pending_images.saturating_sub(1);
        match stored {
            Ok(attachment) if self.is_open && !self.images.iter().any(|image| image.attachment.path == attachment.path) => {
                let placeholder = t!("composer.media_alt_placeholder").to_string();
                let alt = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
                self.images.push(ComposerImage { attachment, alt });
            }
            Ok(_) => {}
            Err(reason) => self.error = Some(t!("composer.media_failed", reason = reason).to_string()),
        }
        cx.notify();
    }

    /// Removes the attached image at `index`.
    pub(super) fn remove_image(&mut self, index: usize, _: &mut Window, cx: &mut Context<Self>) {
        if index < self.images.len() {
            self.images.remove(index);
            cx.notify();
        }
    }

    /// The attached images with their descriptions, then a typed path that exists.
    pub(super) fn image_attachments(&self, cx: &App) -> Vec<ImageAttachment> {
        let attached = self.images.iter().map(|image| ImageAttachment { alt: image.alt.read(cx).value().trim().to_string(), ..image.attachment.clone() });
        let typed = self.typed_image(cx).map(ImageAttachment::at);
        attached.chain(typed).take(MAX_GALLERY_IMAGES).collect()
    }

    /// The attached images and their description fields, for the dialog.
    pub(super) fn image_views(&self) -> Vec<(String, Entity<InputState>)> {
        self.images.iter().map(|image| (image.attachment.path.clone(), image.alt.clone())).collect()
    }

    /// sb D3: asks the host for image ideas about the active draft's text.
    pub(super) fn image_ideas(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let text = self.drafts.get(self.active_draft).or(self.drafts.first()).map(|slot| slot.input.read(cx).value().to_string()).unwrap_or_default();
        cx.emit(ComposerModalEvent::ImageIdeas(text));
    }

    /// How many images are still being copied in.
    pub(super) fn pending_images(&self) -> usize {
        self.pending_images
    }
}
