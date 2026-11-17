//! The tool loop: send, run requested tools, thread their results back, repeat until the
//! model answers (or the step budget runs out and it must answer).

use super::transport::Transport;
use crate::agent::llm::wire::STRUCTURED_OUTPUT_NAME;
use crate::agent::llm::{ChatMessage, ChatRequest, ToolChoice};
use crate::agent::tools::{ToolBox, ToolOutput};
use crate::domain::post::{SourceRef, ToolUse};
use serde_json::Value;

/// Characters of a tool call's arguments kept in its [`ToolUse`] summary.
const SUMMARY_CHARS: usize = 120;
const NO_TOOLS_ERROR: &str = "No tools are available.";

/// The model's final answer.
#[derive(Debug, Clone, PartialEq)]
pub enum Answer {
    Text(String),
    /// Arguments of the `emit_post` tool (structured output over tool calling).
    Structured(Value),
}

/// The answer, the conversation that produced it and what the tools did.
#[derive(Debug, Clone)]
pub struct LoopOutcome {
    pub answer: Answer,
    pub request: ChatRequest,
    pub tools: Vec<ToolUse>,
    pub sources: Vec<SourceRef>,
}

/// Runs the loop for at most `max_steps` tool rounds. On the last round tools are disabled,
/// or the emit tool is forced when `emit_tool` is set.
pub async fn converse(
    transport: &Transport,
    mut request: ChatRequest,
    toolbox: Option<&ToolBox>,
    emit_tool: bool,
    max_steps: u8,
) -> Result<LoopOutcome, String> {
    let mut tools = Vec::new();
    let mut sources: Vec<SourceRef> = Vec::new();
    for step in 0..=max_steps {
        if step == max_steps && !request.tools.is_empty() {
            request.tool_choice = if emit_tool { ToolChoice::Force(STRUCTURED_OUTPUT_NAME.to_string()) } else { ToolChoice::Disabled };
        }
        let reply = transport.send(&request).await?;
        if let Some(emit) = reply.tool_calls.iter().find(|call| call.name == STRUCTURED_OUTPUT_NAME) {
            let answer = Answer::Structured(emit.arguments.clone());
            return Ok(LoopOutcome { answer, request, tools, sources });
        }
        if reply.tool_calls.is_empty() || step == max_steps {
            return Ok(LoopOutcome { answer: Answer::Text(reply.text), request, tools, sources });
        }
        request.messages.push(ChatMessage::assistant(reply.text.clone(), reply.tool_calls.clone()));
        for call in &reply.tool_calls {
            let output = match toolbox {
                Some(toolbox) => toolbox.execute(call).await,
                None => ToolOutput::error(NO_TOOLS_ERROR),
            };
            tools.push(ToolUse { name: call.name.clone(), summary: summarize(&call.arguments), failed: output.failed });
            for source in output.sources {
                if !sources.contains(&source) {
                    sources.push(source);
                }
            }
            request.messages.push(ChatMessage::tool_result(call, output.content));
        }
    }
    Err("The tool loop ended without an answer".to_string())
}

/// Compact JSON of the arguments, shortened.
fn summarize(arguments: &Value) -> String {
    let text = arguments.to_string();
    if text.chars().count() <= SUMMARY_CHARS {
        return text;
    }
    let mut short: String = text.chars().take(SUMMARY_CHARS).collect();
    short.push('…');
    short
}
