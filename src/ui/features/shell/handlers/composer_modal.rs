//! Composer modal handlers: open, close, attachments and mode-aware submission.

use super::super::consts::{IMAGE_EXTENSIONS, IMAGE_FILTER_NAME, PATH_QUOTE_DOUBLE, PATH_QUOTE_SINGLE, PICK_IMAGE_TITLE};
use super::super::view::AppView;
use crate::ui::features::composer::ComposerModalMode;
use gpui_kit::*;
use std::path::Path;

impl AppView {
    /// Opens the Threads-style composer modal in the specified mode (NewThread, Reply, Quote).
    pub fn open_composer_modal(
        &mut self,
        mode: ComposerModalMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.composer_modal.mode = mode;
        self.composer_modal.is_open = true;
        self.composer_modal.clear(window, cx);
        self.composer_modal.composer_input.update(cx, |state, cx| state.focus(window, cx));
        cx.notify();
    }

    /// Closes the composer modal and clears modal state.
    pub fn close_composer_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.composer_modal.is_open = false;
        self.composer_modal.clear(window, cx);
        cx.notify();
    }

    /// Submits the composer modal content according to its mode (NewThread, Reply, Quote).
    pub fn submit_composer_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let modal = &self.composer_modal;
        let content = modal.composer_input.read(cx).value().to_string();
        let link_val = modal.link_input.read(cx).value().to_string();
        let link_opt = (!link_val.trim().is_empty()).then(|| link_val.trim().to_string());
        let image_opt = self.modal_image_attachment(cx);
        if content.trim().is_empty() && link_opt.is_none() && image_opt.is_none() {
            return;
        }
        let result = match &self.composer_modal.mode {
            ComposerModalMode::NewThread => {
                self.state.submit_post_with_attachments(&content, link_opt, image_opt, None, None)
            }
            ComposerModalMode::Reply(parent) => {
                self.state.submit_reply(&parent.id, &content, link_opt, image_opt)
            }
            ComposerModalMode::Quote(quoted) => {
                self.state.submit_quote(&quoted.id, &content, link_opt, image_opt)
            }
        };
        if let Ok(post_id) = result {
            if !matches!(self.composer_modal.mode, ComposerModalMode::Reply(_)) {
                self.feed.selected_thread_post_id = Some(post_id);
            }
            self.close_composer_modal(window, cx);
        }
    }

    /// The picked image, or a typed path that points to an existing file.
    fn modal_image_attachment(&self, cx: &App) -> Option<String> {
        let typed = self.composer_modal.image_input.read(cx).value();
        let manual_img = typed
            .trim()
            .trim_matches(PATH_QUOTE_DOUBLE)
            .trim_matches(PATH_QUOTE_SINGLE)
            .to_string();
        self.composer_modal.image_path.clone().or_else(|| {
            (!manual_img.is_empty() && Path::new(&manual_img).exists()).then_some(manual_img)
        })
    }

    /// Shows or hides the link input, focusing it when shown.
    pub(crate) fn toggle_modal_link(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let modal = &mut self.composer_modal;
        modal.is_link_input_open = !modal.is_link_input_open;
        if modal.is_link_input_open {
            modal.link_input.update(cx, |st, cx| st.focus(window, cx));
        }
        cx.notify();
    }

    /// Shows or hides the image path input, focusing it when shown.
    pub(crate) fn toggle_modal_image(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let modal = &mut self.composer_modal;
        modal.is_image_input_open = !modal.is_image_input_open;
        if modal.is_image_input_open {
            modal.image_input.update(cx, |st, cx| st.focus(window, cx));
        }
        cx.notify();
    }

    /// Opens a native file dialog and attaches the chosen image.
    pub(crate) fn pick_modal_image(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        cx.spawn(async |entity, cx| {
            let file = rfd::AsyncFileDialog::new()
                .set_title(PICK_IMAGE_TITLE)
                .add_filter(IMAGE_FILTER_NAME, &IMAGE_EXTENSIONS)
                .pick_file()
                .await;
            let Some(file_handle) = file else { return };
            let path = file_handle.path().to_string_lossy().to_string();
            let _ = entity.update(cx, |this, cx| {
                if Path::new(&path).exists() {
                    this.composer_modal.image_path = Some(path);
                    this.composer_modal.is_image_input_open = true;
                    cx.notify();
                }
            });
        })
        .detach();
    }

    /// Removes the attached image and hides the path input.
    pub(crate) fn clear_modal_image(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.composer_modal.image_path = None;
        self.composer_modal.is_image_input_open = false;
        self.composer_modal.image_input.update(cx, |st, cx| st.set_value("", window, cx));
        cx.notify();
    }

    /// Clears and hides the link input.
    pub(crate) fn clear_modal_link(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.composer_modal.link_input.update(cx, |st, cx| st.set_value("", window, cx));
        self.composer_modal.is_link_input_open = false;
        cx.notify();
    }
}
