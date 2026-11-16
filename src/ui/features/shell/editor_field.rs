//! The free-text fields of the Agent & Persona editor and their normalization rules.

use super::consts::{HANDLE_PREFIX, PATH_QUOTE};
use crate::domain::{AgentConfig, Profile};

/// Every free-text field of the Agent & Persona editor that is backed by an input entity.
///
/// Text inputs hold an unsaved draft until the user saves, while chips, steppers and presets
/// commit immediately. Routing both paths through this one list lets enum edits refresh only
/// the text fields they actually changed, so pending drafts survive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EditorTextField {
    Name,
    Handle,
    Description,
    Occupation,
    Location,
    Pronouns,
    AvatarPath,
    Bio,
    Hobbies,
    Values,
    Routine,
    PrimaryLanguage,
    SecondaryLanguages,
    TypingQuirks,
    SignaturePhrases,
    AvoidedWords,
    PreferredTopics,
    MentionInstruction,
    SystemPrompt,
    PromptTemplate,
}

use EditorTextField as F;

impl EditorTextField {
    pub(super) const ALL: [Self; 20] = [
        F::Name,
        F::Handle,
        F::Description,
        F::Occupation,
        F::Location,
        F::Pronouns,
        F::AvatarPath,
        F::Bio,
        F::Hobbies,
        F::Values,
        F::Routine,
        F::PrimaryLanguage,
        F::SecondaryLanguages,
        F::TypingQuirks,
        F::SignaturePhrases,
        F::AvoidedWords,
        F::PreferredTopics,
        F::MentionInstruction,
        F::SystemPrompt,
        F::PromptTemplate,
    ];

    /// The text an input shows for this field of `profile`.
    pub(super) fn display_value(self, profile: &Profile) -> String {
        let cfg = &profile.config;
        match self {
            F::Name => profile.name.clone(),
            F::Handle => profile.handle.clone(),
            F::Description => cfg.description.clone(),
            F::Occupation => cfg.occupation.clone(),
            F::Location => cfg.location_culture.clone(),
            F::Pronouns => cfg.pronouns.clone(),
            F::AvatarPath => profile.avatar_path.clone().unwrap_or_default(),
            F::Bio => cfg.bio.clone(),
            F::Hobbies => cfg.hobbies.clone(),
            F::Values => cfg.core_values.clone(),
            F::Routine => cfg.daily_routine.clone(),
            F::PrimaryLanguage => cfg.primary_language.clone(),
            F::SecondaryLanguages => cfg.secondary_languages.clone(),
            F::TypingQuirks => cfg.typing_quirks.clone(),
            F::SignaturePhrases => cfg.signature_phrases.clone(),
            F::AvoidedWords => cfg.avoided_words.clone(),
            F::PreferredTopics => cfg.preferred_topics.clone(),
            F::MentionInstruction => cfg.custom_mention_instruction.clone(),
            F::SystemPrompt => cfg.system_prompt.clone(),
            F::PromptTemplate => cfg.prompt_template.clone(),
        }
    }

    /// Writes the raw input text into `profile`, applying the field's normalization rules.
    pub(super) fn apply(self, profile: &mut Profile, raw: &str) {
        let trimmed = raw.trim();
        match self {
            // Name, handle and primary language are required: an empty input keeps the old value.
            F::Name if !trimmed.is_empty() => profile.name = trimmed.to_string(),
            F::Handle if !trimmed.is_empty() => {
                profile.handle = if trimmed.starts_with(HANDLE_PREFIX) {
                    trimmed.to_string()
                } else {
                    format!("{HANDLE_PREFIX}{trimmed}")
                };
            }
            F::PrimaryLanguage if !trimmed.is_empty() => {
                profile.config.primary_language = trimmed.to_string();
            }
            F::Name | F::Handle | F::PrimaryLanguage => {}
            F::AvatarPath => {
                let path = trimmed.trim_matches(PATH_QUOTE);
                crate::ui::atoms::forget_avatar(path);
                profile.avatar_path = (!path.is_empty()).then(|| path.to_string());
            }
            _ => self.apply_config_text(&mut profile.config, raw),
        }
    }

    /// Stores an optional config text field: short fields trimmed, long-form fields verbatim.
    fn apply_config_text(self, cfg: &mut AgentConfig, raw: &str) {
        let trimmed = raw.trim();
        match self {
            F::Description => cfg.description = trimmed.to_string(),
            F::Occupation => cfg.occupation = trimmed.to_string(),
            F::Location => cfg.location_culture = trimmed.to_string(),
            F::Pronouns => cfg.pronouns = trimmed.to_string(),
            // Long-form fields keep their whitespace verbatim.
            F::Bio => cfg.bio = raw.to_string(),
            F::Hobbies => cfg.hobbies = raw.to_string(),
            F::Values => cfg.core_values = raw.to_string(),
            F::Routine => cfg.daily_routine = raw.to_string(),
            F::SecondaryLanguages => cfg.secondary_languages = raw.to_string(),
            F::TypingQuirks => cfg.typing_quirks = raw.to_string(),
            F::SignaturePhrases => cfg.signature_phrases = raw.to_string(),
            F::AvoidedWords => cfg.avoided_words = raw.to_string(),
            F::PreferredTopics => cfg.preferred_topics = raw.to_string(),
            F::MentionInstruction => cfg.custom_mention_instruction = raw.to_string(),
            F::SystemPrompt => cfg.system_prompt = raw.to_string(),
            F::PromptTemplate => cfg.prompt_template = raw.to_string(),
            F::Name | F::Handle | F::PrimaryLanguage | F::AvatarPath => {}
        }
    }
}
