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

/// Channel distance under which two series colours read as the same hue.
const SAME_COLOUR: f32 = 0.25;
/// How far a repeated colour moves towards the text (or the card) colour to stand apart;
/// failing that, its hue turns round the wheel.
const REPEAT_SHIFT: f32 = 0.45;

/// `colour` with its hue turned by `turn` (a share of the colour wheel).
fn rotate_hue(colour: Rgba, turn: f32) -> Rgba {
    let mut hsla: gpui_kit::Hsla = colour.into();
    hsla.h = (hsla.h + turn).rem_euclid(1.0);
    hsla.to_rgb()
}

fn distance(a: Rgba, b: Rgba) -> f32 {
    (a.r - b.r).abs() + (a.g - b.g).abs() + (a.b - b.b).abs()
}

/// The categorical series colours, in order: the semantic accents, each kept legible on the
/// card surface, and a palette's near-duplicate accents (two blues) moved apart in lightness.
/// Series past the end wrap around.
pub fn categorical(theme: &ThemeTokens) -> [Rgba; SERIES_COUNT] {
    let keep = |c: Rgba| legible(c, theme.bg_card, theme.text_primary, MIN_MARK_CONTRAST);
    let accents = [theme.accent_brand, theme.accent_success, theme.accent_dialectic, theme.info, theme.accent_danger, theme.accent_alt, theme.accent_user, theme.warning];
    let mut series = [theme.accent_brand; SERIES_COUNT];
    for (i, accent) in accents.into_iter().enumerate() {
        let fresh = |c: &Rgba| series[..i].iter().all(|earlier| distance(*c, *earlier) >= SAME_COLOUR);
        let shifted = [keep(mix(accent, theme.text_primary, REPEAT_SHIFT)), keep(mix(accent, theme.bg_card, REPEAT_SHIFT))];
        let turned = (1..SERIES_COUNT).map(|k| keep(rotate_hue(accent, k as f32 / SERIES_COUNT as f32)));
        series[i] = std::iter::once(keep(accent)).chain(shifted).chain(turned).find(fresh).unwrap_or(shifted[0]);
    }
    series
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
        NodeKind::Profile => theme.accent_success,
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
