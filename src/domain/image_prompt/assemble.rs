//! Pure prompt assembly: picks + context + engine → the prompt sent to the image engine.

use super::choices::{ImageChoices, ImageTarget};
use super::engine::PromptEngine;
use super::options::{ImageAvoid, ImagePalette};
use super::subject::ImageSubject;

/// Longest prompt sent, in characters (hosted engines cap the prompt length).
pub const MAX_PROMPT_CHARS: usize = 900;
/// Theme colours named in a "match my theme" palette.
const MAX_THEME_COLOURS: usize = 3;
const SEPARATOR: &str = ", ";
const POST_SUBJECT_LEAD: &str = "visual metaphor for: ";
const PALETTE_HINT_LEAD: &str = "colours inspired by the";
const PALETTE_HINT_TAIL: &str = "palette";
const AND: &str = " and ";

/// Everything the prompt is about besides the picks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageContext {
    pub target: ImageTarget,
    pub subject: ImageSubject,
    /// Names of the active theme's main colours ("violet", "gold"), for "match my theme".
    pub theme_colours: Vec<String>,
}

/// A finished prompt for one engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImagePrompt {
    pub text: String,
    /// Only for engines that take a separate negative prompt.
    pub negative: Option<String>,
    /// `WIDTHxHEIGHT`.
    pub size: String,
}

impl ImagePrompt {
    /// The prompt text and the negative prompt.
    pub fn into_parts(self) -> (String, Option<String>) {
        (self.text, self.negative)
    }
}

/// Assembles the prompt of `choices` about `context` for `engine`.
pub fn assemble_prompt(choices: &ImageChoices, context: &ImageContext, engine: &PromptEngine) -> ImagePrompt {
    let negative_ok = engine.supports_negative();
    let subject = subject_phrase(context);
    let positives = if negative_ok { Vec::new() } else { choices.avoid.iter().map(|hint| hint.positive().to_string()).collect() };
    let parts = [Some(engine.style_prefix.clone()), Some(choices.style.phrase().to_string()), Some(choices.framing.phrase().to_string()), subject]
        .into_iter()
        .flatten()
        .chain(context.subject.details.iter().cloned())
        .chain([Some(choices.mood.phrase().to_string()), palette_phrase(choices.palette, context)].into_iter().flatten())
        .chain(positives)
        .filter(|part| !part.trim().is_empty())
        .collect::<Vec<_>>();
    let negative = negative_ok.then(|| negative_phrase(&choices.avoid)).filter(|text| !text.is_empty());
    ImagePrompt { text: clipped(&parts.join(SEPARATOR)), negative, size: engine.size.clone() }
}

fn subject_phrase(context: &ImageContext) -> Option<String> {
    let noun = context.subject.noun.trim();
    match (noun.is_empty(), context.target) {
        (true, _) => None,
        (false, ImageTarget::Avatar) => Some(noun.to_string()),
        (false, ImageTarget::Post) => Some(format!("{POST_SUBJECT_LEAD}{noun}")),
    }
}

/// The palette words; "match my theme" names the theme colours, else the agent's palette.
fn palette_phrase(palette: ImagePalette, context: &ImageContext) -> Option<String> {
    if palette != ImagePalette::MatchTheme {
        return Some(palette.phrase().to_string());
    }
    let colours: Vec<&str> = context.theme_colours.iter().map(String::as_str).take(MAX_THEME_COLOURS).collect();
    match (colours.as_slice(), &context.subject.palette_hint) {
        ([], Some(hint)) => Some(format!("{PALETTE_HINT_LEAD} {} {PALETTE_HINT_TAIL}", hint.replace('_', " "))),
        ([], None) => None,
        (names, _) => Some(format!("{} {}", palette.phrase(), joined_names(names))),
    }
}

/// "violet", "violet and gold", "violet, gold and black".
fn joined_names(names: &[&str]) -> String {
    match names.split_last() {
        Some((last, [])) => last.to_string(),
        Some((last, rest)) => format!("{}{AND}{last}", rest.join(SEPARATOR)),
        None => String::new(),
    }
}

fn negative_phrase(avoid: &[ImageAvoid]) -> String {
    avoid.iter().map(|hint| hint.phrase()).collect::<Vec<_>>().join(SEPARATOR)
}

/// `text` cut to [`MAX_PROMPT_CHARS`] at the last separator that fits.
fn clipped(text: &str) -> String {
    if text.chars().count() <= MAX_PROMPT_CHARS {
        return text.to_string();
    }
    let head: String = text.chars().take(MAX_PROMPT_CHARS).collect();
    head.rfind(SEPARATOR).map_or(head.clone(), |at| head[..at].to_string())
}
