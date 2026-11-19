//! Plain English names of theme colours ("violet", "gold", "near-black"), so "match my
//! theme" can describe the active palette to an image model.

/// Below this saturation a colour is a grey.
const GREY_SATURATION: f32 = 0.18;
/// Lightness bounds of the near-black and near-white greys.
const DARK_LIGHTNESS: f32 = 0.18;
const LIGHT_LIGHTNESS: f32 = 0.82;
const FULL_TURN: f32 = 360.0;
const SEXTANT: f32 = 60.0;
const HALF: f32 = 0.5;
const CHANNEL_MAX: f32 = 255.0;

/// Upper hue bound (degrees) and name of each hue band, in order.
const HUE_BANDS: [(f32, &str); 10] = [
    (15.0, "red"),
    (40.0, "orange"),
    (65.0, "gold"),
    (90.0, "lime"),
    (168.0, "green"),
    (190.0, "teal"),
    (240.0, "blue"),
    (290.0, "violet"),
    (335.0, "magenta"),
    (360.0, "red"),
];
const NEAR_BLACK: &str = "near-black";
const NEAR_WHITE: &str = "off-white";
const GREY: &str = "grey";

/// (hue in degrees, saturation, lightness) of an sRGB colour.
pub fn hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let [r, g, b] = [r, g, b].map(|c| f32::from(c) / CHANNEL_MAX);
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let lightness = (max + min) * HALF;
    let delta = max - min;
    if delta <= f32::EPSILON {
        return (0.0, 0.0, lightness);
    }
    let saturation = delta / (1.0 - (2.0 * lightness - 1.0).abs());
    let sector = match max {
        m if m == r => ((g - b) / delta).rem_euclid(6.0),
        m if m == g => (b - r) / delta + 2.0,
        _ => (r - g) / delta + 4.0,
    };
    ((sector * SEXTANT).rem_euclid(FULL_TURN), saturation, lightness)
}

/// The plain name of an sRGB colour.
pub fn colour_name(r: u8, g: u8, b: u8) -> &'static str {
    let (hue, saturation, lightness) = hsl(r, g, b);
    match (saturation < GREY_SATURATION, lightness) {
        (_, l) if l < DARK_LIGHTNESS => NEAR_BLACK,
        (_, l) if l > LIGHT_LIGHTNESS => NEAR_WHITE,
        (true, _) => GREY,
        (false, _) => HUE_BANDS.iter().find(|(bound, _)| hue < *bound).map_or(HUE_BANDS[0].1, |(_, name)| *name),
    }
}

/// Distinct names of `colours`, in order.
pub fn distinct_names(colours: &[(u8, u8, u8)]) -> Vec<String> {
    colours.iter().map(|&(r, g, b)| colour_name(r, g, b)).fold(Vec::new(), |mut names: Vec<String>, name| {
        if !names.iter().any(|known| known == name) {
            names.push(name.to_string());
        }
        names
    })
}
