//! The lightbox: a post's images on the app background, one at a time, zoomed to fit. Esc
//! (or a click) closes it, the arrow keys (bound in the central keymap's `Lightbox` scope)
//! and buttons move through the images, and the image's description shows underneath.

mod chrome;

use super::consts::LIGHTBOX_ID;
use crate::domain::media::{fit_within, ImageAttachment, Step};
use crate::theme::ThemeTokens;
use crate::ui::keymap::LIGHTBOX_KEY_CONTEXT;
use chrome::{caption, close_button, step_button};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::path::PathBuf;
use std::rc::Rc;

actions!(simulacra_media, [CloseLightbox, PreviousImage, NextImage]);

/// What the lightbox asks its host to do.
#[derive(Clone)]
pub struct LightboxActions {
    pub on_close: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    pub on_step: Rc<dyn Fn(Step, &mut Window, &mut App) + 'static>,
}

/// The open lightbox. Image sizes come from the attachments or the media scan.
#[derive(IntoElement)]
pub struct Lightbox {
    pub theme: ThemeTokens,
    pub images: Rc<[ImageAttachment]>,
    pub index: usize,
    /// Focused while open so the keys reach it.
    pub focus: FocusHandle,
    pub actions: LightboxActions,
}

impl RenderOnce for Lightbox {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = &self.theme;
        let count = self.images.len();
        let index = self.index.min(count.saturating_sub(1));
        let image = self.images.get(index).cloned().unwrap_or_default();
        let (on_close, on_step) = (self.actions.on_close.clone(), self.actions.on_step.clone());
        let (close_key, previous_key, next_key) = (on_close.clone(), on_step.clone(), on_step.clone());
        div()
            .id(LIGHTBOX_ID)
            .absolute()
            .inset_0()
            .occlude()
            .bg(theme.bg_app)
            .key_context(LIGHTBOX_KEY_CONTEXT)
            .track_focus(&self.focus)
            .on_action(move |_: &CloseLightbox, window, cx| close_key(window, cx))
            .on_action(move |_: &PreviousImage, window, cx| previous_key(Step::Previous, window, cx))
            .on_action(move |_: &NextImage, window, cx| next_key(Step::Next, window, cx))
            .on_mouse_down(MouseButton::Left, move |_, window, cx| on_close(window, cx))
            .child(stage(&image, window.viewport_size(), theme))
            .child(close_button(self.actions.on_close.clone(), theme))
            .when(count > 1, |this| {
                this.child(step_button(Step::Previous, self.actions.on_step.clone(), theme))
                    .child(step_button(Step::Next, self.actions.on_step.clone(), theme))
            })
            .child(caption(index, count, &image.alt, theme))
    }
}

/// The image zoomed to fit the window inside the controls' margins, centred.
fn stage(image: &ImageAttachment, viewport: Size<Pixels>, theme: &ThemeTokens) -> Div {
    let (inset, bottom) = (theme.lightbox_inset, theme.lightbox_inset + theme.lightbox_button_size);
    let room = (f32::from(viewport.width - inset * 2.0), f32::from(viewport.height - inset - bottom));
    let (width, height) = image.size.map_or(room, |size| fit_within(size, room));
    div()
        .absolute()
        .top(inset)
        .bottom(bottom)
        .left(inset)
        .right(inset)
        .flex()
        .items_center()
        .justify_center()
        .child(img(PathBuf::from(&image.path)).flex_none().w(px(width)).h(px(height)).object_fit(ObjectFit::Contain))
}
