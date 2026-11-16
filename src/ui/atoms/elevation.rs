//! Theme-driven drop shadow for floating surfaces (dialogs, popovers, menus).

use crate::theme::ThemeTokens;
use gpui_kit::{px, BoxShadow, Styled};

/// Floating surfaces cast a single soft shadow straight down.
const SHADOW_OFFSET_X: f32 = 0.0;

/// Adds the theme's elevation shadow (black at a mode-aware alpha) to a styled element.
pub trait ElevationExt: Styled + Sized {
    /// Replaces the element's shadow with the theme's floating-surface shadow.
    fn elevation_shadow(self, theme: &ThemeTokens) -> Self {
        let shadow = BoxShadow::new(px(SHADOW_OFFSET_X), theme.shadow_offset_y, theme.shadow.into())
            .blur_radius(theme.shadow_blur);
        self.shadow(vec![shadow])
    }
}

impl<T: Styled + Sized> ElevationExt for T {}
