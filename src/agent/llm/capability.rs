//! What each provider's native wire format supports.

use super::LlmProvider;

/// How structured (JSON-schema) output is requested from a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaStrategy {
    /// Native schema field (OpenAI `response_format`, Gemini `responseSchema`).
    ResponseFormat,
    /// A forced `emit_post` tool whose arguments are the answer (Anthropic, and Gemini
    /// when other tools are in play).
    EmitTool,
    /// Ask for JSON in the prompt, parse it, and repair once if needed.
    Instruct,
}

impl LlmProvider {
    /// Whether the provider has its own web search (Gemini grounding, OpenAI search
    /// models, Anthropic server tool, OpenRouter web plugin).
    pub fn native_web_search(self) -> bool {
        matches!(self, Self::Gemini | Self::OpenAi | Self::Anthropic | Self::OpenRouter)
    }

    /// How to request structured output, given whether function tools are also offered.
    pub fn schema_strategy(self, with_tools: bool) -> SchemaStrategy {
        match self {
            Self::OpenAi | Self::OpenRouter => SchemaStrategy::ResponseFormat,
            Self::Gemini if with_tools => SchemaStrategy::EmitTool,
            Self::Gemini => SchemaStrategy::ResponseFormat,
            Self::Anthropic => SchemaStrategy::EmitTool,
            Self::Groq | Self::Custom => SchemaStrategy::Instruct,
        }
    }
}
