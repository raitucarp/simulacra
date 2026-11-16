//! Main input row: author avatar, identity line, textarea, quote preview and attachments.

use super::super::avatar::composer_avatar;
use super::super::consts::{
    MODAL_FALLBACK_HANDLE, MODAL_FALLBACK_INITIAL, MODAL_TEXTAREA_WRAPPER_ID, TOPIC_PREFIX,
};
use super::super::keys::{is_submit_chord, SubmitPostDraft};
use super::quote_preview::render_quote_preview;
use super::ComposerModalView;
use gpui_kit::component::input::Textarea;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

impl ComposerModalView {
    pub(super) fn input_row(&self, image_path: Option<String>, can_post: bool) -> Div {
        let theme = &self.theme;
        let avatar = composer_avatar(
            self.profile.as_ref(),
            MODAL_FALLBACK_INITIAL,
            theme.avatar_size_md,
            theme,
        );
        div()
            .w_full()
            .px(theme.spacing_lg)
            .py(theme.spacing_md)
            .flex()
            .gap(theme.spacing_md)
            .child(
                div()
                    .w(theme.post_avatar_col_width)
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(avatar),
            )
            .child(self.input_column(image_path, can_post))
    }

    fn input_column(&self, image_path: Option<String>, can_post: bool) -> Div {
        let theme = &self.theme;
        div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(self.identity_line())
            // Textarea wrapper with Ctrl+Enter interceptor
            .child(self.textarea(can_post))
            // Quoted Post Preview (if Quote mode)
            .when_some(self.quote_target(), |this, target| {
                this.child(render_quote_preview(target, theme))
            })
            // Dedicated Link Input Row (if open)
            .when(self.is_link_input_open, |this| this.child(self.link_input_row()))
            // Dedicated Image Path Input Row (if open)
            .when(self.is_image_input_open, |this| this.child(self.image_input_row()))
            // Local Image Preview Card (when a valid local file path exists)
            .when_some(image_path.as_ref(), |this, path| {
                this.child(self.image_preview(path))
            })
            // Attachment Action Bar
            .child(self.attachment_bar())
    }

    fn identity_line(&self) -> Div {
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

    fn textarea(&self, can_post: bool) -> Stateful<Div> {
        let on_submit_action = self.on_submit.clone();
        let on_submit_key = self.on_submit.clone();
        div()
            .id(MODAL_TEXTAREA_WRAPPER_ID)
            .w_full()
            .on_action(move |_: &SubmitPostDraft, window, cx| {
                if can_post {
                    (on_submit_action)(window, cx);
                }
            })
            .capture_key_down(move |event: &KeyDownEvent, window, cx| {
                if is_submit_chord(&event.keystroke) {
                    window.prevent_default();
                    cx.stop_propagation();
                    if can_post {
                        (on_submit_key)(window, cx);
                    }
                }
            })
            .child(Textarea::new(&self.composer_input).bordered(false))
    }
}
