//! Starting picks of the Engine's choices: the global model and native requests (nothing to
//! set up), structured answers once the model should ask for images, few tools.

use super::super::engine_param::EngineParam;
use crate::domain::engine::{EngineConfig, ImageTrigger, OutputMode};

/// Whether images are on and wait for the model to ask (which needs a structured answer).
fn model_asks_for_images(engine: &EngineConfig) -> bool {
    engine.image.is_enabled() && engine.image.trigger == ImageTrigger::WhenModelAsks
}

/// One to three starting options of an Engine choice (`None` for settings that are not choices).
pub(super) fn engine_picks(param: EngineParam, engine: &EngineConfig) -> Option<Vec<&'static str>> {
    use EngineParam as E;
    let structured = matches!(engine.output, OutputMode::JsonSchema(_));
    let picks: &[&'static str] = match param {
        E::Source => &["use_global"],
        E::Provider => &["gemini", "open_ai", "anthropic"],
        E::Request => &["native"],
        E::Method => &["post", "get"],
        E::Output if model_asks_for_images(engine) => &["json_schema"],
        E::Output => &["free_text", "json_schema"],
        E::SchemaPreset if engine.image.is_enabled() => &["illustrated", "post"],
        E::SchemaPreset => &["post", "thread"],
        E::Image => &["none", "google", "open_ai"],
        E::ImageSize => &["square", "landscape"],
        E::SdFlavor => &["comfy_ui"],
        E::Trigger if structured => &["when_model_asks", "chance"],
        E::Trigger => &["chance", "never"],
        E::Tools => &["vault_search", "read_network"],
        E::SearchBackend => &["provider_native"],
        _ => return None,
    };
    Some(picks.to_vec())
}
