//! What each Engine-step card does to an [`EngineConfig`]: switching a variant keeps what
//! still applies (sampling settings, a chance percent, granted tools) and fills sensible
//! defaults for the rest.

use crate::agent::llm::LlmProvider;
use crate::domain::engine::{
    CustomRequest, EngineConfig, ImageEngine, ImageEngineKind, ImageTrigger, OutputKind, OutputMode, OwnModel, RequestKind,
    RequestMode, SchemaPreset, SdFlavor, StructuredOutput, TextModel, TextSource, TriggerKind, DEFAULT_IMAGE_CHANCE,
};

/// Default ComfyUI server.
pub const COMFY_DEFAULT_URL: &str = "http://127.0.0.1:8188";
/// Default AUTOMATIC1111 server.
pub const A1111_DEFAULT_URL: &str = "http://127.0.0.1:7860";
/// Body of a new custom image request (OpenAI-images compatible).
const IMAGE_BODY_TEMPLATE: &str = r#"{"model": {{model}}, "prompt": {{prompt}}, "size": {{size}}}"#;
/// Where a new custom image request reads the picture.
const IMAGE_REPLY_PATH: &str = "data.0.b64_json";

pub use super::tool_grants::{add_custom_tool, custom_tool_mut, granted, remove_custom_tool, set_tools, web_search_mut};

/// The default server URL of a local Stable Diffusion flavour.
pub const fn sd_default_url(flavor: SdFlavor) -> &'static str {
    match flavor {
        SdFlavor::ComfyUi => COMFY_DEFAULT_URL,
        SdFlavor::A1111 => A1111_DEFAULT_URL,
    }
}

/// Global model, or an own model on the default provider.
pub fn set_text_source(engine: &mut EngineConfig, source: TextSource) {
    engine.text = match (source, &engine.text) {
        (TextSource::Own, TextModel::Own(own)) => TextModel::Own(own.clone()),
        (TextSource::Own, TextModel::UseGlobal) => TextModel::Own(OwnModel::for_provider(LlmProvider::default())),
        (TextSource::UseGlobal, _) => TextModel::UseGlobal,
    };
}

/// Moves the own model to `provider` (its default model and key), keeping the sampling.
pub fn set_provider(engine: &mut EngineConfig, provider: LlmProvider) {
    if let TextModel::Own(own) = &mut engine.text {
        if own.provider != provider {
            *own = OwnModel { temperature: own.temperature, max_tokens: own.max_tokens, ..OwnModel::for_provider(provider) };
        }
    }
}

/// Native requests, or a custom request (starting from the OpenAI-compatible template).
pub fn set_request_kind(engine: &mut EngineConfig, kind: RequestKind) {
    if RequestKind::of(&engine.request) != kind {
        engine.request = match kind {
            RequestKind::Native => RequestMode::Native,
            RequestKind::Custom => RequestMode::Custom(CustomRequest::default()),
        };
    }
}

/// Free text, or the default (post) schema.
pub fn set_output_kind(engine: &mut EngineConfig, kind: OutputKind) {
    if OutputKind::of(&engine.output) != kind {
        engine.output = match kind {
            OutputKind::FreeText => OutputMode::FreeText,
            OutputKind::JsonSchema => OutputMode::JsonSchema(StructuredOutput::default()),
        };
    }
}

/// Loads a preset's schema and mapping; `Custom` keeps the current schema to edit by hand.
/// Whether the agent may tint or react is a setting, not part of the schema, so it stays.
pub fn apply_preset(engine: &mut EngineConfig, preset: SchemaPreset) {
    let OutputMode::JsonSchema(structured) = &mut engine.output else { return };
    if preset == SchemaPreset::Custom {
        structured.preset = SchemaPreset::Custom;
    } else {
        *structured = StructuredOutput { may_tint: structured.may_tint, may_react: structured.may_react, ..StructuredOutput::from_preset(preset) };
    }
}

/// Lets the agent tint its posts, or not (sb C4).
pub fn set_may_tint(engine: &mut EngineConfig, may_tint: bool) {
    if let OutputMode::JsonSchema(structured) = &mut engine.output {
        structured.may_tint = may_tint;
    }
}

/// Lets the agent react to the posts it answers, or not (R8).
pub fn set_may_react(engine: &mut EngineConfig, may_react: bool) {
    if let OutputMode::JsonSchema(structured) = &mut engine.output {
        structured.may_react = may_react;
    }
}

/// Replaces the schema text; a hand edit makes it `Custom` unless it matches the preset.
pub fn edit_schema(engine: &mut EngineConfig, schema: String) {
    let OutputMode::JsonSchema(structured) = &mut engine.output else { return };
    if schema != structured.preset.schema() {
        structured.preset = SchemaPreset::Custom;
    }
    structured.schema = schema;
}

/// Switches the image engine, filling a local server URL or a custom request template.
pub fn set_image_kind(engine: &mut EngineConfig, kind: ImageEngineKind) {
    if engine.image.engine.kind() == kind {
        return;
    }
    engine.image.engine = match kind {
        ImageEngineKind::None => ImageEngine::None,
        ImageEngineKind::OpenAi => ImageEngine::OpenAi,
        ImageEngineKind::Google => ImageEngine::Google,
        ImageEngineKind::LocalSd => ImageEngine::LocalSd { flavor: SdFlavor::default(), url: COMFY_DEFAULT_URL.to_string() },
        ImageEngineKind::Custom => ImageEngine::Custom(CustomRequest {
            body_template: IMAGE_BODY_TEMPLATE.to_string(),
            reply_path: IMAGE_REPLY_PATH.to_string(),
            tool_call_path: String::new(),
            ..CustomRequest::default()
        }),
    };
    engine.image.credential = None;
}

/// Picks a local server flavour; an untouched default URL follows it.
pub fn set_sd_flavor(engine: &mut EngineConfig, flavor: SdFlavor) {
    if let ImageEngine::LocalSd { flavor: current, url } = &mut engine.image.engine {
        let untouched = url.trim().is_empty() || SdFlavor::ALL.iter().any(|f| url.trim() == sd_default_url(*f));
        if untouched {
            *url = sd_default_url(flavor).to_string();
        }
        *current = flavor;
    }
}

/// Switches when images are drawn, keeping the chance percent.
pub fn set_trigger(engine: &mut EngineConfig, kind: TriggerKind) {
    let percent = match engine.image.trigger {
        ImageTrigger::Chance { percent } => percent,
        _ => DEFAULT_IMAGE_CHANCE,
    };
    engine.image.trigger = kind.trigger(percent);
}
