//! Input entities backing the free-text fields of the Agent & Persona editor.

use super::consts::{EDITOR_LONG_ROWS, EDITOR_SHORT_ROWS};
use super::editor_field::EditorTextField;
use super::placeholders as ph;
use crate::domain::Profile;
use crate::ui::features::agent_editor::AgentEditorInputs;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::*;

/// One owned input entity per [`EditorTextField`].
pub struct AgentEditorFields {
    name: Entity<InputState>,
    handle: Entity<InputState>,
    description: Entity<InputState>,
    occupation: Entity<InputState>,
    location: Entity<InputState>,
    pronouns: Entity<InputState>,
    avatar_path: Entity<InputState>,
    bio: Entity<TextareaState>,
    hobbies: Entity<InputState>,
    values: Entity<InputState>,
    routine: Entity<InputState>,
    primary_lang: Entity<InputState>,
    secondary_lang: Entity<InputState>,
    typing_quirks: Entity<InputState>,
    signature_phrases: Entity<InputState>,
    avoided_words: Entity<InputState>,
    preferred_topics: Entity<InputState>,
    mention_instruction: Entity<TextareaState>,
    system_prompt: Entity<TextareaState>,
    prompt_template: Entity<TextareaState>,
}

/// Creates editor inputs pre-filled from one profile.
struct FieldFactory<'a> {
    profile: &'a Profile,
    window: &'a mut Window,
    cx: &'a mut App,
}

impl FieldFactory<'_> {
    fn line(&mut self, field: EditorTextField, placeholder: &'static str) -> Entity<InputState> {
        let value = field.display_value(self.profile);
        let window = &mut *self.window;
        self.cx.new(|cx| InputState::new(window, cx).default_value(value).placeholder(placeholder))
    }

    fn area(
        &mut self,
        field: EditorTextField,
        placeholder: &'static str,
        (min_rows, max_rows): (usize, usize),
    ) -> Entity<TextareaState> {
        let value = field.display_value(self.profile);
        let window = &mut *self.window;
        self.cx.new(|cx| {
            TextareaState::new(window, cx)
                .default_value(value)
                .placeholder(placeholder)
                .auto_grow(min_rows, max_rows)
        })
    }
}

impl AgentEditorFields {
    /// Creates every editor input, showing the values of `profile`.
    pub(super) fn new(profile: &Profile, window: &mut Window, cx: &mut App) -> Self {
        use EditorTextField as F;
        let mut f = FieldFactory { profile, window, cx };
        Self {
            name: f.line(F::Name, ph::NAME),
            handle: f.line(F::Handle, ph::HANDLE),
            description: f.line(F::Description, ph::DESCRIPTION),
            occupation: f.line(F::Occupation, ph::OCCUPATION),
            location: f.line(F::Location, ph::LOCATION),
            pronouns: f.line(F::Pronouns, ph::PRONOUNS),
            avatar_path: f.line(F::AvatarPath, ph::AVATAR_PATH),
            bio: f.area(F::Bio, ph::BIO, EDITOR_LONG_ROWS),
            hobbies: f.line(F::Hobbies, ph::HOBBIES),
            values: f.line(F::Values, ph::VALUES),
            routine: f.line(F::Routine, ph::ROUTINE),
            primary_lang: f.line(F::PrimaryLanguage, ph::PRIMARY_LANGUAGE),
            secondary_lang: f.line(F::SecondaryLanguages, ph::SECONDARY_LANGUAGES),
            typing_quirks: f.line(F::TypingQuirks, ph::TYPING_QUIRKS),
            signature_phrases: f.line(F::SignaturePhrases, ph::SIGNATURE_PHRASES),
            avoided_words: f.line(F::AvoidedWords, ph::AVOIDED_WORDS),
            preferred_topics: f.line(F::PreferredTopics, ph::PREFERRED_TOPICS),
            mention_instruction: f.area(F::MentionInstruction, ph::MENTION, EDITOR_SHORT_ROWS),
            system_prompt: f.area(F::SystemPrompt, ph::SYSTEM_PROMPT, EDITOR_LONG_ROWS),
            prompt_template: f.area(F::PromptTemplate, ph::PROMPT_TEMPLATE, EDITOR_LONG_ROWS),
        }
    }

    /// Borrowed view of the inputs in the shape the editor renderer expects.
    pub(super) fn as_params(&self) -> AgentEditorInputs<'_> {
        AgentEditorInputs {
            name_input: &self.name,
            handle_input: &self.handle,
            description_input: &self.description,
            occupation_input: &self.occupation,
            location_input: &self.location,
            pronouns_input: &self.pronouns,
            avatar_path_input: &self.avatar_path,
            bio_input: &self.bio,
            hobbies_input: &self.hobbies,
            values_input: &self.values,
            routine_input: &self.routine,
            primary_lang_input: &self.primary_lang,
            secondary_lang_input: &self.secondary_lang,
            typing_quirks_input: &self.typing_quirks,
            signature_phrases_input: &self.signature_phrases,
            avoided_words_input: &self.avoided_words,
            preferred_topics_input: &self.preferred_topics,
            mention_instruction_input: &self.mention_instruction,
            system_prompt_input: &self.system_prompt,
            prompt_template_input: &self.prompt_template,
        }
    }
}
