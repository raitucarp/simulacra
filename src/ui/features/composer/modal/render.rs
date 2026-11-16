//! Modal shell: scrim backdrop and the dialog stacking header, context, editor and footer.

use super::super::consts::{MODAL_BACKDROP_ID, MODAL_DIALOG_ID};
use super::reply_context::render_reply_context;
use super::ComposerModalView;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

impl RenderOnce for ComposerModalView {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let image_path = self.effective_image_path(cx);
        let can_post = self.can_post(image_path.is_some(), cx);
        let on_close_backdrop = self.on_close.clone();
        div()
            .id(MODAL_BACKDROP_ID)
            .absolute()
            .inset_0()
            .bg(self.theme.bg_scrim)
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
                (on_close_backdrop)(window, cx);
            })
            .child(self.dialog(image_path, can_post))
    }
}

impl ComposerModalView {
    fn dialog(&self, image_path: Option<String>, can_post: bool) -> Stateful<Div> {
        let theme = &self.theme;
        div()
            .id(MODAL_DIALOG_ID)
            .w(theme.modal_composer_width)
            .max_h(theme.modal_composer_max_height)
            .rounded(theme.radius_lg)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .bg(theme.bg_elevated)
            .shadow_2xl()
            .flex()
            .flex_col()
            .overflow_hidden()
            .on_mouse_down(MouseButton::Left, |_ev, _window, cx| {
                cx.stop_propagation();
            })
            // 1. Header: Cancel, Title, Options
            .child(self.header())
            // 2. Parent Context (if Reply mode)
            .when_some(self.reply_parent(), |this, parent| {
                this.child(render_reply_context(parent, theme))
            })
            // 3. Main Composer Input Row
            .child(self.input_row(image_path, can_post))
            // 4. Modal Footer: Privacy indicator & Post button
            .child(self.footer(can_post))
    }
}
