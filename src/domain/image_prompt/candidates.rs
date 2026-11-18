//! "Write it for me": the request that asks the language model for three image prompts, and
//! a forgiving parser of its answer (a JSON array, a numbered or bulleted list, or labelled
//! lines; quotes and labels stripped, duplicates dropped).

use super::assemble::{assemble_prompt, ImageContext, MAX_PROMPT_CHARS};
use super::choices::{ImageChoices, ImageTarget};
use super::engine::PromptEngine;
use crate::domain::post_actions::ActionPrompt;

/// How many candidates are asked for and kept.
pub const CANDIDATE_COUNT: usize = 3;
/// Shortest line taken as a prompt (shorter ones are headings or noise).
const MIN_CANDIDATE_CHARS: usize = 12;
/// Answer budget of the request, in tokens.
pub const CANDIDATE_ANSWER_TOKENS: u32 = 600;

const SYSTEM: &str = "You write prompts for text-to-image models. Each prompt is one vivid, concrete paragraph of at most 60 words: subject, setting, composition, style, light and colour. Never put words or letters in the picture. Answer with exactly three prompts as a JSON array of strings and nothing else.";
const AVATAR_TASK: &str = "Write three different avatar prompts for this character.";
const POST_TASK: &str = "Write three different prompts for a picture that goes with this post.";
const DRAFT_LEAD: &str = "A starting point built from the user's picks (improve on it, keep its style and mood):";
const LIST_MARKERS: [char; 4] = ['-', '*', '•', '·'];
const QUOTES: [char; 6] = ['"', '\'', '“', '”', '«', '»'];
const LABEL_WORDS: [&str; 3] = ["prompt", "option", "candidate"];
/// A label is short ("Prompt 3", "Option B").
const LABEL_LIMIT: usize = 14;
/// A line ending like this introduces a list instead of being a prompt.
const HEADING_END: char = ':';

/// The request for three candidates about `context`, seeded with the assembled picks.
pub fn candidates_prompt(choices: &ImageChoices, context: &ImageContext) -> ActionPrompt {
    let engine = PromptEngine { kind: crate::domain::engine::ImageEngineKind::None, style_prefix: String::new(), size: String::new() };
    let draft = assemble_prompt(choices, context, &engine).text;
    let task = match context.target {
        ImageTarget::Avatar => AVATAR_TASK,
        ImageTarget::Post => POST_TASK,
    };
    let details = context.subject.details.join(", ");
    let user = format!("{task}\n\nSubject: {}\nDetails: {details}\n\n{DRAFT_LEAD}\n{draft}", context.subject.noun);
    ActionPrompt { system: SYSTEM.to_string(), user }
}

/// Up to [`CANDIDATE_COUNT`] distinct prompts found in `answer`.
pub fn parse_candidates(answer: &str) -> Vec<String> {
    let raw = json_strings(answer).unwrap_or_else(|| answer.lines().map(str::to_string).collect());
    raw.iter()
        .map(|line| cleaned(line))
        .filter(|line| line.chars().count() >= MIN_CANDIDATE_CHARS && !line.ends_with(HEADING_END))
        .fold(Vec::new(), |mut kept: Vec<String>, line| {
            if !kept.iter().any(|known| known.eq_ignore_ascii_case(&line)) {
                kept.push(line);
            }
            kept
        })
        .into_iter()
        .take(CANDIDATE_COUNT)
        .collect()
}

/// The strings of the first JSON array in `answer` (fenced or bare).
fn json_strings(answer: &str) -> Option<Vec<String>> {
    let (start, end) = (answer.find('[')?, answer.rfind(']')?);
    let values: Vec<serde_json::Value> = serde_json::from_str(answer.get(start..=end)?).ok()?;
    let strings: Vec<String> = values
        .into_iter()
        .filter_map(|value| match value {
            serde_json::Value::String(text) => Some(text),
            serde_json::Value::Object(map) => map.values().find_map(|v| v.as_str().map(str::to_string)),
            _ => None,
        })
        .collect();
    (!strings.is_empty()).then_some(strings)
}

/// One line without list markers, numbering, "Prompt 1:" labels, quotes or a trailing comma.
fn cleaned(line: &str) -> String {
    let line = line.trim().trim_start_matches(LIST_MARKERS).trim_start();
    let line = line.trim_start_matches(|c: char| c.is_ascii_digit()).trim_start_matches(['.', ')', ':']).trim_start();
    let line = strip_label(line);
    let line = line.trim().trim_end_matches(',').trim().trim_matches(QUOTES).trim();
    line.chars().take(MAX_PROMPT_CHARS).collect()
}

/// `line` without a leading "Prompt 2:" / "**Option A** —" label.
fn strip_label(line: &str) -> &str {
    let unbolded = line.trim_start_matches('*');
    let lower = unbolded.to_lowercase();
    let labelled = LABEL_WORDS.iter().any(|word| lower.starts_with(word));
    match (labelled, unbolded.find([':', '—', '-'])) {
        (true, Some(at)) if at < LABEL_LIMIT => unbolded[at..].trim_start_matches([':', '—', '-', '*', ' ']),
        _ => line,
    }
}
