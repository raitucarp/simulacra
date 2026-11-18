//! How one image engine wants its prompt: the house style put first, the size, and whether
//! it takes a separate negative prompt (local Stable Diffusion) or needs the hints said
//! positively (hosted engines and custom requests).

use crate::domain::engine::{ImageEngine, ImageEngineConfig, ImageEngineKind, SdFlavor};
use crate::domain::preferences::{Choice, ImageEngineChoice, ImagePrefs};

/// Sizes of the image shapes (`WIDTHxHEIGHT`, the engine `ImageSize` keys).
const SIZES: [(&str, &str); 4] = [("square", "1024x1024"), ("portrait", "1024x1536"), ("landscape", "1536x1024"), ("small", "512x512")];

/// The size of an aspect key, or `None` for an unknown key.
pub fn size_of_aspect(aspect: &str) -> Option<&'static str> {
    SIZES.iter().find(|(key, _)| *key == aspect).map(|(_, size)| *size)
}

/// The prompt-relevant traits of one engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptEngine {
    pub kind: ImageEngineKind,
    /// Put before every prompt (the agent's house style, else the preference).
    pub style_prefix: String,
    /// `WIDTHxHEIGHT`.
    pub size: String,
}

impl PromptEngine {
    /// The engine of `config`; a blank style prefix falls back to `default_prefix`.
    pub fn of(config: &ImageEngineConfig, default_prefix: &str) -> Self {
        let own = config.style_prefix.trim();
        let style_prefix = if own.is_empty() { default_prefix.trim() } else { own };
        Self { kind: config.engine.kind(), style_prefix: style_prefix.to_string(), size: config.size.trim().to_string() }
    }

    /// Whether the engine takes a separate negative prompt.
    pub const fn supports_negative(&self) -> bool {
        matches!(self.kind, ImageEngineKind::LocalSd)
    }

    /// The same engine at the size of `aspect` (unknown aspects keep the size).
    pub fn with_aspect(self, aspect: &str) -> Self {
        let size = size_of_aspect(aspect).map_or(self.size.clone(), str::to_string);
        Self { size, ..self }
    }
}

/// Where a local Stable Diffusion server listens by default (ComfyUI).
pub const LOCAL_SD_DEFAULT_URL: &str = "http://127.0.0.1:8188";

/// The app-wide image engine of Settings › Images, as an engine config.
pub fn global_engine_config(prefs: &ImagePrefs) -> ImageEngineConfig {
    let engine = match prefs.default_engine {
        ImageEngineChoice::None => ImageEngine::None,
        ImageEngineChoice::Google => ImageEngine::Google,
        ImageEngineChoice::OpenAi => ImageEngine::OpenAi,
        ImageEngineChoice::LocalSd => ImageEngine::LocalSd { flavor: SdFlavor::default(), url: LOCAL_SD_DEFAULT_URL.to_string() },
    };
    let size = size_of_aspect(prefs.size.key()).unwrap_or_default().to_string();
    ImageEngineConfig { engine, size, style_prefix: prefs.style_prefix.clone(), ..ImageEngineConfig::default() }
}

/// The engine of an agent when it has one, else the app-wide one.
pub fn engine_for(own: &ImageEngineConfig, prefs: &ImagePrefs) -> ImageEngineConfig {
    if is_configured(&own.engine) { own.clone() } else { global_engine_config(prefs) }
}

/// Whether `engine` can render at all (it may still lack a key).
pub fn is_configured(engine: &ImageEngine) -> bool {
    !matches!(engine, ImageEngine::None)
}
