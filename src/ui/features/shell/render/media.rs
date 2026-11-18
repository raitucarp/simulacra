//! The lightbox overlay (sb C3), while open.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::domain::media::{ImageAttachment, ImageScan};
use crate::ui::features::media::{Lightbox, LightboxActions};
use std::rc::Rc;

impl AppView {
    /// The lightbox over the whole window, while a post's images are open in it.
    pub(super) fn lightbox(&self, bind: &Binder) -> Option<Lightbox> {
        let open = self.media.lightbox.as_ref()?;
        let actions = LightboxActions { on_close: Rc::new(bind.act(Self::close_lightbox)), on_step: Rc::new(bind.with(Self::step_lightbox)) };
        let images = open.images.iter().map(|image| self.sized(image)).collect();
        Some(Lightbox { theme: self.theme.clone(), images, index: open.index, focus: self.media.lightbox_focus.clone(), actions })
    }

    /// `image` with its pixel size filled in from the media scan when the post did not record it.
    fn sized(&self, image: &ImageAttachment) -> ImageAttachment {
        let scanned = match self.state.media.image(&image.path) {
            Some(ImageScan::Known(facts)) => Some(facts.size),
            _ => None,
        };
        ImageAttachment { size: image.size.or(scanned), ..image.clone() }
    }
}
