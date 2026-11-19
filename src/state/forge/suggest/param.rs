//! [`Param`]: every agent and persona parameter the forge edits, one typed id over the field
//! registries (`fields::*::ALL`), the Engine settings and the photo zoom.

use super::engine_param::EngineParam;
use crate::state::forge::fields::{
    AvatarField, CountField, HourField, OptionalTextField, ScoreField, TextField, ToggleField, TraitField, TraitSet,
};
use crate::state::forge::ForgeStep;
use serde::{Deserialize, Serialize};

/// Caption path of the photo zoom slider.
const PHOTO_ZOOM_PATH: &str = "photo_zoom";

/// One parameter of the forge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Param {
    Text(TextField),
    OptionalText(OptionalTextField),
    Avatar(AvatarField),
    Score(ScoreField),
    Hour(HourField),
    Count(CountField),
    Toggle(ToggleField),
    Trait(TraitField),
    Set(TraitSet),
    Engine(EngineParam),
    /// How far an imported photo is zoomed in its circle.
    PhotoZoom,
}

impl Param {
    /// Every parameter, registry by registry, in declaration order.
    pub fn all() -> Vec<Self> {
        let mut all: Vec<Self> = TextField::ALL.iter().copied().map(Self::Text).collect();
        all.extend(OptionalTextField::ALL.iter().copied().map(Self::OptionalText));
        all.extend(AvatarField::ALL.iter().copied().map(Self::Avatar));
        all.extend(ScoreField::ALL.iter().copied().map(Self::Score));
        all.extend(HourField::ALL.iter().copied().map(Self::Hour));
        all.extend(CountField::ALL.iter().copied().map(Self::Count));
        all.extend(ToggleField::ALL.iter().copied().map(Self::Toggle));
        all.extend(TraitField::ALL.iter().copied().map(Self::Trait));
        all.extend(TraitSet::ALL.iter().copied().map(Self::Set));
        all.extend(EngineParam::ALL.iter().copied().map(Self::Engine));
        all.push(Self::PhotoZoom);
        all
    }

    /// Serde path of the field (the caption path for Engine settings and the zoom).
    pub fn path(self) -> &'static str {
        match self {
            Self::Text(field) => field.path(),
            Self::OptionalText(field) => field.path(),
            Self::Avatar(field) => field.path(),
            Self::Score(field) => field.path(),
            Self::Hour(field) => field.path(),
            Self::Count(field) => field.path(),
            Self::Toggle(field) => field.path(),
            Self::Trait(field) => field.path(),
            Self::Set(field) => field.path(),
            Self::Engine(param) => param.key(),
            Self::PhotoZoom => PHOTO_ZOOM_PATH,
        }
    }

    /// The step that edits the parameter.
    pub fn step(self) -> ForgeStep {
        match self {
            Self::Text(field) => field.step(),
            Self::OptionalText(field) => field.step(),
            Self::Avatar(field) => field.step(),
            Self::Score(field) => field.step(),
            Self::Hour(field) => field.step(),
            Self::Count(field) => field.step(),
            Self::Toggle(field) => field.step(),
            Self::Trait(field) => field.step(),
            Self::Set(field) => field.step(),
            Self::Engine(_) => ForgeStep::Engine,
            Self::PhotoZoom => ForgeStep::Appearance,
        }
    }

    /// The parameter with path `path`, if any.
    pub fn from_path(path: &str) -> Option<Self> {
        Self::all().into_iter().find(|param| param.path() == path)
    }

    /// Every parameter `step` edits.
    pub fn for_step(step: ForgeStep) -> Vec<Self> {
        Self::all().into_iter().filter(|param| param.step() == step).collect()
    }

    /// Whether "Suggest from context" may ask the AI for this parameter.
    pub fn ai_capable(self) -> bool {
        match self {
            Self::Engine(param) => param.ai_capable(),
            Self::OptionalText(_) | Self::Avatar(_) | Self::PhotoZoom => false,
            _ => true,
        }
    }
}
