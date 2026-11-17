//! [`TraitOption`] for the engine's own enums (and the text providers), so the Engine step can
//! draw them as illustrated cards like any trait.

use super::image::{ImageEngineKind, SdFlavor};
use super::presets::SchemaPreset;
use super::request::HttpMethod;
use super::tools::{SearchBackend, ToolKind};
use crate::agent::llm::LlmProvider;
use crate::domain::trait_option::TraitOption;

/// Implements [`TraitOption`] for an existing enum: `Variant => ("key", "Label")`.
macro_rules! picker_option {
    ($ty:ident in $group:literal { $($variant:ident => ($key:literal, $label:literal)),+ $(,)? }) => {
        impl TraitOption for $ty {
            const ALL: &'static [Self] = &[$($ty::$variant),+];
            const GROUP: &'static str = $group;

            fn key(self) -> &'static str {
                match self { $(Self::$variant => $key),+ }
            }

            fn label(self) -> &'static str {
                match self { $(Self::$variant => $label),+ }
            }
        }
    };
}

picker_option!(LlmProvider in "engine_provider" {
    Gemini => ("gemini", "Gemini"),
    OpenAi => ("open_ai", "OpenAI"),
    Anthropic => ("anthropic", "Anthropic"),
    OpenRouter => ("open_router", "OpenRouter"),
    Groq => ("groq", "Groq"),
    Custom => ("custom", "Custom endpoint"),
});

picker_option!(HttpMethod in "engine_method" {
    Get => ("get", "GET"),
    Post => ("post", "POST"),
    Put => ("put", "PUT"),
    Patch => ("patch", "PATCH"),
    Delete => ("delete", "DELETE"),
});

picker_option!(SchemaPreset in "engine_schema" {
    Post => ("post", "Post"),
    Illustrated => ("illustrated", "Illustrated"),
    Thread => ("thread", "Thread"),
    Expressive => ("expressive", "Expressive"),
    Custom => ("custom", "Custom"),
});

picker_option!(ImageEngineKind in "engine_image" {
    None => ("none", "No images"),
    OpenAi => ("open_ai", "OpenAI images"),
    Google => ("google", "Google images"),
    LocalSd => ("local_sd", "Local Stable Diffusion"),
    Custom => ("custom", "Custom engine"),
});

picker_option!(SdFlavor in "engine_sd" {
    ComfyUi => ("comfy_ui", "ComfyUI"),
    A1111 => ("a1111", "AUTOMATIC1111"),
});

picker_option!(ToolKind in "engine_tool" {
    VaultSearch => ("vault_search", "Vault search"),
    ReadNetwork => ("read_network", "Read the network"),
    WebSearch => ("web_search", "Web search"),
    WebFetch => ("web_fetch", "Web fetch"),
    CustomHttp => ("custom_http", "Custom HTTP tool"),
});

picker_option!(SearchBackend in "engine_search" {
    ProviderNative => ("provider_native", "Provider search"),
    Brave => ("brave", "Brave Search"),
    Tavily => ("tavily", "Tavily"),
});
