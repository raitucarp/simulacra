//! Attachment UI: link and image path input rows and the action bar (attached images are
//! listed by `media_tray`).

use super::super::consts::{
    MODAL_BROWSE_IMAGE_BTN_ID, MODAL_CLEAR_IMAGE_INPUT_BTN_ID, MODAL_CLEAR_LINK_BTN_ID,
    MODAL_IMAGE_IDEAS_BTN_ID, MODAL_IMAGE_INPUT_ROW_ID, MODAL_PICK_IMAGE_BTN_ID, MODAL_TAG_BTN_ID,
    MODAL_TOGGLE_IMAGE_INPUT_BTN_ID, MODAL_TOGGLE_LINK_BTN_ID,
};
use super::{ComposerModalView, ModalActionCallback};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Input;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

impl ComposerModalView {
    pub(super) fn link_input_row(&self) -> Div {
        let theme = &self.theme;
        input_row_frame(theme)
            .child(Icon::new(LucideIcon::Link2))
            .child(div().flex_1().child(Input::new(&self.link_input).appearance(false)))
            .child(clear_button(MODAL_CLEAR_LINK_BTN_ID, self.callbacks.on_clear_link.clone(), theme))
    }

    pub(super) fn image_input_row(&self) -> Stateful<Div> {
        let theme = &self.theme;
        let on_pick_image_browse = self.callbacks.on_pick_image.clone();
        input_row_frame(theme)
            .id(MODAL_IMAGE_INPUT_ROW_ID)
            .child(div().text_color(theme.text_muted).child(Icon::new(LucideIcon::Image)))
            .child(div().flex_1().child(Input::new(&self.image_input).appearance(false)))
            .child(
                Button::new(MODAL_BROWSE_IMAGE_BTN_ID)
                    .ghost()
                    .small()
                    .icon(Icon::new(LucideIcon::Folder))
                    .tooltip(t!("composer.pick_image").to_string())
                    .on_click(move |_, window, cx| (on_pick_image_browse)(window, cx)),
            )
            .child(clear_button(MODAL_CLEAR_IMAGE_INPUT_BTN_ID, self.callbacks.on_clear_image.clone(), theme))
    }

    pub(super) fn attachment_bar(&self) -> Div {
        let pick_image = self.callbacks.on_pick_image.clone();
        let toggle_image = self.callbacks.on_toggle_image.clone();
        let toggle_link = self.callbacks.on_toggle_link.clone();
        let insert_tag = self.callbacks.on_insert_tag.clone();
        let image_ideas = self.callbacks.on_image_ideas.clone();
        div()
            .w_full()
            .flex()
            .items_center()
            .gap(self.theme.spacing_xs)
            .pt(self.theme.spacing_xs)
            .child(
                attachment_button(MODAL_PICK_IMAGE_BTN_ID, LucideIcon::Image)
                    .tooltip(t!("composer.pick_image").to_string())
                    .on_click(move |_, window, cx| (pick_image)(window, cx)),
            )
            .child(
                attachment_button(MODAL_TOGGLE_IMAGE_INPUT_BTN_ID, LucideIcon::Folder)
                    .tooltip(t!("composer.toggle_image_input").to_string())
                    .on_click(move |_, window, cx| (toggle_image)(window, cx)),
            )
            .child(
                attachment_button(MODAL_TOGGLE_LINK_BTN_ID, LucideIcon::Link2)
                    .tooltip(t!("composer.attach_link").to_string())
                    .on_click(move |_, window, cx| (toggle_link)(window, cx)),
            )
            .child(
                attachment_button(MODAL_TAG_BTN_ID, LucideIcon::Hash)
                    .tooltip(t!("composer.insert_tag").to_string())
                    .on_click(move |_, window, cx| (insert_tag)(window, cx)),
            )
            .child(
                attachment_button(MODAL_IMAGE_IDEAS_BTN_ID, LucideIcon::Sparkles)
                    .tooltip(t!("image_ideas.open").to_string())
                    .on_click(move |_, window, cx| (image_ideas)(window, cx)),
            )
    }
}

/// Bordered inline row hosting a single attachment input.
fn input_row_frame(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border_strong)
        .bg(theme.bg_app)
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
}

/// Muted "X" glyph that clears an attachment input.
fn clear_button(
    id: &'static str,
    on_clear: ModalActionCallback,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    div()
        .id(id)
        .cursor_pointer()
        .text_color(theme.text_muted)
        .hover(move |s| s.text_color(theme.text_primary))
        .on_click(move |_, window, cx| (on_clear)(window, cx))
        .child(Icon::new(LucideIcon::X))
}

fn attachment_button(id: &'static str, icon: LucideIcon) -> Button {
    Button::new(id).ghost().small().icon(Icon::new(icon))
}
