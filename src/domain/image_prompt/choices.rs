//! What the picture is for ([`ImageTarget`]) and the user's picks ([`ImageChoices`]).

use super::options::{ImageAvoid, ImageFraming, ImageMood, ImagePalette, ImageStyle};

/// An avatar (agent or persona) or a picture for a post.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageTarget {
    Avatar,
    Post,
}

const AVATAR_FRAMINGS: [ImageFraming; 3] = [ImageFraming::HeadShoulders, ImageFraming::Emblem, ImageFraming::FullFigure];
const POST_FRAMINGS: [ImageFraming; 3] = [ImageFraming::Hero, ImageFraming::Diagram, ImageFraming::Abstract];
/// Avatars are square, posts wide.
const AVATAR_ASPECT: &str = "square";

impl ImageTarget {
    /// The framings offered for this target, the first being the default.
    pub const fn framings(self) -> &'static [ImageFraming] {
        match self {
            Self::Avatar => &AVATAR_FRAMINGS,
            Self::Post => &POST_FRAMINGS,
        }
    }

    /// Whether `framing` belongs to this target.
    pub fn allows(self, framing: ImageFraming) -> bool {
        self.framings().contains(&framing)
    }

    /// The aspect forced on this target (`None`: the preference decides).
    pub const fn forced_aspect(self) -> Option<&'static str> {
        match self {
            Self::Avatar => Some(AVATAR_ASPECT),
            Self::Post => None,
        }
    }
}

/// One full set of picks. Immutable: every change returns a new value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageChoices {
    pub style: ImageStyle,
    pub mood: ImageMood,
    pub palette: ImagePalette,
    pub framing: ImageFraming,
    /// What to keep out, in [`ImageAvoid::ALL`] order without repeats.
    pub avoid: Vec<ImageAvoid>,
}

impl ImageChoices {
    /// The defaults of `target`: an illustration, calm, in the theme's colours, no text.
    pub fn for_target(target: ImageTarget) -> Self {
        Self {
            style: ImageStyle::default(),
            mood: ImageMood::default(),
            palette: ImagePalette::default(),
            framing: target.framings()[0],
            avoid: vec![ImageAvoid::Text, ImageAvoid::Watermark],
        }
    }

    pub fn with_style(self, style: ImageStyle) -> Self {
        Self { style, ..self }
    }

    pub fn with_mood(self, mood: ImageMood) -> Self {
        Self { mood, ..self }
    }

    pub fn with_palette(self, palette: ImagePalette) -> Self {
        Self { palette, ..self }
    }

    pub fn with_framing(self, framing: ImageFraming) -> Self {
        Self { framing, ..self }
    }

    /// Adds `hint` when absent, removes it when present (kept in catalogue order).
    pub fn toggle_avoid(self, hint: ImageAvoid) -> Self {
        let on = !self.avoid.contains(&hint);
        let avoid = ImageAvoid::ALL.iter().copied().filter(|item| if *item == hint { on } else { self.avoid.contains(item) }).collect();
        Self { avoid, ..self }
    }

    /// The same picks moved to `target` (a framing of the other target falls back to its default).
    pub fn retargeted(self, target: ImageTarget) -> Self {
        let framing = if target.allows(self.framing) { self.framing } else { target.framings()[0] };
        Self { framing, ..self }
    }
}
