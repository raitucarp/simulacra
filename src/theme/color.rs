//! sRGB color math for theme derivation: mixing, alpha, hue, hex text and WCAG 2.x contrast.

mod hue;

pub use hue::with_hue;

use gpui_kit::Rgba;

/// Byte scale of one 8-bit channel.
const CHANNEL_MAX: f32 = 255.0;
/// sRGB transfer function breakpoint, linear segment slope and gamma curve constants.
const SRGB_BREAKPOINT: f32 = 0.04045;
const SRGB_LINEAR_SLOPE: f32 = 12.92;
const SRGB_OFFSET: f32 = 0.055;
const SRGB_SCALE: f32 = 1.055;
const SRGB_GAMMA: f32 = 2.4;
/// Rec. 709 luminance weights used by WCAG relative luminance.
const LUMA_RED: f32 = 0.2126;
const LUMA_GREEN: f32 = 0.7152;
const LUMA_BLUE: f32 = 0.0722;
/// Flare term WCAG adds to both luminances of a contrast ratio.
const CONTRAST_FLARE: f32 = 0.05;
/// Fraction moved toward the target per contrast-repair step.
const CONTRAST_STEP: f32 = 0.05;
/// Steps needed to reach the target from any start (1 / step).
const CONTRAST_MAX_STEPS: u32 = 20;
/// Hex digit prefix of a color literal.
const HEX_PREFIX: char = '#';

/// WCAG 2.x minimum contrast for normal text (AA), enhanced text (AAA) and large text / UI.
pub const AA_TEXT_CONTRAST: f32 = 4.5;
pub const AAA_TEXT_CONTRAST: f32 = 7.0;
pub const AA_UI_CONTRAST: f32 = 3.0;

/// Pure white and pure black, the end points every contrast repair can fall back to.
pub const WHITE: Rgba = Rgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
pub const BLACK: Rgba = Rgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
/// Fully transparent black.
pub const TRANSPARENT: Rgba = Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

/// Linear interpolation of every channel (alpha included) from `from` toward `to`.
pub fn mix(from: Rgba, to: Rgba, amount: f32) -> Rgba {
    let t = amount.clamp(0.0, 1.0);
    let lerp = |a: f32, b: f32| a + (b - a) * t;
    Rgba { r: lerp(from.r, to.r), g: lerp(from.g, to.g), b: lerp(from.b, to.b), a: lerp(from.a, to.a) }
}

/// `color` with its alpha replaced by `alpha`.
pub fn with_alpha(color: Rgba, alpha: f32) -> Rgba {
    Rgba { a: alpha.clamp(0.0, 1.0), ..color }
}

/// `over` alpha-composited onto an opaque `under`.
pub fn composite(over: Rgba, under: Rgba) -> Rgba {
    let opaque = Rgba { a: 1.0, ..over };
    Rgba { a: 1.0, ..mix(Rgba { a: 1.0, ..under }, opaque, over.a) }
}

fn linear_channel(value: f32) -> f32 {
    if value <= SRGB_BREAKPOINT {
        value / SRGB_LINEAR_SLOPE
    } else {
        ((value + SRGB_OFFSET) / SRGB_SCALE).powf(SRGB_GAMMA)
    }
}

/// WCAG relative luminance of the color's opaque RGB.
pub fn relative_luminance(color: Rgba) -> f32 {
    LUMA_RED * linear_channel(color.r)
        + LUMA_GREEN * linear_channel(color.g)
        + LUMA_BLUE * linear_channel(color.b)
}

/// WCAG contrast ratio (1.0 – 21.0) between two colors; alpha is ignored.
pub fn contrast_ratio(a: Rgba, b: Rgba) -> f32 {
    let (la, lb) = (relative_luminance(a), relative_luminance(b));
    (la.max(lb) + CONTRAST_FLARE) / (la.min(lb) + CONTRAST_FLARE)
}

/// Lowest contrast of `color` against every background.
pub fn min_contrast(color: Rgba, backgrounds: &[Rgba]) -> f32 {
    backgrounds.iter().map(|bg| contrast_ratio(color, *bg)).fold(f32::INFINITY, f32::min)
}

/// `color` nudged toward `target` in small steps until it reaches `minimum` contrast against
/// every background (unchanged when it already does).
pub fn ensure_contrast(color: Rgba, target: Rgba, backgrounds: &[Rgba], minimum: f32) -> Rgba {
    (0..=CONTRAST_MAX_STEPS)
        .map(|step| mix(color, target, step as f32 * CONTRAST_STEP))
        .find(|candidate| min_contrast(*candidate, backgrounds) >= minimum)
        .unwrap_or(target)
}

/// The light end point when it reaches `minimum` against `background`, else the dark one.
pub fn readable_on(background: Rgba, light: Rgba, dark: Rgba, minimum: f32) -> Rgba {
    if contrast_ratio(light, background) >= minimum {
        light
    } else {
        dark
    }
}

fn channel_byte(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * CHANNEL_MAX).round() as u8
}

/// `#rrggbb` text for an opaque color, `#rrggbbaa` otherwise.
pub fn to_hex(color: Rgba) -> String {
    let [r, g, b, a] = [color.r, color.g, color.b, color.a].map(channel_byte);
    if a == u8::MAX {
        format!("{HEX_PREFIX}{r:02x}{g:02x}{b:02x}")
    } else {
        format!("{HEX_PREFIX}{r:02x}{g:02x}{b:02x}{a:02x}")
    }
}

/// Parses `#rgb`, `#rrggbb` or `#rrggbbaa`.
pub fn parse_hex(text: &str) -> Option<Rgba> {
    let trimmed = text.trim();
    trimmed.starts_with(HEX_PREFIX).then(|| Rgba::try_from(trimmed).ok()).flatten()
}

/// The color quantized to 8-bit channels (what persistence and the GPU actually keep).
pub fn quantized(color: Rgba) -> Rgba {
    let q = |v: f32| channel_byte(v) as f32 / CHANNEL_MAX;
    Rgba { r: q(color.r), g: q(color.g), b: q(color.b), a: q(color.a) }
}

#[cfg(test)]
mod tests;
