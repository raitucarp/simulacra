//! What the image ideas sheet shows, and the events it sends to its host.

use crate::domain::image_prompt::{ImageAvoid, ImageChoices, ImageContext, ImageFraming, ImageMood, ImagePalette, ImageStyle, PromptEngine};
use crate::domain::engine::ImageEngineConfig;
use crate::domain::ProfileId;

/// Where a chosen picture goes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdeasDestination {
    /// The avatar of the draft open in the Agent Forge.
    Forge,
    /// The avatar of a stored agent or persona.
    Profile(ProfileId),
    /// The draft open in the composer.
    Composer,
}

/// Why something could not be done; localized at render time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdeasFailure {
    /// No image engine is chosen.
    NoEngine,
    /// The image engine has no key.
    NoImageKey,
    /// No language model key for "Write it for me".
    NoTextKey,
    /// The shared daily AI budget is used up.
    Budget,
    /// The service could not be reached or did not answer in time.
    Offline,
    /// Anything else, with a short redacted detail.
    Failed(String),
}

/// "Write it for me".
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CandidatesStatus {
    #[default]
    Idle,
    Running,
    Ready(Vec<String>),
    Failed(IdeasFailure),
}

/// "Generate".
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GenerationStatus {
    #[default]
    Idle,
    Running,
    Failed(IdeasFailure),
}

/// The open sheet.
#[derive(Debug, Clone, PartialEq)]
pub struct IdeasSheet {
    pub destination: IdeasDestination,
    /// Who or what the picture is for (a name, or the post's first words).
    pub about: String,
    pub context: ImageContext,
    pub choices: ImageChoices,
    pub engine: PromptEngine,
    /// The engine that draws (the agent's own, else Settings › Images).
    pub engine_config: ImageEngineConfig,
    /// Set when the engine cannot render (no engine, or no key).
    pub engine_problem: Option<IdeasFailure>,
    /// The negative prompt of engines that take one.
    pub negative: Option<String>,
    pub candidates: CandidatesStatus,
    pub generation: GenerationStatus,
    /// Saved pictures of this session, newest first.
    pub results: Vec<String>,
    /// Bumped by every "Surprise me".
    pub seed: u64,
}

/// A pick clicked in the sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdeasPick {
    Style(ImageStyle),
    Mood(ImageMood),
    Palette(ImagePalette),
    Framing(ImageFraming),
    Avoid(ImageAvoid),
}

impl IdeasPick {
    /// `choices` with this pick applied.
    pub fn apply(self, choices: ImageChoices) -> ImageChoices {
        match self {
            Self::Style(style) => choices.with_style(style),
            Self::Mood(mood) => choices.with_mood(mood),
            Self::Palette(palette) => choices.with_palette(palette),
            Self::Framing(framing) => choices.with_framing(framing),
            Self::Avoid(hint) => choices.toggle_avoid(hint),
        }
    }

    /// Whether this pick is part of `choices`.
    pub fn is_on(self, choices: &ImageChoices) -> bool {
        match self {
            Self::Style(style) => choices.style == style,
            Self::Mood(mood) => choices.mood == mood,
            Self::Palette(palette) => choices.palette == palette,
            Self::Framing(framing) => choices.framing == framing,
            Self::Avoid(hint) => choices.avoid.contains(&hint),
        }
    }
}

/// Everything the sheet asks its host to do.
#[derive(Debug, Clone, PartialEq)]
pub enum IdeasEvent {
    Pick(IdeasPick),
    Surprise,
    WriteForMe,
    UseCandidate(usize),
    Generate,
    Cancel,
    /// Set the result at this index as the avatar, or attach it to the draft.
    UseResult(usize),
    OpenSettings,
    Close,
}
