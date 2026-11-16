//! Table lookup from an [`EditorTextField`] to the input entity that edits it.

use super::editor_field::EditorTextField;
use super::editor_inputs::AgentEditorFields;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::*;

/// A single-line or multi-line editor input.
#[derive(Clone, Copy)]
pub(super) enum EditorSlot<'a> {
    Line(&'a Entity<InputState>),
    Area(&'a Entity<TextareaState>),
}

impl EditorSlot<'_> {
    /// Current (possibly unsaved) text of the input.
    pub(super) fn read(self, cx: &App) -> String {
        match self {
            Self::Line(input) => input.read(cx).value().to_string(),
            Self::Area(input) => input.read(cx).value().to_string(),
        }
    }

    /// Replaces the input text.
    pub(super) fn set(self, value: &str, window: &mut Window, cx: &mut App) {
        match self {
            Self::Line(input) => input.update(cx, |s, cx| s.set_value(value, window, cx)),
            Self::Area(input) => input.update(cx, |s, cx| s.set_value(value, window, cx)),
        }
    }
}

impl AgentEditorFields {
    /// Every input, in [`EditorTextField::ALL`] order.
    pub(super) fn slots(&self) -> [EditorSlot<'_>; EditorTextField::ALL.len()] {
        use EditorSlot::{Area, Line};
        let p = self.as_params();
        [
            Line(p.name_input),
            Line(p.handle_input),
            Line(p.description_input),
            Line(p.occupation_input),
            Line(p.location_input),
            Line(p.pronouns_input),
            Line(p.avatar_path_input),
            Area(p.bio_input),
            Line(p.hobbies_input),
            Line(p.values_input),
            Line(p.routine_input),
            Line(p.primary_lang_input),
            Line(p.secondary_lang_input),
            Line(p.typing_quirks_input),
            Line(p.signature_phrases_input),
            Line(p.avoided_words_input),
            Line(p.preferred_topics_input),
            Area(p.mention_instruction_input),
            Area(p.system_prompt_input),
            Area(p.prompt_template_input),
        ]
    }

    /// The input editing `field`.
    pub(super) fn slot(&self, field: EditorTextField) -> EditorSlot<'_> {
        self.slots()[field as usize]
    }

    /// Pairs each field with its input.
    pub(super) fn fields(&self) -> impl Iterator<Item = (EditorTextField, EditorSlot<'_>)> {
        EditorTextField::ALL.into_iter().zip(self.slots())
    }
}
