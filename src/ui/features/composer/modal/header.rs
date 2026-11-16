//! Modal header: cancel action, mode title and options glyph.

use super::super::consts::MODAL_CANCEL_BTN_ID;
use super::ComposerModalView;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

impl ComposerModalView {
    pub(super) fn header(&self) -> Div {
        let theme = &self.theme;
        div()
            .w_full()
            .h(theme.feed_header_height)
            .px(theme.spacing_lg)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border)
            .flex()
            .items_center()
            .justify_between()
            .child(self.cancel_button())
            .child(
                div()
                    .font_family(theme.font_family_display.clone())
                    .font_weight(FontWeight::BOLD)
                    .text_size(theme.font_size_base)
                    .text_color(theme.text_primary)
                    .child(self.title_text()),
            )
            .child(
                div()
                    .text_color(theme.text_muted)
                    .child(Icon::new(LucideIcon::MoreHorizontal)),
            )
    }

    fn cancel_button(&self) -> Stateful<Div> {
        let theme = &self.theme;
        let on_close_cancel = self.callbacks.on_close.clone();
        div()
            .id(MODAL_CANCEL_BTN_ID)
            .cursor_pointer()
            .text_size(theme.font_size_sm)
            .text_color(theme.text_secondary)
            .hover(move |s| s.text_color(theme.text_primary))
            .on_click(move |_, window, cx| {
                (on_close_cancel)(window, cx);
            })
            .child(t!("composer.cancel").to_string())
    }
}
