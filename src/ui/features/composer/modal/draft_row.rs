//! One post of the thread: avatar with threadline, editor, and its meter / split / remove row.

use super::super::avatar::composer_avatar;
use super::super::consts::{MODAL_FALLBACK_INITIAL, MODAL_REMOVE_DRAFT_BTN_ID, MODAL_SPLIT_DRAFT_BTN_ID};
use super::{ComposerModalView, DraftEditor};
use crate::domain::MAX_POST_CHARS;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_char_meter;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

impl ComposerModalView {
    pub(super) fn draft_row(&self, index: usize, draft: &DraftEditor) -> Div {
        let theme = &self.theme;
        let column = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(self.identity_line())
            .child(self.draft_body(index, draft));
        let column = if index == 0 && !self.is_editing() { self.first_post_extras(column) } else { column };
        let hint = self.atomic_hint.as_ref().filter(|hint| hint.draft == index).map(|hint| crate::ui::features::thinking::atomic::render_atomic_hint(&hint.report, hint.on_split.clone(), hint.on_dismiss.clone(), theme));
        let meter = self.readability.as_ref().filter(|meter| meter.draft == index).map(|meter| super::super::panel::readability_meter::render_readability_meter(meter, theme));
        let column = column.children(hint).children(meter);
        div()
            .w_full()
            .px(theme.panel_padding)
            .pt(theme.spacing_md)
            .flex()
            .gap(theme.spacing_md)
            .child(self.avatar_with_threadline(theme.avatar_size_md))
            .child(column.child(self.meter_row(index, draft.length)))
    }

    /// Author avatar with the threadline leading down to the next post of the thread.
    pub(super) fn avatar_with_threadline(&self, size: Pixels) -> Div {
        let theme = &self.theme;
        div()
            .w(theme.post_avatar_col_width)
            .flex()
            .flex_col()
            .items_center()
            .child(composer_avatar(self.profile.as_ref(), MODAL_FALLBACK_INITIAL, size, theme))
            .child(threadline_segment(theme))
    }

    /// Split suggestion (when over the limit) on the left; meter and remove button on the right.
    fn meter_row(&self, index: usize, length: usize) -> Div {
        let theme = &self.theme;
        let on_split = self.callbacks.on_split_draft.clone();
        let on_remove = self.callbacks.on_remove_draft.clone();
        let split = Button::new((MODAL_SPLIT_DRAFT_BTN_ID, index))
            .ghost()
            .xsmall()
            .icon(Icon::new(LucideIcon::Sparkles))
            .label(t!("composer.split_into_thread").to_string())
            .on_click(move |_, window, cx| (on_split)(index, window, cx));
        let remove = Button::new((MODAL_REMOVE_DRAFT_BTN_ID, index))
            .ghost()
            .xsmall()
            .icon(Icon::new(LucideIcon::X))
            .tooltip(t!("composer.remove_post").to_string())
            .on_click(move |_, window, cx| (on_remove)(index, window, cx));
        div()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .gap(theme.spacing_sm)
            .child(div().when(length > MAX_POST_CHARS, |this| this.child(split)))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(theme.spacing_sm)
                    .child(render_char_meter(length, theme))
                    .when(index > 0, |this| this.child(remove)),
            )
    }
}

/// Vertical threadline filling the rest of the avatar column.
fn threadline_segment(theme: &ThemeTokens) -> Div {
    div()
        .w(theme.border_width_threadline)
        .flex_1()
        .min_h(theme.threadline_min_height)
        .bg(theme.border_subtle)
        .rounded(theme.radius_full)
        .mt(theme.spacing_2xs)
}
