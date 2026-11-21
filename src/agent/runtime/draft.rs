//! The post an engine answer describes, and how a structured answer maps onto it.

use crate::domain::engine::{parse_tint, value_at, value_text, OutputMapping};
use crate::domain::post::Tint;
use crate::domain::post::sanitize::sanitize_agent_markdown;
use serde_json::Value;

/// Hashtag sigil.
const HASHTAG_SIGIL: char = '#';
/// Separators of hashtags given as one string.
const HASHTAG_SEPARATORS: [char; 2] = [',', ' '];

/// What the agent will post.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PostDraft {
    pub content: String,
    pub image_prompt: Option<String>,
    pub hashtags: Vec<String>,
    pub mood: Option<String>,
    /// Follow-up posts chained under the first.
    pub thread_parts: Vec<String>,
    /// The container tint the answer names (sb C4; kept only while the agent may tint).
    pub tint: Option<Tint>,
}

impl PostDraft {
    /// A free-text answer.
    pub fn from_text(text: &str) -> Self {
        Self { content: sanitize_agent_markdown(text).trim().to_string(), ..Self::default() }
    }

    /// Maps a structured answer through `mapping`. When the content is missing, the first
    /// thread part becomes the content; `None` when nothing postable is found.
    pub fn from_structured(answer: &Value, mapping: &OutputMapping) -> Option<Self> {
        let text = |path: &str| (!path.trim().is_empty()).then(|| value_text(answer, path)).flatten();
        let non_blank = |value: Option<String>| value.map(|v| sanitize_agent_markdown(&v).trim().to_string()).filter(|v| !v.is_empty());
        let mut thread_parts: Vec<String> = string_list(mapped(answer, &mapping.thread_parts), false).iter().map(|part| sanitize_agent_markdown(part).trim().to_string()).filter(|part| !part.is_empty()).collect();
        let content = match non_blank(text(&mapping.content)) {
            Some(content) => content,
            None if !thread_parts.is_empty() => thread_parts.remove(0),
            None => return None,
        };
        Some(Self {
            content,
            image_prompt: non_blank(text(&mapping.image_prompt)),
            hashtags: string_list(mapped(answer, &mapping.hashtags), true).iter().map(|tag| normalize_tag(tag)).filter(|t| !t.is_empty()).collect(),
            mood: non_blank(text(&mapping.mood)),
            thread_parts,
            tint: text(&mapping.tint).as_deref().and_then(parse_tint),
        })
    }

    /// The post body: the content plus every hashtag it does not already contain.
    pub fn body(&self) -> String {
        let lower = self.content.to_lowercase();
        let missing: Vec<String> = self
            .hashtags
            .iter()
            .map(|tag| format!("{HASHTAG_SIGIL}{tag}"))
            .filter(|tag| !lower.contains(&tag.to_lowercase()))
            .collect();
        if missing.is_empty() {
            self.content.clone()
        } else {
            format!("{} {}", self.content, missing.join(" "))
        }
    }
}

fn mapped<'a>(answer: &'a Value, path: &str) -> Option<&'a Value> {
    (!path.trim().is_empty()).then(|| value_at(answer, path)).flatten()
}

/// Strings of an array, or one string (split on commas and spaces when `split`).
fn string_list(value: Option<&Value>, split: bool) -> Vec<String> {
    let items: Vec<String> = match value {
        Some(Value::Array(items)) => items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect(),
        Some(Value::String(text)) if split => text.split(HASHTAG_SEPARATORS).map(str::to_string).collect(),
        Some(Value::String(text)) => vec![text.clone()],
        _ => Vec::new(),
    };
    items.into_iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
}

/// `#Deep Work` → `DeepWork`.
fn normalize_tag(tag: &str) -> String {
    tag.trim_start_matches(HASHTAG_SIGIL).chars().filter(|c| c.is_alphanumeric() || *c == '_').collect()
}
