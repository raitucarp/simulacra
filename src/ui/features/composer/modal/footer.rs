//! Modal footer: the format toolbar and post options hint, and the primary post (or
//! "post all") button.

use super::super::consts::MODAL_SUBMIT_POST_BTN_ID;
use super::super::format::format_buttons;
use super::super::preview::preview_button;
use gpui_kit::prelude::FluentBuilder;
use super::ComposerModalView;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;

/// Id scope of the modal's format buttons.
const MODAL_FORMAT_SCOPE: &str = "modal";

impl ComposerModalView {
    pub(super) fn footer(&self, can_post: bool) -> Div {
        let theme = &self.theme;
        let on_submit_btn = self.callbacks.on_submit.clone();
        let label = if self.is_editing() {
            t!("data.edit.save")
        } else if self.drafts.len() > 1 {
            t!("composer.post_all_button")
        } else {
            t!("composer.post_button")
        };
        div()
            .w_full()
            .px(theme.panel_padding)
            .py(theme.spacing_md)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border)
            .flex()
            .items_center()
            .justify_between()
            .child(self.footer_tools())
            .child(
                Button::new(MODAL_SUBMIT_POST_BTN_ID)
                    .primary()
                    .small()
                    .disabled(!can_post)
                    .label(label.to_string())
                    .on_click(move |_, window, cx| {
                        (on_submit_btn)(window, cx);
                    }),
            )
    }

    /// The preview toggle, the format toolbar (acting on the last focused draft; hidden while
    /// previewing) and the audience hint.
    fn footer_tools(&self) -> Div {
        let theme = &self.theme;
        let hint = div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("composer.post_options").to_string());
        let toggle = preview_button(MODAL_FORMAT_SCOPE, self.preview, self.callbacks.on_toggle_preview.clone());
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .child(toggle)
            .when(!self.preview, |row| row.child(format_buttons(MODAL_FORMAT_SCOPE, self.callbacks.on_format.clone(), theme)))
            .child(hint)
    }
}
