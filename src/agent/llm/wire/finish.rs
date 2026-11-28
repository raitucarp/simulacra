//! Why a reply came back with neither text nor tool calls: the provider's stop reason, a
//! safety block or a blocked prompt, appended to the "empty reply" error so the notice says why.

use super::super::LlmProvider;
use serde_json::Value;

/// Stop reasons meaning the output token limit was reached (Gemini, OpenAI, Anthropic).
const TOKEN_LIMIT_REASONS: [&str; 3] = ["MAX_TOKENS", "length", "max_tokens"];
/// Stop reasons of a normal end (nothing to explain).
const NORMAL_REASONS: [&str; 6] = ["STOP", "FINISH_REASON_UNSPECIFIED", "stop", "end_turn", "tool_calls", "tool_use"];
/// Gemini stop reasons of a reply held back by a filter.
const WITHHELD_REASONS: [&str; 7] = ["SAFETY", "IMAGE_SAFETY", "PROHIBITED_CONTENT", "IMAGE_PROHIBITED_CONTENT", "BLOCKLIST", "SPII", "RECITATION"];
const TOKEN_LIMIT_NOTE: &str = "the max-token limit was reached before any answer (thinking counts toward it); raise Max tokens";
/// Prefix of Gemini harm categories, dropped for display.
const HARM_PREFIX: &str = "HARM_CATEGORY_";

/// The explanation of an empty `reply`, when the provider gave one.
pub(crate) fn empty_reason(provider: LlmProvider, reply: &Value) -> Option<String> {
    match provider {
        LlmProvider::Gemini => gemini_reason(reply),
        LlmProvider::Anthropic => stop_note(reply["stop_reason"].as_str()?),
        _ => openai_reason(&reply["choices"][0]),
    }
}

/// A blocked prompt (`promptFeedback.blockReason`) or the first candidate's `finishReason`
/// (with the blocked safety categories).
pub(crate) fn gemini_reason(reply: &Value) -> Option<String> {
    if let Some(block) = reply["promptFeedback"]["blockReason"].as_str() {
        return Some(format!("the prompt was blocked ({block})"));
    }
    let candidate = &reply["candidates"][0];
    let reason = candidate["finishReason"].as_str()?;
    let categories: Vec<&str> = candidate["safetyRatings"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
        .iter()
        .filter(|rating| rating["blocked"] == Value::Bool(true))
        .filter_map(|rating| rating["category"].as_str())
        .map(|category| category.trim_start_matches(HARM_PREFIX))
        .collect();
    if categories.is_empty() {
        stop_note(reason)
    } else {
        Some(format!("the reply was withheld ({reason}: {})", categories.join(", ")))
    }
}

/// A refusal, a content filter or the stop reason of an OpenAI-style choice.
fn openai_reason(choice: &Value) -> Option<String> {
    if let Some(refusal) = choice["message"]["refusal"].as_str().map(str::trim).filter(|text| !text.is_empty()) {
        return Some(format!("the model refused: {refusal}"));
    }
    match choice["finish_reason"].as_str()? {
        "content_filter" => Some("the reply was withheld by the content filter (content_filter)".to_string()),
        other => stop_note(other),
    }
}

/// The note for a stop reason (`None` for a normal end).
fn stop_note(reason: &str) -> Option<String> {
    if NORMAL_REASONS.contains(&reason) {
        None
    } else if TOKEN_LIMIT_REASONS.contains(&reason) {
        Some(format!("{TOKEN_LIMIT_NOTE} ({reason})"))
    } else if WITHHELD_REASONS.contains(&reason) {
        Some(format!("the reply was withheld ({reason})"))
    } else {
        Some(format!("the reply ended early ({reason})"))
    }
}
