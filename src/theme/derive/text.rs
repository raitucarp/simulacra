//! Text colors, contrast-repaired against every surface they are drawn on.

use crate::theme::color::{ensure_contrast, mix, readable_on, AA_TEXT_CONTRAST, AA_UI_CONTRAST, BLACK, WHITE};
use crate::theme::palette::ThemePalette;
use crate::theme::ThemeTokens;

/// How far secondary text moves from the muted color toward the primary foreground.
const SECONDARY_TEXT_MIX: f32 = 0.3;
/// The brand tint of a selected card or tab (matches `accent_brand_subtle`).
const SELECTED_TINT: f32 = 0.125;

/// Primary/secondary/muted text, the link colour (L1), plus the readable foregrounds on accent, danger and scrims.
///
/// Text that misses WCAG AA on the app background, card or elevated surface is nudged toward
/// white (dark themes) or black (light themes) just far enough to pass.
pub(super) fn apply_text(t: &mut ThemeTokens, palette: &ThemePalette) {
    let c = &palette.colors;
    let surfaces = [c.bg, c.surface, c.elevated];
    let extreme = if palette.mode.is_dark() { WHITE } else { BLACK };
    let readable = |color| ensure_contrast(color, extreme, &surfaces, AA_TEXT_CONTRAST);
    set_tokens!(t {
        text_primary: readable(c.fg),
        text_secondary: readable(mix(c.fg_muted, c.fg, SECONDARY_TEXT_MIX)),
        text_muted: readable(c.fg_muted),
        link_text: readable(c.accent),
        accent_brand_text: ensure_contrast(c.accent, extreme, &[c.bg, c.surface, c.elevated, mix(c.surface, c.accent, SELECTED_TINT), mix(c.elevated, c.accent, SELECTED_TINT)], AA_TEXT_CONTRAST),
        on_accent: readable_on(c.accent, WHITE, BLACK, AA_UI_CONTRAST),
        on_danger: readable_on(c.danger, WHITE, BLACK, AA_UI_CONTRAST),
        on_overlay: WHITE,
    });
}
