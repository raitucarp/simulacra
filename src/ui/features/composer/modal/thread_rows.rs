//! Scrollable thread body (one row per post plus "Add to thread") and the notice line.

use super::super::avatar::composer_avatar;
use super::super::consts::{MODAL_ADD_DRAFT_BTN_ID, MODAL_FALLBACK_INITIAL, MODAL_THREAD_SCROLL_ID};
use super::super::messages::render_notice;
use super::ComposerModalView;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

impl ComposerModalView {
    /// Every draft row followed by the "Add to thread" row; shrinks (default flex-shrink) and
    /// scrolls once the dialog reaches its maximum height.
    pub(super) fn thread_body(&self) -> Stateful<Div> {
        let rows: Vec<Div> = self
            .drafts
            .iter()
            .enumerate()
            .map(|(index, draft)| self.draft_row(index, draft))
            .collect();
        div()
            .id(MODAL_THREAD_SCROLL_ID)
            .w_full()
            .min_h_0()
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .children(rows)
            .children((!self.is_editing()).then(|| self.add_row()))
    }

    /// Small avatar and "Add to thread" (disabled until the last post has text).
    fn add_row(&self) -> Div {
        let theme = &self.theme;
        let on_add = self.callbacks.on_add_draft.clone();
        let can_add = self.drafts.last().is_some_and(|draft| draft.length > 0);
        div()
            .w_full()
            .px(theme.panel_padding)
            .py(theme.spacing_md)
            .flex()
            .items_center()
            .gap(theme.spacing_md)
            .child(
                div()
                    .w(theme.post_avatar_col_width)
                    .flex()
                    .justify_center()
                    .child(composer_avatar(
                        self.profile.as_ref(),
                        MODAL_FALLBACK_INITIAL,
                        theme.avatar_size_sm,
                        theme,
                    )),
            )
            .child(
                Button::new(MODAL_ADD_DRAFT_BTN_ID)
                    .ghost()
                    .small()
                    .icon(Icon::new(LucideIcon::Plus))
                    .label(t!("composer.add_to_thread").to_string())
                    .disabled(!can_add)
                    .on_click(move |_, window, cx| (on_add)(window, cx)),
            )
    }

    /// Publish error or the reason posting is blocked, shown under the editors.
    pub(super) fn notice_row(&self, notice: String) -> Div {
        let theme = &self.theme;
        render_notice(notice, theme).px(theme.panel_padding).py(theme.spacing_xs)
    }
}
