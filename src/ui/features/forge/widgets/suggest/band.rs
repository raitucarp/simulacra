//! The typical band drawn under a slider's track: the usual range tinted, a tick at the
//! suggested value, and "Typical 0.60–0.90 · suggested 0.75" underneath.

use crate::state::forge::suggest::RangeHint;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::tr_with;
use gpui_kit::*;

const TYPICAL_KEY: &str = "suggest.ui.typical_suggested";
const ARG_LOW: &str = "low";
const ARG_HIGH: &str = "high";
const ARG_VALUE: &str = "value";

/// Where `value` sits along the valid range, from 0 to 1.
pub fn band_fraction(hint: &RangeHint, value: u16) -> f32 {
    let (min, max) = hint.valid;
    if max <= min {
        return f32::default();
    }
    f32::from(value.clamp(min, max) - min) / f32::from(max - min)
}

/// The caption under the band.
pub fn band_caption(hint: &RangeHint) -> String {
    let unit = hint.unit;
    let args = [(ARG_LOW, unit.format(hint.typical.0)), (ARG_HIGH, unit.format(hint.typical.1)), (ARG_VALUE, unit.format(hint.recommended))];
    tr_with(TYPICAL_KEY, &args)
}

/// The band strip and its caption, as wide as the slider above it.
pub fn typical_band(hint: &RangeHint, theme: &ThemeTokens) -> Div {
    let (low, high, tick) = (band_fraction(hint, hint.typical.0), band_fraction(hint, hint.typical.1), band_fraction(hint, hint.recommended));
    let band = div()
        .absolute()
        .top_0()
        .h_full()
        .left(relative(low))
        .w(relative(high - low))
        .rounded(theme.radius_full)
        .bg(theme.accent_dialectic_border);
    let marker = div().absolute().top_0().h_full().left(relative(tick)).w(theme.suggest_tick_width).bg(theme.accent_dialectic);
    let strip = div()
        .relative()
        .w_full()
        .h(theme.suggest_band_height)
        .rounded(theme.radius_full)
        .bg(theme.border_subtle)
        .child(band)
        .child(marker);
    let caption = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(band_caption(hint));
    div().flex().flex_col().gap(theme.spacing_2xs).child(strip).child(caption)
}
