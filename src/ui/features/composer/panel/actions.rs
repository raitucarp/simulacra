//! Composer modal panel actions: attachment subscriptions, clearing, submission and attachments.

use super::super::consts::{PATH_QUOTE_DOUBLE, PATH_QUOTE_SINGLE};
use super::{ComposerModalEvent, ComposerModalPanel, ComposerSubmission};
use crate::ui::components::pick_image_path;
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use std::path::Path;
use rust_i18n::t;

impl ComposerModalPanel {
    /// Re-renders when the attachment inputs change.
    pub(super) fn subscribe(&self, cx: &mut Context<Self>) -> Vec<Subscription> {
        let link = cx.subscribe(&self.link_input, notify_on_change);
        let image = cx.subscribe(&self.image_input, notify_on_change);
        vec![link, image]
    }

    /// Resets to one empty draft and clears attachments and errors (the mode is kept).
    pub(super) fn clear(&mut self, window: &mut Window, cx: &mut App) {
        self.is_link_input_open = false;
        self.is_image_input_open = false;
        self.image_path = None;
        self.error = None;
        for input in [&self.link_input, &self.image_input] {
            input.update(cx, |state, cx| state.set_value("", window, cx));
        }
        self.drafts.truncate(1);
        if let Some(first) = self.drafts.first() {
            first.input.update(cx, |state, cx| state.set_value("", window, cx));
        }
    }

    /// Emits the thread for publishing, or re-renders to explain why it cannot be posted.
    pub fn submit(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if self.submit_problem(cx).is_some() {
            cx.notify();
            return;
        }
        let link_val = self.link_input.read(cx).value().to_string();
        let link = (!link_val.trim().is_empty()).then(|| link_val.trim().to_string());
        let image = self.image_attachment(cx);
        let drafts = self.draft_texts(cx);
        let mode = self.mode.clone();
        cx.emit(ComposerModalEvent::Submit(ComposerSubmission { mode, drafts, link, image }));
    }

    /// The picked image, or a typed path that points to an existing file.
    pub(super) fn image_attachment(&self, cx: &App) -> Option<String> {
        let typed = self.image_input.read(cx).value();
        let manual_img = typed
            .trim()
            .trim_matches(PATH_QUOTE_DOUBLE)
            .trim_matches(PATH_QUOTE_SINGLE)
            .to_string();
        self.image_path.clone().or_else(|| {
            (!manual_img.is_empty() && Path::new(&manual_img).exists()).then_some(manual_img)
        })
    }

    /// Shows or hides the link input, focusing it when shown.
    pub(super) fn toggle_link(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_link_input_open = !self.is_link_input_open;
        if self.is_link_input_open {
            self.link_input.update(cx, |st, cx| st.focus(window, cx));
        }
        cx.notify();
    }

    /// Shows or hides the image path input, focusing it when shown.
    pub(super) fn toggle_image(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_image_input_open = !self.is_image_input_open;
        if self.is_image_input_open {
            self.image_input.update(cx, |st, cx| st.focus(window, cx));
        }
        cx.notify();
    }

    /// Opens a native file dialog and attaches the chosen image.
    pub(super) fn pick_image(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let title = t!("composer.pick_image_title").to_string();
        cx.spawn(async move |entity, cx| {
            let Some(path) = pick_image_path(&title).await else { return };
            let _ = entity.update(cx, |this, cx| {
                if Path::new(&path).exists() {
                    this.image_path = Some(path);
                    this.is_image_input_open = true;
                    cx.notify();
                }
            });
        })
        .detach();
    }

    /// Removes the attached image and hides the path input.
    pub(super) fn clear_image(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.image_path = None;
        self.is_image_input_open = false;
        self.image_input.update(cx, |st, cx| st.set_value("", window, cx));
        cx.notify();
    }

    /// Clears and hides the link input.
    pub(super) fn clear_link(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.link_input.update(cx, |st, cx| st.set_value("", window, cx));
        self.is_link_input_open = false;
        cx.notify();
    }
}

fn notify_on_change(
    _: &mut ComposerModalPanel,
    _: Entity<InputState>,
    event: &InputEvent,
    cx: &mut Context<ComposerModalPanel>,
) {
    if matches!(event, InputEvent::Change) {
        cx.notify();
    }
}
