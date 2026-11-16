//! Text input entities bound to the selected agent/persona.

use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::*;

/// Text input entities bound to the currently selected agent/persona in the right pane.
pub struct AgentEditorInputs<'a> {
    pub name_input: &'a Entity<InputState>,
    pub handle_input: &'a Entity<InputState>,
    pub description_input: &'a Entity<InputState>,
    pub occupation_input: &'a Entity<InputState>,
    pub location_input: &'a Entity<InputState>,
    pub pronouns_input: &'a Entity<InputState>,
    pub avatar_path_input: &'a Entity<InputState>,
    pub bio_input: &'a Entity<TextareaState>,
    pub hobbies_input: &'a Entity<InputState>,
    pub values_input: &'a Entity<InputState>,
    pub routine_input: &'a Entity<InputState>,
    pub primary_lang_input: &'a Entity<InputState>,
    pub secondary_lang_input: &'a Entity<InputState>,
    pub typing_quirks_input: &'a Entity<InputState>,
    pub signature_phrases_input: &'a Entity<InputState>,
    pub avoided_words_input: &'a Entity<InputState>,
    pub preferred_topics_input: &'a Entity<InputState>,
    pub mention_instruction_input: &'a Entity<TextareaState>,
    pub system_prompt_input: &'a Entity<TextareaState>,
    pub prompt_template_input: &'a Entity<TextareaState>,
}
