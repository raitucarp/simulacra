//! [`Illustration`]: a themed illustration template rendered as a sized `img()`.

use super::active::with_active_cache;
use super::keys::{ClassKey, EmptyKey, GuidePose, IllustrationKind, NoticeKey, SpotKey, StepKey};
use super::template::{raster_width, TemplateExtras};
use crate::domain::TraitOption;
use gpui_kit::gpui::Image;
use gpui_kit::*;
use std::sync::Arc;

/// A themed illustration (trait card, step banner, class art, guide pose or empty state),
/// width-driven with the template's aspect ratio. Defaults to the template's canvas width.
#[derive(IntoElement)]
pub struct Illustration {
    path: String,
    kind: IllustrationKind,
    width: Pixels,
    extras: TemplateExtras,
}

impl Illustration {
    /// Any illustration asset (`illustrations/...svg`) of `kind`.
    pub fn asset(path: impl Into<String>, kind: IllustrationKind) -> Self {
        Self { path: path.into(), kind, width: px(kind.canvas().0), extras: TemplateExtras::none() }
    }

    /// Option card of a trait value (`illustrations/<group>/<key>.svg`).
    pub fn trait_card<T: TraitOption>(option: T) -> Self {
        Self::asset(option.illustration(), IllustrationKind::TraitCard)
    }

    /// Banner of a forge step.
    pub fn step(key: StepKey) -> Self {
        Self::asset(key.path(), StepKey::KIND)
    }

    /// Art of an archetype class.
    pub fn class(key: ClassKey) -> Self {
        Self::asset(key.path(), ClassKey::KIND)
    }

    /// The guide owl in `pose`.
    pub fn guide(pose: GuidePose) -> Self {
        Self::asset(pose.path(), GuidePose::KIND)
    }

    /// An empty-state scene.
    pub fn empty(key: EmptyKey) -> Self {
        Self::asset(key.path(), EmptyKey::KIND)
    }

    /// A reusable spot scene (sb D3).
    pub fn spot(key: SpotKey) -> Self {
        Self::asset(key.path(), SpotKey::KIND)
    }

    /// The art beside a notice (sb D3).
    pub fn notice(key: NoticeKey) -> Self {
        Self::asset(key.path(), NoticeKey::KIND)
    }

    /// Displayed width (height follows the canvas aspect ratio).
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }

    /// Displayed height (width follows the canvas aspect ratio).
    pub fn height(mut self, height: Pixels) -> Self {
        let (w, h) = self.kind.canvas();
        self.width = px(height.as_f32() * w / h);
        self
    }

    /// Extra placeholder fills (`{{skin}}`, `{{hair}}`).
    pub fn extras(mut self, extras: TemplateExtras) -> Self {
        self.extras = extras;
        self
    }

    /// Displayed (width, height).
    pub fn display_size(&self) -> (Pixels, Pixels) {
        let (w, h) = self.kind.canvas();
        (self.width, px(self.width.as_f32() * h / w))
    }

    /// The themed image from the UI thread's cache.
    pub fn image(&self) -> Arc<Image> {
        let width = Some(raster_width(self.width.as_f32()));
        with_active_cache(|cache, colors| cache.load_sized(&self.path, colors, &self.extras, width))
    }
}

impl RenderOnce for Illustration {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let (width, height) = self.display_size();
        img(self.image()).w(width).h(height).flex_shrink_0()
    }
}
