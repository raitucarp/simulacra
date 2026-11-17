//! Per-provider request bodies and reply parsing over the neutral [`super::chat`] types.
//! Every builder is pure, so request shapes are golden-tested without a network.

pub(crate) mod anthropic;
pub(crate) mod gemini;
pub(crate) mod openai;
mod schema;

/// Name of the structured-output schema (OpenAI `json_schema.name`) and of the forced
/// emit tool used where structured output rides on tool calling.
pub const STRUCTURED_OUTPUT_NAME: &str = "emit_post";

/// Provider-specific error message prefixes.
pub(crate) struct ErrorLabels {
    /// Prefix for transport failures: `"{network}: {error}"`.
    pub network: &'static str,
    /// Prefix for non-success HTTP statuses: `"{api} ({status}): {body}"`.
    pub api: &'static str,
    /// Prefix for JSON decoding failures: `"{parse}: {error}"`.
    pub parse: &'static str,
    /// The reply had neither text nor tool calls.
    pub empty: &'static str,
}
