//! Quick capture, paste anywhere and the Inbox: dumped text and images are saved at once,
//! pasted files go to the import sheet.

use super::super::import_state::ImportUi;
use super::super::view::AppView;
use crate::domain::preferences::ToastKind;
use crate::import::parse::sha256_hex;
use crate::state::{FeedTab, NavigationTab};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;

/// Folder under the media folder for captured images.
const CAPTURES_DIR: &str = "captures";
/// Characters of the checksum used as a captured image's file name.
const NAME_HASH_CHARS: usize = 16;

fn image_extension(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Png => "png",
        ImageFormat::Jpeg => "jpg",
        ImageFormat::Webp => "webp",
        ImageFormat::Gif => "gif",
        ImageFormat::Svg => "svg",
        ImageFormat::Bmp => "bmp",
        ImageFormat::Tiff => "tiff",
        ImageFormat::Ico => "ico",
        ImageFormat::Pnm => "pnm",
    }
}

impl AppView {
    pub(crate) fn open_quick_capture(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.import.capture_open = true;
        self.import.capture_input.update(cx, |state, cx| state.focus(window, cx));
        cx.notify();
    }

    pub(crate) fn close_quick_capture(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.import.capture_open = false;
        cx.notify();
    }

    /// Saves the capture box's text to the Inbox and empties it (the box stays open).
    pub(crate) fn save_capture(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = ImportUi::text(&self.import.capture_input, cx);
        if self.capture_text(&text, window, cx) {
            self.import.capture_input.update(cx, |state, cx| state.set_value("", window, cx));
        }
    }

    /// Saves `text` as a capture, with a toast either way; whether it was saved.
    fn capture_text(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.capture_with_image(text, None, window, cx)
    }

    fn capture_with_image(&mut self, text: &str, image: Option<String>, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if image.is_none() && !crate::state::AppState::is_capturable(text) {
            return false;
        }
        let saved = self.state.capture(text, image);
        match &saved {
            Ok(_) => self.toast(ToastKind::Confirmation, Notification::success(t!("capture.saved").to_string()), window, cx),
            Err(err) => window.push_notification(Notification::error(t!("capture.failed", reason = err.to_string()).to_string()), cx),
        }
        cx.notify();
        saved.is_ok()
    }

    /// Paste with nothing focused: files open the import sheet, an image or text is captured.
    pub(crate) fn paste_anywhere(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(item) = cx.read_from_clipboard() else { return };
        let Some(entry) = item.entries().first().cloned() else { return };
        match entry {
            ClipboardEntry::ExternalPaths(paths) => self.import_paths(paths.paths().to_vec(), window, cx),
            ClipboardEntry::Image(image) => self.capture_image(&image, window, cx),
            ClipboardEntry::String(text) => {
                self.capture_text(text.text(), window, cx);
            }
        }
    }

    /// Stores a pasted image under `<media>/captures/` and captures it as an image post.
    fn capture_image(&mut self, image: &Image, window: &mut Window, cx: &mut Context<Self>) {
        let hash: String = sha256_hex(&image.bytes).chars().take(NAME_HASH_CHARS).collect();
        let path = self.state.media_dir().join(CAPTURES_DIR).join(format!("{hash}.{}", image_extension(image.format)));
        let written = path.parent().map_or(Ok(()), std::fs::create_dir_all).and_then(|()| std::fs::write(&path, &image.bytes));
        match written {
            Ok(()) => {
                self.capture_with_image("", Some(path.display().to_string()), window, cx);
            }
            Err(err) => window.push_notification(Notification::error(t!("capture.failed", reason = err.to_string()).to_string()), cx),
        }
    }

    /// Shows the Inbox: the timeline filtered to captured posts.
    pub(crate) fn open_inbox(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.import.capture_open = false;
        self.select_nav(NavigationTab::Timeline, window, cx);
        self.select_feed_tab(FeedTab::Inbox, window, cx);
    }
}
