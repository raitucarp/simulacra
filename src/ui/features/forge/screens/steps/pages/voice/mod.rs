//! Voice (agents and personas): the live sample on top, then languages, word choice and
//! rhythm, how the text looks (capitals, punctuation, emoji) and personal habits. Every
//! change restyles the sample immediately.

mod sample;

use super::super::super::common::{card, heading};
use crate::state::forge::{DraftEdit, ScoreField, TextField, TraitField};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::ScoreSlider;
use crate::state::forge::suggest::Param;
use crate::ui::features::forge::text::field_label;
use gpui_kit::*;

const LANGUAGES_KEY: &str = "forge.ui.voice.languages";
const WORDS_KEY: &str = "forge.ui.voice.words";
const LOOK_KEY: &str = "forge.ui.voice.look";
const HABITS_KEY: &str = "forge.ui.voice.habits";

impl ForgeView {
    pub(in super::super) fn voice_page(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        div()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.voice_sample_card(handle))
            .child(
                card(theme)
                    .child(heading(tr(LANGUAGES_KEY), theme))
                    .child(self.text_row(TextField::PrimaryLanguage, handle))
                    .child(self.text_row(TextField::SecondaryLanguages, handle))
                    .child(self.trait_block(TraitField::CodeSwitching, handle)),
            )
            .child(
                card(theme)
                    .child(heading(tr(WORDS_KEY), theme))
                    .child(self.trait_block(TraitField::DictionRegister, handle))
                    .child(self.score_row(ScoreField::VocabularyComplexity, handle))
                    .child(self.trait_block(TraitField::SentenceLength, handle))
                    .child(self.trait_block(TraitField::GrammarStyle, handle)),
            )
            .child(
                card(theme)
                    .child(heading(tr(LOOK_KEY), theme))
                    .child(self.trait_block(TraitField::Capitalization, handle))
                    .child(self.trait_block(TraitField::PunctuationHabit, handle))
                    .child(self.trait_block(TraitField::EmojiUsage, handle)),
            )
            .child(
                card(theme)
                    .child(heading(tr(HABITS_KEY), theme))
                    .child(self.text_row(TextField::SignaturePhrases, handle))
                    .child(self.text_row(TextField::TypingQuirks, handle))
                    .child(self.text_row(TextField::AvoidedWords, handle)),
            )
            .into_any_element()
    }

    /// A score slider with its field's "?" explainer.
    pub(super) fn score_row(&self, field: ScoreField, handle: &ForgeHandle) -> Div {
        let edit = handle.edit(move |value: u8| DraftEdit::Score(field, value));
        let hint = self.slider_hint(Param::Score(field), &field_label(field.path()));
        let slider = ScoreSlider::new(field, &self.draft.profile().config, &self.theme, move |v, w, cx| edit(v, w, cx)).hint(hint);
        div().child(slider)
    }
}
