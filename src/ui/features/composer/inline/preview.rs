//! The inline composer's preview toggle: the draft shown through the feed's body renderer.

use super::super::format::format_buttons;
use super::super::preview::{preview_button, render_preview};
use super::render::{format_input, INLINE_FORMAT_SCOPE};
use super::PostComposerView;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

impl PostComposerView {
    /// The text area, or the draft as it will read once posted while previewing.
    pub(super) fn editor(&self, cx: &mut Context<Self>) -> AnyElement {
        if self.preview {
            return render_preview(&self.composer_input.read(cx).value(), &self.theme);
        }
        self.textarea(cx).into_any_element()
    }

    /// The preview toggle, then the format buttons while editing.
    pub(super) fn format_tools(&self, cx: &mut Context<Self>) -> Div {
        let entity = cx.entity();
        let toggle = preview_button(INLINE_FORMAT_SCOPE, self.preview, Rc::new(move |window, cx| entity.update(cx, |this, cx| this.toggle_preview(window, cx))));
        div()
            .flex()
            .items_center()
            .gap(self.theme.control_gap)
            .child(toggle)
            .when(!self.preview, |row| row.child(format_buttons(INLINE_FORMAT_SCOPE, format_input(&self.composer_input), &self.theme)))
    }

    /// Switches between editing and previewing; back in the editor, it gets the focus.
    fn toggle_preview(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.preview = !self.preview;
        if !self.preview {
            self.composer_input.update(cx, |state, cx| state.focus(window, cx));
        }
        cx.notify();
    }
}
