//! Chart colours, all taken from theme tokens so they follow the palette in dark and light
//! mode: a categorical series order (the semantic accents, which every palette keeps legible
//! on its surfaces) and a sequential ramp (one accent at the heatmap opacities).

use crate::domain::graph::NodeKind;
use crate::domain::insights::PostKind;
use crate::theme::ThemeTokens;
use gpui_kit::Rgba;

/// Number of categorical series colours.
pub const SERIES_COUNT: usize = 8;

/// Contrast (WCAG) that chart marks keep against the card surface: the graphics minimum.
pub const MIN_MARK_CONTRAST: f32 = 3.0;
/// Steps of the blend towards the text colour when a mark is too faint.
const CONTRAST_STEPS: usize = 10;

/// WCAG relative luminance of an sRGB colour.
pub fn luminance(c: Rgba) -> f32 {
    let channel = |v: f32| if v <= 0.039_28 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) };
    0.2126 * channel(c.r) + 0.7152 * channel(c.g) + 0.0722 * channel(c.b)
}

/// WCAG contrast ratio of two opaque colours (1–21).
pub fn contrast(a: Rgba, b: Rgba) -> f32 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

/// `colour`, blended towards `toward` just enough to reach `ratio` against `surface`.
pub fn legible(colour: Rgba, surface: Rgba, toward: Rgba, ratio: f32) -> Rgba {
    (0..=CONTRAST_STEPS)
        .map(|step| mix(colour, toward, step as f32 / CONTRAST_STEPS as f32))
        .find(|c| contrast(*c, surface) >= ratio)
        .unwrap_or(toward)
}

/// The categorical series colours, in order (each kept legible on the card surface); series
/// past the end wrap around.
pub fn categorical(theme: &ThemeTokens) -> [Rgba; SERIES_COUNT] {
    let keep = |c: Rgba| legible(c, theme.bg_card, theme.text_primary, MIN_MARK_CONTRAST);
    [
        keep(theme.accent_brand),
        keep(theme.accent_success),
        keep(theme.accent_dialectic),
        keep(theme.info),
        keep(theme.accent_danger),
        keep(theme.accent_alt),
        keep(theme.accent_user),
        keep(theme.warning),
    ]
}

/// Colour of series `index`.
pub fn series(index: usize, theme: &ThemeTokens) -> Rgba {
    let colours = categorical(theme);
    colours[index % colours.len()]
}

/// Colour of a post kind (stable across views).
pub fn kind_colour(kind: PostKind, theme: &ThemeTokens) -> Rgba {
    series(kind.index(), theme)
}

/// Colour of a graph node kind (the Mind Graph's mapping, so both views agree).
pub fn node_kind_colour(kind: NodeKind, theme: &ThemeTokens) -> Rgba {
    match kind {
        NodeKind::Post => theme.accent_agent,
        NodeKind::Document => theme.accent_user,
        NodeKind::Hashtag => theme.accent_alt,
        NodeKind::Profile => theme.accent_dialectic,
    }
}

/// `colour` with its alpha scaled by `opacity`.
pub fn faded(colour: Rgba, opacity: f32) -> Rgba {
    Rgba { a: colour.a * opacity, ..colour }
}

/// The sequential ramp at `share` (0–1): empty cells use the subtle border tone, then the
/// brand accent at the four heatmap opacities.
pub fn sequential(share: f32, theme: &ThemeTokens) -> Rgba {
    let levels = [theme.cadence_alpha_1, theme.cadence_alpha_2, theme.cadence_alpha_3, theme.cadence_alpha_4];
    if share <= 0.0 {
        return theme.border_subtle;
    }
    let level = ((share.clamp(0.0, 1.0) * levels.len() as f32).ceil() as usize).clamp(1, levels.len());
    faded(theme.accent_brand, levels[level - 1])
}

/// Linear blend of two colours.
pub fn mix(a: Rgba, b: Rgba, t: f32) -> Rgba {
    let t = t.clamp(0.0, 1.0);
    Rgba { r: a.r + (b.r - a.r) * t, g: a.g + (b.g - a.g) * t, b: a.b + (b.b - a.b) * t, a: a.a + (b.a - a.a) * t }
}
