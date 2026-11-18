//! Every setting the Engine step edits, keyed by the caption path its card shows
//! (`engine_field("engine_model", …)`), so each one can carry suggestions.

use crate::domain::document::keyed::keyed_enum;
use crate::state::forge::EngineSection;

keyed_enum! {
    /// One Engine-step setting (an `EngineConfig` part, or a part of one custom tool).
    EngineParam {
        Source => "engine_source",
        Provider => "engine_provider",
        Model => "engine_model",
        Temperature => "engine_temperature",
        MaxTokens => "engine_max_tokens",
        Endpoint => "engine_endpoint",
        Credential => "engine_credential",
        Request => "engine_request",
        Method => "engine_method",
        Url => "engine_url",
        Headers => "engine_headers",
        Body => "engine_body",
        ReplyPath => "engine_reply_path",
        ToolCallPath => "engine_tool_call_path",
        Output => "engine_output",
        SchemaPreset => "engine_schema_preset",
        Schema => "engine_schema",
        Mapping => "engine_mapping",
        Image => "engine_image",
        ImageModel => "engine_image_model",
        ImageSize => "engine_image_size",
        ImageReplyPath => "engine_image_reply_path",
        SdFlavor => "engine_sd_flavor",
        SdUrl => "engine_sd_url",
        StylePrefix => "engine_style_prefix",
        Trigger => "engine_trigger",
        ImageChance => "engine_image_chance",
        Tools => "engine_tools",
        SearchBackend => "engine_search_backend",
        MaxToolSteps => "engine_max_tool_steps",
        ToolName => "engine_tool_name",
        ToolDescription => "engine_tool_description",
        UrlTemplate => "engine_url_template",
        ParamsSchema => "engine_params_schema",
    }
}

/// Captions that group several settings (each of which has its own suggestions).
pub const GROUP_CAPTIONS: [&str; 1] = ["engine_sampling"];

impl EngineParam {
    /// The Engine-step section the setting sits in.
    pub const fn section(self) -> EngineSection {
        use EngineParam::*;
        match self {
            Source | Provider | Model | Temperature | MaxTokens | Endpoint | Credential => EngineSection::Model,
            Request | Method | Url | Headers | Body | ReplyPath | ToolCallPath => EngineSection::Request,
            Output | SchemaPreset | Schema | Mapping => EngineSection::Output,
            Image | ImageModel | ImageSize | ImageReplyPath | SdFlavor | SdUrl | StylePrefix | Trigger | ImageChance => {
                EngineSection::Image
            }
            Tools | SearchBackend | MaxToolSteps | ToolName | ToolDescription | UrlTemplate | ParamsSchema => {
                EngineSection::Tools
            }
        }
    }

    /// Whether the AI may propose values: never for keys, servers or reply shapes only the
    /// user knows, nor for the parts of one custom tool.
    pub const fn ai_capable(self) -> bool {
        use EngineParam::*;
        matches!(self, Temperature | MaxTokens | Output | SchemaPreset | ImageSize | StylePrefix | Trigger | ImageChance | Tools | MaxToolSteps)
    }
}
