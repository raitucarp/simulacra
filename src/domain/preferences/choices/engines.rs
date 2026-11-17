//! Model and diagnostics choices: the embedding provider, the default image engine and size,
//! and the log level.

use super::super::choice::choice_enum;

choice_enum! {
    /// Where text embeddings are computed.
    EmbeddingProvider, default: Gemini,
    [Gemini => "gemini", LocalServer => "local_server", OpenAi => "open_ai", Custom => "custom"]
}

choice_enum! {
    /// The image engine new agents start with (keys match `ImageEngineKind`).
    ImageEngineChoice, default: None,
    [None => "none", Google => "google", OpenAi => "open_ai", LocalSd => "local_sd"]
}

choice_enum! {
    /// The default image shape (keys match the engine `ImageSize`).
    ImageAspect, default: Square,
    [Square => "square", Portrait => "portrait", Landscape => "landscape", Small => "small"]
}

choice_enum! {
    /// How much the app writes to its log.
    LogLevel, default: Warn,
    [Error => "error", Warn => "warn", Info => "info", Debug => "debug"]
}
