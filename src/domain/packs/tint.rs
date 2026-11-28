//! Runtime colouring of layered raster art. Portrait and form layers are generated in key
//! colours: a neutral grey ([`PRIMARY_KEY`]) where the persona's main colour goes (skin, hair,
//! a form's body) and cyan ([`SECONDARY_KEY`]) where its second colour goes. Each key pixel
//! takes the target's hue and saturation while its lightness is remapped around the key's,
//! so shading survives; whites, blacks and other colours stay as drawn.

/// The main recolourable fill: neutral mid-grey.
pub const PRIMARY_KEY: [u8; 3] = [0x80, 0x80, 0x80];
/// The second recolourable fill: pure cyan.
pub const SECONDARY_KEY: [u8; 3] = [0x00, 0xff, 0xff];
/// Lightness of both keys.
const KEY_LIGHTNESS: f32 = 128.0 / 255.0;
/// Chroma below which a pixel counts fully as grey, and the span it fades out over.
const GREY_CHROMA: f32 = 0.18;
/// Hue of the cyan key and how far from it a pixel still counts; the chroma ramp it fades in over.
const CYAN_HUE: f32 = 180.0; const CYAN_HUE_SPAN: f32 = 35.0;
const CYAN_CHROMA_MIN: f32 = 0.15; const CYAN_CHROMA_SPAN: f32 = 0.2;
const HEX_PREFIX: char = '#';

/// Which persona colours a layer takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerRole {
    /// Drawn as is.
    Fixed,
    /// Grey takes the skin colour (`portrait/face`).
    Skin,
    /// Grey takes the hair colour (`portrait/hair`, `portrait/brows`).
    Hair,
    /// Grey takes the primary and cyan the secondary colour (`forms/<kind>/body`).
    Body,
    /// Cyan takes the secondary colour (`forms/<kind>/markings`).
    Markings,
}

impl LayerRole {
    /// The role of the layer `key` (`portrait/face/round`, `forms/plant/body/oak`).
    pub fn of_key(key: &str) -> Self {
        let parts: Vec<&str> = key.split('/').collect();
        match parts.as_slice() {
            ["portrait", "face", _] => Self::Skin,
            ["portrait", "hair" | "brows", _] => Self::Hair,
            ["forms", _, "body", _] => Self::Body,
            ["forms", _, "markings", _] => Self::Markings,
            _ => Self::Fixed,
        }
    }
}

/// Target colours for the grey and cyan keys (`None` leaves that key as drawn).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Tint {
    pub primary: Option<[u8; 3]>,
    pub secondary: Option<[u8; 3]>,
}

impl Tint {
    /// Whether the tint changes nothing.
    pub fn is_identity(&self) -> bool {
        self.primary.is_none() && self.secondary.is_none()
    }
}

/// `#rrggbb` as RGB.
pub fn parse_hex(hex: &str) -> Option<[u8; 3]> {
    let digits = hex.trim().strip_prefix(HEX_PREFIX)?;
    let value = (digits.len() == 6).then(|| u32::from_str_radix(digits, 16).ok()).flatten()?;
    Some([(value >> 16) as u8, (value >> 8) as u8, value as u8])
}

/// Recolours every key pixel of an RGBA buffer in place.
pub fn tint_rgba(pixels: &mut [u8], tint: &Tint) {
    if tint.is_identity() {
        return;
    }
    for pixel in pixels.chunks_exact_mut(4).filter(|pixel| pixel[3] > 0) {
        let [r, g, b] = tint_rgb([pixel[0], pixel[1], pixel[2]], tint);
        pixel[..3].copy_from_slice(&[r, g, b]);
    }
}

/// One pixel recoloured: blended toward each key's target by how much it looks like that key.
pub fn tint_rgb(rgb: [u8; 3], tint: &Tint) -> [u8; 3] {
    let (hue, chroma, lightness) = hcl(rgb);
    let grey = (1.0 - chroma / GREY_CHROMA).clamp(0.0, 1.0);
    let near_cyan = (1.0 - (hue - CYAN_HUE).abs() / CYAN_HUE_SPAN).clamp(0.0, 1.0);
    let cyan = near_cyan * ((chroma - CYAN_CHROMA_MIN) / CYAN_CHROMA_SPAN).clamp(0.0, 1.0);
    let mut out = rgb;
    for (weight, target) in [(cyan, tint.secondary), (grey, tint.primary)] {
        if let (true, Some(target)) = (weight > 0.0, target) {
            out = mix(out, recolour(lightness, target), weight);
        }
    }
    out
}

/// `target`'s hue and saturation at the lightness `lightness` maps to around the key's.
fn recolour(lightness: f32, target: [u8; 3]) -> [u8; 3] {
    let (hue, saturation, target_l) = hsl(target);
    let mapped = if lightness <= KEY_LIGHTNESS {
        lightness / KEY_LIGHTNESS * target_l
    } else {
        target_l + (lightness - KEY_LIGHTNESS) / (1.0 - KEY_LIGHTNESS) * (1.0 - target_l)
    };
    from_hsl(hue, saturation, mapped)
}

fn mix(from: [u8; 3], to: [u8; 3], weight: f32) -> [u8; 3] {
    let lerp = |a: u8, b: u8| (f32::from(a) + (f32::from(b) - f32::from(a)) * weight).round().clamp(0.0, 255.0) as u8;
    [lerp(from[0], to[0]), lerp(from[1], to[1]), lerp(from[2], to[2])]
}

/// Hue (degrees), chroma and lightness (0–1).
fn hcl([r, g, b]: [u8; 3]) -> (f32, f32, f32) {
    let [r, g, b] = [r, g, b].map(|c| f32::from(c) / 255.0);
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let chroma = max - min;
    let hue = if chroma <= f32::EPSILON {
        0.0
    } else if max == r {
        60.0 * ((g - b) / chroma).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / chroma + 2.0)
    } else {
        60.0 * ((r - g) / chroma + 4.0)
    };
    (hue, chroma, (max + min) / 2.0)
}

/// Hue (degrees), HSL saturation and lightness (0–1).
fn hsl(rgb: [u8; 3]) -> (f32, f32, f32) {
    let (hue, chroma, lightness) = hcl(rgb);
    let denominator = 1.0 - (2.0 * lightness - 1.0).abs();
    let saturation = if denominator <= f32::EPSILON { 0.0 } else { (chroma / denominator).min(1.0) };
    (hue, saturation, lightness)
}

fn from_hsl(hue: f32, saturation: f32, lightness: f32) -> [u8; 3] {
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x = chroma * (1.0 - ((hue / 60.0).rem_euclid(2.0) - 1.0).abs());
    let (r, g, b) = match (hue / 60.0) as u32 {
        0 => (chroma, x, 0.0),
        1 => (x, chroma, 0.0),
        2 => (0.0, chroma, x),
        3 => (0.0, x, chroma),
        4 => (x, 0.0, chroma),
        _ => (chroma, 0.0, x),
    };
    let m = lightness - chroma / 2.0;
    [r, g, b].map(|c| ((c + m) * 255.0).round().clamp(0.0, 255.0) as u8)
}
