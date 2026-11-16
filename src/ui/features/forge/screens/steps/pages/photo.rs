//! Photo avatars: pick an image (the host imports it into the vault), then frame it with a
//! zoom slider and pan arrows.

use super::super::super::common::{card, muted};
use crate::domain::portrait::{AvatarMode, PhotoCrop, ZOOM_MAX, ZOOM_MIN, ZOOM_STEP};
use crate::state::forge::{AvatarField, DraftEdit};
use crate::ui::components::pick_image_path;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeEvent, ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::NumberSlider;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use std::path::PathBuf;

const IMPORT_KEY: &str = "portrait.import_photo";
const ZOOM_KEY: &str = "portrait.zoom";
const RESET_KEY: &str = "portrait.reset_crop";
const NO_PHOTO_KEY: &str = "forge.ui.appearance.no_photo";
const PICK_TITLE_KEY: &str = "forge.ui.appearance.pick_photo_title";
const IMPORT_ID: &str = "forge-photo-import";
const RESET_ID: &str = "forge-photo-reset";
const ZOOM_ID: &str = "forge-photo-zoom";
const PAN_ID_PREFIX: &str = "forge-photo-pan";
/// One pan arrow click, in per-mille of the circle diameter.
const PAN_STEP: i32 = 50;
/// Per-mille in one whole, for the "1.5×" zoom label.
const PER_MILLE_F: f32 = 1000.0;
const ZOOM_SUFFIX: &str = "×";

/// The four pan arrows: icon and (dx, dy) direction.
const PAN_ARROWS: [(LucideIcon, i32, i32); 4] = [
    (LucideIcon::ArrowLeft, -1, 0),
    (LucideIcon::ArrowUp, 0, -1),
    (LucideIcon::ArrowDown, 0, 1),
    (LucideIcon::ArrowRight, 1, 0),
];

impl ForgeView {
    /// Opens the image dialog; the chosen file goes to the host for import.
    pub(super) fn pick_photo(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let title = tr(PICK_TITLE_KEY);
        cx.spawn_in(window, async move |this, cx| {
            let Some(path) = pick_image_path(&title).await else { return };
            let _ = this.update(cx, |_, cx| cx.emit(ForgeEvent::ImportPhoto(PathBuf::from(path))));
        })
        .detach();
    }

    /// The photo's framing controls (or a prompt to import one).
    pub(super) fn photo_panel(&self, crop: Option<PhotoCrop>, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let import = handle.act(ForgeView::pick_photo);
        let import_button = Button::new(IMPORT_ID).outline().icon(LucideIcon::Image).label(tr(IMPORT_KEY)).on_click(move |_, w, cx| import(w, cx));
        let Some(crop) = crop else {
            return card(theme).child(muted(tr(NO_PHOTO_KEY), theme)).child(import_button);
        };
        let zoom = handle.with(move |this, zoom: u16, _, cx| this.set_crop(PhotoCrop { zoom, ..crop }.clamped(), cx));
        let slider = NumberSlider::new(ZOOM_ID, tr(ZOOM_KEY), crop.zoom, (ZOOM_MIN, ZOOM_MAX, ZOOM_STEP), theme, move |v, w, cx| zoom(v, w, cx))
            .format(|zoom| format!("{:.1}{ZOOM_SUFFIX}", f32::from(zoom) / PER_MILLE_F));
        let reset = handle.act(|this, _, cx| this.set_crop(PhotoCrop::default(), cx));
        let arrows = PAN_ARROWS.iter().enumerate().map(|(at, &(icon, dx, dy))| {
            let pan = handle.act(move |this, _, cx| this.set_crop(crop.panned(dx * PAN_STEP, dy * PAN_STEP), cx));
            Button::new((PAN_ID_PREFIX, at)).outline().small().icon(icon).on_click(move |_, w, cx| pan(w, cx))
        });
        card(theme)
            .child(import_button)
            .child(slider)
            .child(div().flex().gap(theme.spacing_xs).children(arrows))
            .child(Button::new(RESET_ID).ghost().small().icon(LucideIcon::RefreshCw).label(tr(RESET_KEY)).on_click(move |_, w, cx| reset(w, cx)))
    }

    /// Re-frames the current photo (one undo step).
    fn set_crop(&mut self, crop: PhotoCrop, cx: &mut Context<Self>) {
        if let Some(AvatarMode::Photo { path, .. }) = self.draft.profile().config.avatar.clone() {
            self.apply_edit(DraftEdit::Avatar(AvatarField::Avatar, Some(AvatarMode::Photo { path, crop })), cx);
        }
    }
}
