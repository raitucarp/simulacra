//! Provider-neutral conversation types: messages, tool specs, tool calls and replies.

use crate::domain::post::SourceRef;
use serde_json::Value;

/// Who speaks a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
    /// The result of one tool call.
    Tool,
}

/// One conversation turn.
#[derive(Debug, Clone, PartialEq)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    /// Tool calls requested by an assistant turn.
    pub tool_calls: Vec<ToolCall>,
    /// The call a tool turn answers (id and tool name).
    pub tool_call_id: Option<String>,
    pub tool_name: Option<String>,
}

impl ChatMessage {
    fn plain(role: Role, content: impl Into<String>) -> Self {
        Self { role, content: content.into(), tool_calls: Vec::new(), tool_call_id: None, tool_name: None }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self::plain(Role::System, content)
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::plain(Role::User, content)
    }

    /// An assistant turn, possibly requesting tool calls.
    pub fn assistant(content: impl Into<String>, tool_calls: Vec<ToolCall>) -> Self {
        Self { tool_calls, ..Self::plain(Role::Assistant, content) }
    }

    /// The result of `call`.
    pub fn tool_result(call: &ToolCall, content: impl Into<String>) -> Self {
        Self {
            tool_call_id: Some(call.id.clone()),
            tool_name: Some(call.name.clone()),
            ..Self::plain(Role::Tool, content)
        }
    }
}

/// A tool call requested by the model; `arguments` is the parsed JSON object.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
    /// Opaque thought signature sent back with the call (Gemini 3+ rejects a tool loop without it).
    pub signature: Option<String>,
}

/// A function tool offered to the model; `parameters` is a JSON schema.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

/// Whether and which tool the model must call.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ToolChoice {
    /// The model decides.
    #[default]
    Auto,
    /// The model must call some tool.
    Required,
    /// The model must call this tool.
    Force(String),
    /// Tools stay declared (earlier calls remain valid) but the model must answer in text.
    Disabled,
}

/// One request to a text model.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub tools: Vec<ToolSpec>,
    pub tool_choice: ToolChoice,
    /// Native structured output (OpenAI `response_format`, Gemini `responseSchema`).
    pub schema: Option<Value>,
    /// Enable the provider's own web search.
    pub web_search: bool,
}

impl ChatRequest {
    /// A system + user request with no tools.
    pub fn simple(system: &str, user: &str) -> Self {
        Self { messages: vec![ChatMessage::system(system), ChatMessage::user(user)], ..Self::default() }
    }

    /// Concatenated system turns (providers with a separate system field).
    pub fn system_text(&self) -> String {
        let parts: Vec<&str> =
            self.messages.iter().filter(|m| m.role == Role::System).map(|m| m.content.as_str()).collect();
        parts.join("\n\n")
    }
}

/// A model reply: its text and any requested tool calls.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ChatReply {
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
    /// Web pages the provider's own search grounded the reply on.
    pub sources: Vec<SourceRef>,
}

impl ChatReply {
    /// Whether the reply carries neither text nor tool calls.
    pub fn is_empty(&self) -> bool {
        self.text.trim().is_empty() && self.tool_calls.is_empty()
    }
}

/// Parses tool-call arguments sent as a JSON string; non-JSON text is kept as a string.
pub fn parse_arguments(raw: &Value) -> Value {
    match raw {
        Value::String(text) => serde_json::from_str(text).unwrap_or_else(|_| Value::String(text.clone())),
        Value::Null => Value::Object(Default::default()),
        other => other.clone(),
    }
}
