//! One action request: the action plus the audience, language or concept it was run with.

use super::kind::{ActionInput, ActionKind};
use crate::domain::document::keyed::keyed_enum;
use crate::i18n::Language;

keyed_enum! {
    /// Who an explanation is written for.
    AudiencePreset {
        Child => "child",
        Teenager => "teenager",
        Student => "student",
        Expert => "expert",
        Professor => "professor",
    }
}

impl AudiencePreset {
    /// How the prompt describes the reader (prompts are written in English).
    pub const fn describe(self) -> &'static str {
        match self {
            Self::Child => "a curious five-year-old: short sentences, everyday words, one concrete picture",
            Self::Teenager => "a teenager: plain language, a relatable example, no jargon left unexplained",
            Self::Student => "a university student: precise terms defined once, the reasoning step by step",
            Self::Expert => "an expert in the field: dense, technical, assumes the background, highlights what is new",
            Self::Professor => "a professor: situate it in the literature, name the schools of thought, note open problems",
        }
    }
}

/// The reader of an explanation: a preset or a description the user typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Audience {
    Preset(AudiencePreset),
    Custom(String),
}

impl Default for Audience {
    fn default() -> Self {
        Self::Preset(AudiencePreset::Teenager)
    }
}

impl Audience {
    pub fn describe(&self) -> String {
        match self {
            Self::Preset(preset) => preset.describe().to_string(),
            Self::Custom(text) => text.trim().to_string(),
        }
    }
}

/// The language of a translation: an app locale or a language the user typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetLanguage {
    Locale(Language),
    Typed(String),
}

impl TargetLanguage {
    /// The language's English name, as the prompt asks for it.
    pub fn english_name(&self) -> String {
        match self {
            Self::Locale(language) => language.english_label().to_string(),
            Self::Typed(name) => name.trim().to_string(),
        }
    }
}

/// An action ready to run: the kind plus the one choice it uses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRequest {
    pub kind: ActionKind,
    pub audience: Audience,
    pub language: TargetLanguage,
    /// The concept of "search concept" (blank until one is picked).
    pub concept: String,
}

impl ActionRequest {
    /// `kind` with the default choices: a teenager, the interface `language`, `concept`.
    pub fn new(kind: ActionKind, language: Language, concept: &str) -> Self {
        Self { kind, audience: Audience::default(), language: TargetLanguage::Locale(language), concept: concept.trim().to_string() }
    }

    /// Whether the request has everything it needs (a typed choice may still be blank).
    pub fn is_complete(&self) -> bool {
        match self.kind.input() {
            ActionInput::None => true,
            ActionInput::Audience => !self.audience.describe().is_empty(),
            ActionInput::Language => !self.language.english_name().is_empty(),
            ActionInput::Concept => !self.concept.is_empty(),
        }
    }

    /// A stable key of the request and its choice (results are remembered per key).
    pub fn cache_key(&self) -> String {
        let choice = match self.kind.input() {
            ActionInput::None => String::new(),
            ActionInput::Audience => self.audience.describe(),
            ActionInput::Language => self.language.english_name(),
            ActionInput::Concept => self.concept.clone(),
        };
        format!("{}:{}", self.kind.key(), choice.to_lowercase())
    }

    /// This request switched to `kind`, keeping the choices already made.
    pub fn with_kind(self, kind: ActionKind) -> Self {
        Self { kind, ..self }
    }
}
