//! Modal footer: post options hint and the primary post button.

use super::super::consts::MODAL_SUBMIT_POST_BTN_ID;
use super::ComposerModalView;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;

impl ComposerModalView {
    pub(super) fn footer(&self, can_post: bool) -> Div {
        let theme = &self.theme;
        let on_submit_btn = self.on_submit.clone();
        div()
            .w_full()
            .px(theme.spacing_lg)
            .py(theme.spacing_md)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_size(theme.font_size_xs)
                    .text_color(theme.text_muted)
                    .child(t!("composer.post_options").to_string()),
            )
            .child(
                Button::new(MODAL_SUBMIT_POST_BTN_ID)
                    .primary()
                    .small()
                    .disabled(!can_post)
                    .label(t!("composer.post_button").to_string())
                    .on_click(move |_, window, cx| {
                        (on_submit_btn)(window, cx);
                    }),
            )
    }
}
