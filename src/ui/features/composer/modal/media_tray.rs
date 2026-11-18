//! Attached images of the first post (sb C3): one row each with a thumbnail, an editable
//! description (alt text) and a remove button, plus a placeholder per image still copying.

use super::super::consts::{MODAL_CLEAR_IMAGE_BTN_ID, MODAL_IMAGE_PREVIEW_CARD_ID};
use super::ComposerModalView;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;
use std::path::PathBuf;

impl ComposerModalView {
    /// The attached images, or `None` when there are none.
    pub(super) fn media_tray(&self) -> Option<Div> {
        if self.images.is_empty() && self.pending_images == 0 {
            return None;
        }
        let rows = self.images.iter().enumerate().map(|(index, (path, alt))| self.image_row(index, path, alt).into_any_element());
        let pending = (0..self.pending_images).map(|_| self.row_frame().child(self.thumb_frame().bg(self.theme.hover_bg)).into_any_element());
        Some(div().w_full().flex().flex_col().gap(self.theme.stack_gap).children(rows).children(pending))
    }

    fn row_frame(&self) -> Div {
        let theme = &self.theme;
        div()
            .w_full()
            .p(theme.spacing_xs)
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .bg(theme.bg_app)
            .flex()
            .items_center()
            .gap(theme.control_gap)
    }

    fn thumb_frame(&self) -> Div {
        div().flex_none().size(self.theme.composer_attachment_thumb_size).rounded(self.theme.radius_sm).overflow_hidden()
    }

    fn image_row(&self, index: usize, path: &str, alt: &Entity<InputState>) -> Stateful<Div> {
        let theme = &self.theme;
        let on_remove = self.callbacks.on_remove_image.clone();
        let hover = theme.text_primary;
        let description = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_3xs)
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("composer.media_alt_label").to_string()))
            .child(Input::new(alt).appearance(false));
        let remove = div()
            .id((MODAL_CLEAR_IMAGE_BTN_ID, index))
            .flex_none()
            .p(theme.spacing_2xs)
            .rounded(theme.radius_full)
            .cursor_pointer()
            .text_color(theme.text_muted)
            .hover(move |s| s.text_color(hover))
            .tooltip(move |window, cx| Tooltip::new(t!("composer.media_remove").to_string()).build(window, cx))
            .on_click(move |_, window, cx| on_remove(index, window, cx))
            .child(Icon::new(LucideIcon::X));
        let thumb = self.thumb_frame().child(img(PathBuf::from(path)).size_full().object_fit(ObjectFit::Cover));
        self.row_frame().id((MODAL_IMAGE_PREVIEW_CARD_ID, index)).child(thumb).child(description).child(remove)
    }
}
