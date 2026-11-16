//! One gpui-component color picker per base palette color, kept in step with the settings.

use super::AppearancePanel;
use crate::theme::color::{quantized, to_hex};
use crate::theme::{PaletteColorKey, ThemeSettings};
use gpui_kit::component::color_picker::{ColorPickerEvent, ColorPickerState};
use gpui_kit::{AppContext, Context, Entity, Rgba, Subscription, Window};

/// Picker states (one per [`PaletteColorKey`]) and their change subscriptions.
pub(super) fn build(
    settings: &ThemeSettings,
    window: &mut Window,
    cx: &mut Context<AppearancePanel>,
) -> (Vec<(PaletteColorKey, Entity<ColorPickerState>)>, Vec<Subscription>) {
    let colors = settings.effective_palette().colors;
    let mut pickers = Vec::with_capacity(PaletteColorKey::ALL.len());
    let mut subscriptions = Vec::with_capacity(PaletteColorKey::ALL.len());
    for &key in PaletteColorKey::ALL {
        let value = key.get(&colors);
        let state = cx.new(|cx| ColorPickerState::new(window, cx).default_value(value));
        subscriptions.push(cx.subscribe_in(&state, window, move |pane, _, event: &ColorPickerEvent, window, cx| {
            let ColorPickerEvent::Change(Some(picked)) = event else { return };
            pane.picked(key, Rgba::from(*picked), window, cx);
        }));
        pickers.push((key, state));
    }
    (pickers, subscriptions)
}

impl AppearancePanel {
    /// A picker committed `color` for `key`; ignored when it equals the current color.
    fn picked(&mut self, key: PaletteColorKey, color: Rgba, window: &mut Window, cx: &mut Context<Self>) {
        let current = key.get(&self.settings.effective_palette().colors);
        let color = quantized(color);
        if to_hex(color) != to_hex(current) {
            self.set_color(key, color, window, cx);
        }
    }

    /// Points every picker at the effective palette's colors (without emitting changes).
    pub(super) fn sync_pickers(&self, window: &mut Window, cx: &mut Context<Self>) {
        let colors = self.settings.effective_palette().colors;
        for (key, state) in &self.pickers {
            let value = key.get(&colors);
            let shown = state.read(cx).value().map(Rgba::from);
            if shown.map(to_hex) != Some(to_hex(value)) {
                state.update(cx, |picker, cx| picker.set_value(value, window, cx));
            }
        }
    }
}
