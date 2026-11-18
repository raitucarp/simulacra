//! The choices of the image prompt builder. Every option is a [`TraitOption`]
//! (`traits.image_<group>.<key>` labels, `illustrations/image_<group>/<key>.svg` chip art)
//! with the English phrase it adds to the prompt.
//!
//! [`TraitOption`]: crate::domain::TraitOption

use crate::domain::trait_option::{trait_enum, trait_table};

trait_enum! {
    /// The look of the picture.
    pub enum ImageStyle in "image_style" {
        #[default]
        Illustration("illustration", "Illustration"),
        Watercolour("watercolour", "Watercolour"),
        Ink("ink", "Ink drawing"),
        Isometric("isometric", "Isometric"),
        Photographic("photographic", "Photographic"),
        Pixel("pixel", "Pixel art"),
        Collage("collage", "Paper collage"),
        Clay("clay", "Clay figure"),
    }
}

trait_table! {
    /// The style as prompt words.
    ImageStyle::phrase {
        Illustration => "clean flat vector illustration, soft shapes",
        Watercolour => "loose watercolour painting, soft washes, paper texture",
        Ink => "fine ink drawing, confident linework, cross-hatching",
        Isometric => "isometric 3D illustration, crisp geometry",
        Photographic => "natural photograph, shallow depth of field, soft light",
        Pixel => "detailed pixel art, limited palette, crisp pixels",
        Collage => "cut-paper collage, layered textures, torn edges",
        Clay => "handmade clay figure, stop-motion look, soft studio light",
    }
}

trait_enum! {
    /// The feeling of the picture.
    pub enum ImageMood in "image_mood" {
        #[default]
        Calm("calm", "Calm"),
        Hopeful("hopeful", "Hopeful"),
        Playful("playful", "Playful"),
        Mysterious("mysterious", "Mysterious"),
        Dramatic("dramatic", "Dramatic"),
        Melancholic("melancholic", "Melancholic"),
    }
}

trait_table! {
    /// The mood as prompt words.
    ImageMood::phrase {
        Calm => "calm, quiet atmosphere",
        Hopeful => "hopeful, warm morning light",
        Playful => "playful, lively and light-hearted",
        Mysterious => "mysterious, dim light and deep shadows",
        Dramatic => "dramatic, bold contrast and strong light",
        Melancholic => "melancholic, soft rain-grey light",
    }
}

trait_enum! {
    /// The colours of the picture.
    pub enum ImagePalette in "image_palette" {
        #[default]
        MatchTheme("match_theme", "Match my theme"),
        Warm("warm", "Warm"),
        Cool("cool", "Cool"),
        Monochrome("monochrome", "Monochrome"),
        Pastel("pastel", "Pastel"),
        Vivid("vivid", "Vivid"),
        Earthy("earthy", "Earthy"),
    }
}

trait_table! {
    /// The palette as prompt words (the theme's own colours are named separately).
    ImagePalette::phrase {
        MatchTheme => "colour palette of",
        Warm => "warm palette of reds, oranges and golds",
        Cool => "cool palette of blues, teals and violets",
        Monochrome => "monochrome palette, a single hue in many tones",
        Pastel => "soft pastel palette",
        Vivid => "vivid saturated palette",
        Earthy => "earthy palette of ochre, olive and clay",
    }
}

trait_enum! {
    /// How the subject is framed. The first three suit avatars, the rest posts.
    pub enum ImageFraming in "image_framing" {
        #[default]
        HeadShoulders("head_shoulders", "Head and shoulders"),
        Emblem("emblem", "Emblem"),
        FullFigure("full_figure", "Full figure"),
        Hero("hero", "Wide scene"),
        Diagram("diagram", "Diagram-like"),
        Abstract("abstract", "Abstract"),
    }
}

trait_table! {
    /// The framing as prompt words.
    ImageFraming::phrase {
        HeadShoulders => "head-and-shoulders portrait, centred, plain background",
        Emblem => "symbolic emblem, centred on a simple round badge",
        FullFigure => "full-figure view, standing, whole body in frame",
        Hero => "wide establishing scene, subject small in a large space",
        Diagram => "clear diagram-like composition of simple shapes, arrows and nodes",
        Abstract => "abstract composition of shapes and colour fields",
    }
}

trait_enum! {
    /// Something the picture should not contain.
    pub enum ImageAvoid in "image_avoid" {
        #[default]
        Text("text", "Text and letters"),
        Watermark("watermark", "Watermarks"),
        Distortion("distortion", "Distortions"),
        Clutter("clutter", "Clutter"),
        Blur("blur", "Blur"),
    }
}

trait_table! {
    /// The negative prompt word of the hint.
    ImageAvoid::phrase {
        Text => "text, letters, captions",
        Watermark => "watermark, signature, logo",
        Distortion => "distorted anatomy, extra limbs, warped faces",
        Clutter => "cluttered background, busy details",
        Blur => "blurry, out of focus, low quality",
    }
}

trait_table! {
    /// The hint said positively, for engines without a negative prompt.
    ImageAvoid::positive {
        Text => "no text or lettering",
        Watermark => "no watermark",
        Distortion => "correct proportions",
        Clutter => "uncluttered background",
        Blur => "sharp focus",
    }
}
