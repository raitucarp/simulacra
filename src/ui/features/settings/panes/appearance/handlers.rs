//! User actions of the Appearance pane and the callback adapter that reaches them.

use super::filter::ModeFilter;
use super::AppearancePanel;
use crate::theme::{Density, PaletteColorKey, RadiusScale, TypeScale};
use gpui_kit::{App, Context, Rgba, WeakEntity, Window};
use rust_i18n::t;

/// Weak handle element callbacks capture (elements never keep the pane alive).
pub(super) type PanelHandle = WeakEntity<AppearancePanel>;

/// A `(window, app)` callback running `action` on the pane, if it still exists.
pub(super) fn act(
    handle: &PanelHandle,
    action: impl Fn(&mut AppearancePanel, &mut Window, &mut Context<AppearancePanel>) + 'static,
) -> impl Fn(&mut Window, &mut App) + 'static {
    let handle = handle.clone();
    move |window, cx| {
        let _ = handle.update(cx, |pane, cx| action(pane, window, cx));
    }
}

impl AppearancePanel {
    /// Activates palette `id`.
    pub(super) fn select(&mut self, id: String, window: &mut Window, cx: &mut Context<Self>) {
        self.edit(window, cx, |s| s.select(&id));
    }

    /// Copies the active palette into a new, editable custom palette.
    pub(super) fn duplicate(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let name = t!("appearance.duplicate_name", name = self.settings.base_palette().name.as_str()).to_string();
        self.edit(window, cx, |s| {
            s.duplicate_active(name);
        });
    }

    /// Drops edits and restores the custom palette to its bundled origin.
    pub(super) fn reset(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.edit(window, cx, |s| s.reset_to_preset());
    }

    /// Deletes the active custom palette.
    pub(super) fn delete_active(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id = self.settings.preset_id.clone();
        self.edit(window, cx, |s| s.delete_custom(&id));
    }

    /// Recolors one base color of the active custom palette.
    pub(super) fn set_color(&mut self, key: PaletteColorKey, color: Rgba, window: &mut Window, cx: &mut Context<Self>) {
        self.edit(window, cx, |s| {
            s.set_color(key, color);
        });
    }

    pub(super) fn set_filter(&mut self, filter: ModeFilter, cx: &mut Context<Self>) {
        self.filter = filter;
        cx.notify();
    }

    pub(super) fn set_font(&mut self, family: &'static str, window: &mut Window, cx: &mut Context<Self>) {
        self.edit(window, cx, |s| s.font_family = family.to_string());
    }

    pub(super) fn set_type_scale(&mut self, scale: TypeScale, window: &mut Window, cx: &mut Context<Self>) {
        self.edit(window, cx, |s| s.type_scale = scale);
    }

    pub(super) fn set_density(&mut self, density: Density, window: &mut Window, cx: &mut Context<Self>) {
        self.edit(window, cx, |s| s.density = density);
    }

    pub(super) fn set_radius(&mut self, radius: RadiusScale, window: &mut Window, cx: &mut Context<Self>) {
        self.edit(window, cx, |s| s.radius_scale = radius);
    }
}
