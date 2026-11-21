//! Editor pieces of a draft row: identity line, text area and the first post's attachments.

use super::super::consts::{MODAL_FALLBACK_HANDLE, MODAL_TEXTAREA_WRAPPER_ID, TOPIC_PREFIX};
use super::super::format_keys::handle_format_actions;
use super::super::keys::{is_submit_chord, SubmitPostDraft};
use super::quote_preview::render_quote_preview;
use super::{ComposerModalView, DraftEditor};
use gpui_kit::component::input::Textarea;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

impl ComposerModalView {
    /// Author handle followed by the community / topic hint.
    pub(super) fn identity_line(&self) -> Div {
        let theme = &self.theme;
        let user_handle = self
            .profile
            .as_ref()
            .map(|p| p.display_handle())
            .unwrap_or_else(|| MODAL_FALLBACK_HANDLE.to_string());
        div()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .child(
                div()
                    .font_family(theme.font_family_display.clone())
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_size(theme.font_size_sm)
                    .text_color(theme.text_primary)
                    .child(user_handle),
            )
            .child(
                div()
                    .text_size(theme.font_size_xs)
                    .text_color(theme.text_muted)
                    .child(format!("{TOPIC_PREFIX}{}", t!("composer.community_topic"))),
            )
    }

    /// Quote preview, attachment inputs, attached images and attachment bar of the first post.
    pub(super) fn first_post_extras(&self, column: Div) -> Div {
        let theme = &self.theme;
        column
            .when_some(self.quote_target(), |this, target| {
                this.child(render_quote_preview(target, theme))
            })
            .when(self.is_link_input_open, |this| this.child(self.link_input_row()))
            .when(self.is_image_input_open, |this| this.child(self.image_input_row()))
            .children(self.media_tray())
            .child(self.attachment_bar())
    }

    /// Text area wrapper that submits on the action binding or a Ctrl/Cmd+Enter keystroke;
    /// the panel validates and explains a blocked submission.
    pub(super) fn textarea(&self, index: usize, draft: &DraftEditor) -> Stateful<Div> {
        let on_submit_action = self.callbacks.on_submit.clone();
        let on_submit_key = self.callbacks.on_submit.clone();
        let on_paste = self.callbacks.on_paste.clone();
        handle_format_actions(div().id((MODAL_TEXTAREA_WRAPPER_ID, index)), &draft.input)
            .w_full()
            .on_action(move |_: &SubmitPostDraft, window, cx| (on_submit_action)(window, cx))
            .capture_key_down(move |event: &KeyDownEvent, window, cx| {
                if is_submit_chord(&event.keystroke) {
                    window.prevent_default();
                    cx.stop_propagation();
                    (on_submit_key)(window, cx);
                }
            })
            .child(Textarea::new(&draft.input).bordered(false).on_paste(move |item, window, cx| on_paste(item, window, cx)))
    }
}
