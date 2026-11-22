//! Modal shell: scrim backdrop and the dialog stacking header, context, thread, notice and footer.

use super::super::consts::{MODAL_BACKDROP_ID, MODAL_DIALOG_ID};
use super::super::schedule_picker::schedule_row;
use super::reply_context::render_reply_context;
use super::ComposerModalView;
use crate::ui::atoms::ElevationExt;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

impl RenderOnce for ComposerModalView {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let on_close_backdrop = self.callbacks.on_close.clone();
        div()
            .id(MODAL_BACKDROP_ID)
            .absolute()
            .inset_0()
            .occlude()
            .bg(self.theme.bg_scrim)
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
                (on_close_backdrop)(window, cx);
            })
            .child(self.dialog())
    }
}

impl ComposerModalView {
    fn dialog(&self) -> Stateful<Div> {
        let theme = &self.theme;
        let drop_border = theme.accent_brand;
        div()
            .id(MODAL_DIALOG_ID)
            .w(theme.modal_composer_width)
            .max_h(theme.modal_composer_max_height)
            .rounded(theme.radius_lg)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .bg(theme.bg_elevated)
            .elevation_shadow(theme)
            .flex()
            .flex_col()
            .overflow_hidden()
            .on_mouse_down(MouseButton::Left, |_ev, _window, cx| {
                cx.stop_propagation();
            })
            // Images dragged over the dialog are attached when dropped (sb C3)
            .drag_over::<ExternalPaths>(move |style, _, _, _| style.border_color(drop_border))
            // 1. Header: Cancel, Title, Options
            .child(self.header())
            // 2. Parent Context (if Reply mode)
            .when_some(self.reply_parent(), |this, parent| {
                this.child(render_reply_context(parent, theme))
            })
            // 3. Thread: one editor per post, then "Add to thread"
            .child(self.thread_body())
            // 4. Why the thread cannot be posted (or why publishing failed)
            .when_some(self.notice.clone(), |this, notice| this.child(self.notice_row(notice)))
            // 5. The container tint of the thread (sb C3)
            .child(self.tint_row())
            // 5b. The symbols of the first post (S5)
            .child(self.symbol_row())
            // 6. When the thread goes out, while the picker is open (Y3)
            .when_some(self.schedule.as_ref(), |this, view| this.child(schedule_row(view, theme)))
            // 7. Modal Footer: Privacy indicator & Post button
            .child(self.footer(self.can_post))
    }
}
