//! Accent, status and tinted badge colors plus the mode-aware scrims and shadows.

use crate::theme::color::{with_alpha, BLACK};
use crate::theme::palette::ThemePalette;
use crate::theme::ThemeTokens;

/// Accent alpha of the quote/callout tint.
const CALLOUT_ALPHA: f32 = 0.07;
/// Brand badge fill / outline alphas.
const BRAND_SUBTLE_ALPHA: f32 = 0.125;
const BRAND_BORDER_ALPHA: f32 = 0.165;
/// Success badge fill / outline alphas.
const SUCCESS_SUBTLE_ALPHA: f32 = 0.08;
const SUCCESS_BORDER_ALPHA: f32 = 0.25;
/// Dialectic (gold) badge fill / outline alphas.
const DIALECTIC_SUBTLE_ALPHA: f32 = 0.125;
const DIALECTIC_BORDER_ALPHA: f32 = 0.25;

/// Black alphas of modal scrims, kit overlays and drop shadows, per mode.
struct ShadeAlphas {
    scrim: f32,
    overlay: f32,
    shadow: f32,
}

const DARK_SHADES: ShadeAlphas = ShadeAlphas { scrim: 0.87, overlay: 0.6, shadow: 0.6 };
const LIGHT_SHADES: ShadeAlphas = ShadeAlphas { scrim: 0.45, overlay: 0.3, shadow: 0.14 };

/// Brand, user, agent and status accents with their tinted fills and outlines.
pub(super) fn apply_accents(t: &mut ThemeTokens, palette: &ThemePalette) {
    let c = &palette.colors;
    set_tokens!(t {
        accent_brand: c.accent, accent_user: c.accent_2, accent_agent: c.agent,
        accent_alt: c.warning, accent_dialectic: c.warning, accent_success: c.success,
        accent_danger: c.danger, warning: c.warning, info: c.info, selection: c.selection,
        bg_window_control_close_hover: c.danger,

        bg_callout: with_alpha(c.accent, CALLOUT_ALPHA),
        accent_brand_subtle: with_alpha(c.accent, BRAND_SUBTLE_ALPHA),
        accent_brand_border: with_alpha(c.accent, BRAND_BORDER_ALPHA),
        accent_success_subtle: with_alpha(c.success, SUCCESS_SUBTLE_ALPHA),
        accent_success_border: with_alpha(c.success, SUCCESS_BORDER_ALPHA),
        accent_dialectic_subtle: with_alpha(c.warning, DIALECTIC_SUBTLE_ALPHA),
        accent_dialectic_border: with_alpha(c.warning, DIALECTIC_BORDER_ALPHA),
    });
}

/// Modal scrim, kit overlay and drop shadow: heavy on dark themes, light on light themes.
pub(super) fn apply_shades(t: &mut ThemeTokens, palette: &ThemePalette) {
    let shades = if palette.mode.is_dark() { DARK_SHADES } else { LIGHT_SHADES };
    set_tokens!(t {
        bg_scrim: with_alpha(BLACK, shades.scrim),
        overlay: with_alpha(BLACK, shades.overlay),
        shadow: with_alpha(BLACK, shades.shadow),
    });
}
