//! Composer modal panel actions: attachment subscriptions, clearing, submission and the link
//! and typed-path inputs (attached images live in `media`).

use super::super::consts::{PATH_QUOTE_DOUBLE, PATH_QUOTE_SINGLE};
use super::{ComposerModalEvent, ComposerModalPanel, ComposerSubmission};
use crate::ui::components::pick_image_paths;
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use std::path::Path;
use rust_i18n::t;

impl ComposerModalPanel {
    /// Re-renders when the attachment inputs change; Enter in the image path field attaches it.
    pub(super) fn subscribe(&self, window: &mut Window, cx: &mut Context<Self>) -> Vec<Subscription> {
        let link = cx.subscribe(&self.link_input, notify_on_change);
        let image = cx.subscribe(&self.image_input, notify_on_change);
        let enter = cx.subscribe_in(&self.image_input, window, Self::on_image_path_event);
        vec![link, image, enter]
    }

    /// Attaches the typed image path on Enter and empties the field.
    fn on_image_path_event(&mut self, _: &Entity<InputState>, event: &InputEvent, window: &mut Window, cx: &mut Context<Self>) {
        if !matches!(event, InputEvent::PressEnter { .. }) {
            return;
        }
        if let Some(path) = self.typed_image(cx) {
            self.image_input.update(cx, |state, cx| state.set_value("", window, cx));
            self.attach_paths(vec![std::path::PathBuf::from(path)], window, cx);
        }
    }

    /// Resets to one empty draft and clears attachments and errors (the mode is kept).
    pub(super) fn clear(&mut self, window: &mut Window, cx: &mut App) {
        self.is_link_input_open = false;
        self.is_image_input_open = false;
        self.preview = false;
        self.images.clear();
        self.error = None;
        for input in [&self.link_input, &self.image_input] {
            input.update(cx, |state, cx| state.set_value("", window, cx));
        }
        self.drafts.truncate(1);
        self.reset_atomic_hint();
        self.reset_readability();
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
        let images = self.image_attachments(cx);
        let drafts = self.draft_texts(cx);
        let mode = self.mode.clone();
        cx.emit(ComposerModalEvent::Submit(ComposerSubmission { mode, drafts, link, images }));
    }

    /// A typed image path that points to an existing file.
    pub(super) fn typed_image(&self, cx: &App) -> Option<String> {
        let typed = self.image_input.read(cx).value();
        let manual_img = typed
            .trim()
            .trim_matches(PATH_QUOTE_DOUBLE)
            .trim_matches(PATH_QUOTE_SINGLE)
            .to_string();
        (!manual_img.is_empty() && Path::new(&manual_img).exists()).then_some(manual_img)
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

    /// Opens a native file dialog and attaches the chosen images (several at once).
    pub(super) fn pick_image(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let title = t!("composer.pick_image_title").to_string();
        cx.spawn_in(window, async move |entity, cx| {
            let paths = pick_image_paths(&title).await;
            let _ = entity.update_in(cx, |this, window, cx| this.attach_paths(paths, window, cx));
        })
        .detach();
    }

    /// Clears the typed image path and hides its input.
    pub(super) fn clear_image(&mut self, window: &mut Window, cx: &mut Context<Self>) {
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
