//! Picker options of the Engine step. Every choice the step shows as an illustrated card is a
//! [`TraitOption`] (`traits.engine_<group>.<key>` labels, `illustrations/engine_<group>/<key>.svg`
//! art): the variant keys of the engine enums, plus a few picker-only kinds below.

use super::image::{ImageEngineConfig, ImageTrigger};
use super::output::OutputMode;
use super::request::RequestMode;
use super::text::TextModel;
use crate::domain::trait_option::trait_enum;

/// Default chance of a chance-triggered image, in percent.
pub const DEFAULT_IMAGE_CHANCE: u8 = 30;

trait_enum! {
    /// Where the text model comes from (the variant of [`TextModel`]).
    pub enum TextSource in "engine_source" {
        #[default]
        UseGlobal("use_global", "Global model"),
        Own("own", "Own model"),
    }
}

trait_enum! {
    /// How requests are sent (the variant of [`RequestMode`]).
    pub enum RequestKind in "engine_request" {
        #[default]
        Native("native", "Native API"),
        Custom("custom", "Custom request"),
    }
}

trait_enum! {
    /// The answer's shape (the variant of [`OutputMode`]).
    pub enum OutputKind in "engine_output" {
        #[default]
        FreeText("free_text", "Free text"),
        JsonSchema("json_schema", "Structured JSON"),
    }
}

trait_enum! {
    /// When an image is drawn (the variant of [`ImageTrigger`]).
    pub enum TriggerKind in "engine_trigger" {
        Never("never", "Never"),
        Chance("chance", "By chance"),
        Always("always", "Every post"),
        #[default]
        WhenModelAsks("when_model_asks", "When the model asks"),
    }
}

trait_enum! {
    /// Common image sizes (`ImageEngineConfig::size`).
    pub enum ImageSize in "engine_size" {
        #[default]
        Square("square", "Square"),
        Portrait("portrait", "Portrait"),
        Landscape("landscape", "Landscape"),
        Small("small", "Small square"),
    }
}

impl TextSource {
    pub fn of(text: &TextModel) -> Self {
        match text {
            TextModel::UseGlobal => Self::UseGlobal,
            TextModel::Own(_) => Self::Own,
        }
    }
}

impl RequestKind {
    pub fn of(request: &RequestMode) -> Self {
        match request {
            RequestMode::Native => Self::Native,
            RequestMode::Custom(_) => Self::Custom,
        }
    }
}

impl OutputKind {
    pub fn of(output: &OutputMode) -> Self {
        match output {
            OutputMode::FreeText => Self::FreeText,
            OutputMode::JsonSchema(_) => Self::JsonSchema,
        }
    }
}

impl TriggerKind {
    pub fn of(trigger: ImageTrigger) -> Self {
        match trigger {
            ImageTrigger::Never => Self::Never,
            ImageTrigger::Chance { .. } => Self::Chance,
            ImageTrigger::Always => Self::Always,
            ImageTrigger::WhenModelAsks => Self::WhenModelAsks,
        }
    }

    /// The trigger of this kind (`percent` is kept for [`TriggerKind::Chance`]).
    pub fn trigger(self, percent: u8) -> ImageTrigger {
        match self {
            Self::Never => ImageTrigger::Never,
            Self::Chance => ImageTrigger::Chance { percent },
            Self::Always => ImageTrigger::Always,
            Self::WhenModelAsks => ImageTrigger::WhenModelAsks,
        }
    }
}

impl ImageSize {
    /// The `WIDTHxHEIGHT` string the engines receive.
    pub const fn dimensions(self) -> &'static str {
        match self {
            Self::Square => "1024x1024",
            Self::Portrait => "1024x1536",
            Self::Landscape => "1536x1024",
            Self::Small => "512x512",
        }
    }

    /// The preset matching `config.size`, if it is one.
    pub fn of(config: &ImageEngineConfig) -> Option<Self> {
        let size = config.size.trim().to_ascii_lowercase();
        Self::ALL.into_iter().find(|preset| preset.dimensions() == size)
    }
}
