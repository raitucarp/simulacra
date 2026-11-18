//! "Surprise me": a random but coherent set of picks from a seed (pure, so the same seed
//! always gives the same picks). Each style lists the moods and palettes that suit it.

use super::choices::{ImageChoices, ImageTarget};
use super::options::{ImageAvoid, ImageFraming, ImageMood, ImagePalette, ImageStyle};

use ImageMood as M;
use ImagePalette as P;

/// Salts of the independent draws.
const STYLE_SALT: u64 = 1;
const MOOD_SALT: u64 = 2;
const PALETTE_SALT: u64 = 3;
const FRAMING_SALT: u64 = 4;
/// SplitMix64 constants.
const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
const MIX_A: u64 = 0xBF58_476D_1CE4_E5B9;
const MIX_B: u64 = 0x94D0_49BB_1331_11EB;
const SHIFT_A: u32 = 30;
const SHIFT_B: u32 = 27;
const SHIFT_C: u32 = 31;

/// The moods that suit `style`.
pub const fn moods_for(style: ImageStyle) -> &'static [ImageMood] {
    match style {
        ImageStyle::Illustration => &ImageMood::ALL,
        ImageStyle::Watercolour => &[M::Calm, M::Hopeful, M::Mysterious, M::Melancholic],
        ImageStyle::Ink => &[M::Calm, M::Mysterious, M::Dramatic, M::Melancholic],
        ImageStyle::Isometric => &[M::Calm, M::Hopeful, M::Playful],
        ImageStyle::Photographic => &[M::Calm, M::Hopeful, M::Mysterious, M::Dramatic, M::Melancholic],
        ImageStyle::Pixel => &[M::Hopeful, M::Playful, M::Mysterious],
        ImageStyle::Collage => &[M::Hopeful, M::Playful, M::Dramatic, M::Melancholic],
        ImageStyle::Clay => &[M::Calm, M::Hopeful, M::Playful],
    }
}

/// The palettes that suit `style`.
pub const fn palettes_for(style: ImageStyle) -> &'static [ImagePalette] {
    match style {
        ImageStyle::Illustration => &[P::MatchTheme, P::Warm, P::Cool, P::Pastel, P::Vivid],
        ImageStyle::Watercolour => &[P::Pastel, P::Cool, P::Warm, P::Earthy],
        ImageStyle::Ink => &[P::Monochrome, P::Earthy],
        ImageStyle::Isometric => &[P::MatchTheme, P::Cool, P::Vivid, P::Pastel],
        ImageStyle::Photographic => &[P::Warm, P::Cool, P::Earthy, P::Monochrome],
        ImageStyle::Pixel => &[P::MatchTheme, P::Vivid, P::Cool],
        ImageStyle::Collage => &[P::Earthy, P::Warm, P::Vivid],
        ImageStyle::Clay => &[P::Pastel, P::Warm, P::Vivid],
    }
}

/// One SplitMix64 output for `seed` and `salt`.
fn mix(seed: u64, salt: u64) -> u64 {
    let mut z = seed.wrapping_add(salt.wrapping_mul(GOLDEN_GAMMA));
    z = (z ^ (z >> SHIFT_A)).wrapping_mul(MIX_A);
    z = (z ^ (z >> SHIFT_B)).wrapping_mul(MIX_B);
    z ^ (z >> SHIFT_C)
}

fn pick<T: Copy>(items: &[T], seed: u64, salt: u64) -> T {
    items[(mix(seed, salt) % items.len() as u64) as usize]
}

/// What to keep out for this style and framing: always text and watermarks, plus what the
/// combination tends to get wrong.
pub fn avoid_for(style: ImageStyle, framing: ImageFraming) -> Vec<ImageAvoid> {
    let photo = style == ImageStyle::Photographic;
    let figure = matches!(framing, ImageFraming::FullFigure | ImageFraming::HeadShoulders);
    ImageAvoid::ALL
        .iter()
        .copied()
        .filter(|hint| match hint {
            ImageAvoid::Text | ImageAvoid::Watermark => true,
            ImageAvoid::Distortion => photo || figure,
            ImageAvoid::Clutter => framing == ImageFraming::Diagram,
            ImageAvoid::Blur => photo,
        })
        .collect()
}

/// A coherent random set of picks for `target`.
pub fn surprise(target: ImageTarget, seed: u64) -> ImageChoices {
    let style = pick(&ImageStyle::ALL, seed, STYLE_SALT);
    let framing = pick(target.framings(), seed, FRAMING_SALT);
    ImageChoices {
        style,
        mood: pick(moods_for(style), seed, MOOD_SALT),
        palette: pick(palettes_for(style), seed, PALETTE_SALT),
        framing,
        avoid: avoid_for(style, framing),
    }
}
