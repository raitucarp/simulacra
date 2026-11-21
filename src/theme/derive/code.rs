//! Syntax colors of code blocks: one palette hue per [`CodeStyle`](crate::domain::post::highlight::CodeStyle),
//! each nudged toward white (dark themes) or black (light themes) until it reads at WCAG AA
//! on the code fill over the feed and card surfaces.

use crate::theme::color::{composite, ensure_contrast, AA_TEXT_CONTRAST, BLACK, WHITE};
use crate::theme::palette::ThemePalette;
use crate::theme::ThemeTokens;

/// Contrast headroom over AA, so rounding colors to 8-bit channels cannot drop below it.
const QUANTIZE_HEADROOM: f32 = 0.05;

/// Keyword, type, string, number and comment colors. Runs after the surfaces.
pub(super) fn apply_code(t: &mut ThemeTokens, palette: &ThemePalette) {
    let c = &palette.colors;
    let fills = [composite(t.bg_code, c.bg), composite(t.bg_code, c.surface), composite(t.bg_code, c.elevated)];
    let extreme = if palette.mode.is_dark() { WHITE } else { BLACK };
    let readable = |hue| ensure_contrast(hue, extreme, &fills, AA_TEXT_CONTRAST + QUANTIZE_HEADROOM);
    set_tokens!(t {
        code_keyword: readable(c.accent),
        code_type: readable(c.info),
        code_string: readable(c.success),
        code_number: readable(c.warning),
        code_comment: readable(c.fg_muted),
    });
}
