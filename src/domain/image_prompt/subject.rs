//! What the picture shows ([`ImageSubject`]), derived from an agent's or persona's settings
//! (body, form, material, scale, mood, voice, palette) or from a post's text.

use crate::domain::traits::{Material, Scale, StylisticSchool};
use crate::domain::{AgentConfig, TraitOption};

/// Most words of a post kept as the picture's gist.
pub const GIST_MAX_WORDS: usize = 28;
const SENTENCE_ENDS: [char; 4] = ['.', '!', '?', '\n'];
const LINK_PREFIXES: [&str; 2] = ["http://", "https://"];
const MARKUP: [char; 6] = ['[', ']', '#', '*', '_', '`'];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

/// The drawable subject: a noun phrase plus a few descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ImageSubject {
    /// "a crow", "a philosopher", or a post's gist.
    pub noun: String,
    /// Extra descriptors ("made of metal", "feeling curious").
    pub details: Vec<String>,
    /// The key of the agent's own palette, as a colour hint.
    pub palette_hint: Option<String>,
}

impl ImageSubject {
    /// Whether there is nothing to draw yet.
    pub fn is_empty(&self) -> bool {
        self.noun.trim().is_empty()
    }
}

/// `word` with its indefinite article ("a philosopher", "an engineer").
pub fn with_article(word: &str) -> String {
    let word = word.trim().to_lowercase();
    let article = if word.starts_with(VOWELS) { "an" } else { "a" };
    format!("{article} {word}")
}

fn spaced(key: &str) -> String {
    key.replace('_', " ")
}

/// The subject of an avatar for `config` (agent or persona).
pub fn subject_from_config(config: &AgentConfig) -> ImageSubject {
    let body = &config.embodiment;
    let occupation = config.occupation.trim();
    let noun = match (body.is_human(), occupation.is_empty()) {
        (true, false) => with_article(occupation),
        _ => body.form.noun().to_string(),
    };
    let material = (!matches!(body.material, Material::Flesh | Material::Immaterial)).then(|| format!("made of {}", spaced(body.material.key())));
    let scale = (body.scale != Scale::HumanSized).then(|| format!("{} scale", spaced(body.scale.key())));
    let mood = Some(config.current_mood.trim()).filter(|mood| !mood.is_empty()).map(|mood| format!("feeling {}", mood.to_lowercase()));
    let school = config.rhetoric.stylistic_school;
    let voice = (school != StylisticSchool::Plain).then(|| format!("{} sensibility", spaced(school.key())));
    let details = [material, scale, mood, voice].into_iter().flatten().collect();
    ImageSubject { noun, details, palette_hint: config.palette.clone().filter(|key| !key.trim().is_empty()) }
}

/// The subject of a picture for a post: the gist of its first sentence.
pub fn subject_from_post(text: &str) -> ImageSubject {
    ImageSubject { noun: post_gist(text), details: Vec::new(), palette_hint: None }
}

/// The first sentence of `text` without links or markup, at most [`GIST_MAX_WORDS`] words.
pub fn post_gist(text: &str) -> String {
    let words: Vec<String> = text
        .split_whitespace()
        .filter(|word| !LINK_PREFIXES.iter().any(|prefix| word.starts_with(prefix)))
        .map(|word| word.chars().filter(|c| !MARKUP.contains(c)).collect::<String>())
        .filter(|word| !word.is_empty())
        .collect();
    let joined = words.join(" ");
    let first = joined.split_inclusive(SENTENCE_ENDS).find(|sentence| !sentence.trim().is_empty()).unwrap_or_default();
    let trimmed = first.trim().trim_end_matches(SENTENCE_ENDS);
    trimmed.split_whitespace().take(GIST_MAX_WORDS).collect::<Vec<_>>().join(" ")
}
