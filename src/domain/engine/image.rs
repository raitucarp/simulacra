//! The image engine that may illustrate an agent's posts.

use super::request::CustomRequest;
use serde::{Deserialize, Serialize};

/// Placeholders of a custom image request body (JSON values, like the text placeholders).
pub const IMAGE_PLACEHOLDERS: [&str; 3] = ["{{prompt}}", "{{size}}", "{{model}}"];
const DEFAULT_IMAGE_SIZE: &str = "1024x1024";

/// Which backend renders images.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "engine", rename_all = "snake_case")]
pub enum ImageEngine {
    /// No images (the classic behaviour).
    #[default]
    None,
    /// OpenAI Images API (`gpt-image-1`, `dall-e-3`).
    OpenAi,
    /// Google Imagen (`imagen-*`, `:predict`) or a Gemini image model (`:generateContent`).
    Google,
    /// A local Stable Diffusion server.
    LocalSd { flavor: SdFlavor, url: String },
    /// A raw request whose reply holds base64 image data or an image URL at `reply_path`.
    Custom(CustomRequest),
}

/// Local Stable Diffusion server flavours.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SdFlavor {
    #[default]
    ComfyUi,
    A1111,
}

impl SdFlavor {
    pub const ALL: [SdFlavor; 2] = [Self::ComfyUi, Self::A1111];

    /// Stable key (`comfy_ui` / `a1111`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::ComfyUi => "comfy_ui",
            Self::A1111 => "a1111",
        }
    }
}

/// Variant keys of [`ImageEngine`], for pickers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageEngineKind {
    None,
    OpenAi,
    Google,
    LocalSd,
    Custom,
}

impl ImageEngineKind {
    pub const ALL: [ImageEngineKind; 5] = [Self::None, Self::OpenAi, Self::Google, Self::LocalSd, Self::Custom];

    /// Stable key (`none`, `open_ai`, `google`, `local_sd`, `custom`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OpenAi => "open_ai",
            Self::Google => "google",
            Self::LocalSd => "local_sd",
            Self::Custom => "custom",
        }
    }
}

impl ImageEngine {
    /// The variant's kind (picker key).
    pub fn kind(&self) -> ImageEngineKind {
        match self {
            Self::None => ImageEngineKind::None,
            Self::OpenAi => ImageEngineKind::OpenAi,
            Self::Google => ImageEngineKind::Google,
            Self::LocalSd { .. } => ImageEngineKind::LocalSd,
            Self::Custom(_) => ImageEngineKind::Custom,
        }
    }
}

/// When an image is generated for a post.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "when", rename_all = "snake_case")]
pub enum ImageTrigger {
    Never,
    /// With a `percent` (0..=100) chance per post.
    Chance { percent: u8 },
    Always,
    /// Only when the structured answer carries a non-empty image prompt.
    #[default]
    WhenModelAsks,
}

impl ImageTrigger {
    /// Stable key (`never`, `chance`, `always`, `when_model_asks`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Never => "never",
            Self::Chance { .. } => "chance",
            Self::Always => "always",
            Self::WhenModelAsks => "when_model_asks",
        }
    }
}

/// Image engine settings. `credential` is a credential-store id; its endpoint, when set,
/// replaces the provider base URL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ImageEngineConfig {
    pub engine: ImageEngine,
    /// Provider model id (blank = engine default; the checkpoint name for ComfyUI).
    pub model: String,
    /// `WIDTHxHEIGHT`.
    pub size: String,
    /// Prepended to every image prompt (a house style).
    pub style_prefix: String,
    pub trigger: ImageTrigger,
    pub credential: Option<String>,
}

impl Default for ImageEngineConfig {
    fn default() -> Self {
        Self {
            engine: ImageEngine::None,
            model: String::new(),
            size: DEFAULT_IMAGE_SIZE.to_string(),
            style_prefix: String::new(),
            trigger: ImageTrigger::default(),
            credential: None,
        }
    }
}

impl ImageEngineConfig {
    /// Whether any image can ever be generated.
    pub fn is_enabled(&self) -> bool { self.engine != ImageEngine::None && self.trigger != ImageTrigger::Never }

    /// `(width, height)` parsed from `size`, if well-formed.
    pub fn dimensions(&self) -> Option<(u32, u32)> {
        let (w, h) = self.size.trim().split_once(['x', 'X'])?;
        Some((w.trim().parse().ok()?, h.trim().parse().ok()?))
    }
}
