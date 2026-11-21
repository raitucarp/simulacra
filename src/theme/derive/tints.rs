//! Tinted post containers (sb C1): a background and a border per named tint, each a mix of
//! the app background with one palette hue.
//!
//! Amber, rose, sky and mint take the palette's warning, danger, info and success colors,
//! slate its muted foreground, and violet the info color turned to a violet hue (no palette
//! slot is reliably violet). A background that would leave primary, secondary or muted text below
//! WCAG AA is moved back toward the plain background until it passes; muted text (handles,
//! timestamps) counts too, since a tinted card shows all three.

use crate::theme::color::{ensure_contrast, mix, with_hue, AA_TEXT_CONTRAST};
use crate::theme::palette::ThemePalette;
use crate::theme::ThemeTokens;
use gpui_kit::Rgba;

/// HSL hue (degrees) of the violet tint.
const VIOLET_HUE: f32 = 262.0;
/// How far a tinted background moves from the app background toward its hue, per mode
/// (dark backgrounds show a hue sooner, so they take less).
const DARK_BG_MIX: f32 = 0.14;
const LIGHT_BG_MIX: f32 = 0.12;
/// How far a tint's border moves from the app background toward its hue, per mode.
const DARK_BORDER_MIX: f32 = 0.45;
const LIGHT_BORDER_MIX: f32 = 0.4;
/// Contrast headroom over WCAG AA a background is repaired to, so rounding the colors to
/// 8-bit channels afterwards cannot drop text below AA.
const QUANTIZE_HEADROOM: f32 = 0.05;

/// The background and border of one tint over `bg`, the background kept readable for `text`.
fn tint(bg: Rgba, hue: Rgba, text: &[Rgba], dark: bool) -> (Rgba, Rgba) {
    let (bg_mix, border_mix) = if dark { (DARK_BG_MIX, DARK_BORDER_MIX) } else { (LIGHT_BG_MIX, LIGHT_BORDER_MIX) };
    let background = ensure_contrast(mix(bg, hue, bg_mix), bg, text, AA_TEXT_CONTRAST + QUANTIZE_HEADROOM);
    (background, mix(bg, hue, border_mix))
}

/// Every tint's background and border. Runs after the text colors, which it keeps readable.
pub(super) fn apply_tints(t: &mut ThemeTokens, palette: &ThemePalette) {
    let c = &palette.colors;
    let text = [t.text_primary, t.text_secondary, t.text_muted];
    let dark = palette.mode.is_dark();
    let of = |hue| tint(c.bg, hue, &text, dark);
    let ((amber_bg, amber_border), (rose_bg, rose_border), (sky_bg, sky_border)) = (of(c.warning), of(c.danger), of(c.info));
    let ((mint_bg, mint_border), (violet_bg, violet_border), (slate_bg, slate_border)) = (of(c.success), of(with_hue(c.info, VIOLET_HUE)), of(c.fg_muted));
    set_tokens!(t {
        tint_amber_bg: amber_bg, tint_amber_border: amber_border,
        tint_rose_bg: rose_bg, tint_rose_border: rose_border,
        tint_sky_bg: sky_bg, tint_sky_border: sky_border,
        tint_mint_bg: mint_bg, tint_mint_border: mint_border,
        tint_violet_bg: violet_bg, tint_violet_border: violet_border,
        tint_slate_bg: slate_bg, tint_slate_border: slate_border,
    });
}
