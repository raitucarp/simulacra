//! The AI's values for one parameter, as sparkle chips under its field (click to use), and
//! catalog example chips for Engine text settings.

use super::why_card::{option_text, text};
use super::ID_PREFIX;
use crate::domain::EngineConfig;
use crate::state::forge::suggest::ai::SuggestedValue;
use crate::state::forge::suggest::{range_hint, suggestions_for, EngineParam, FieldSuggestions, OptionRef, Param, SuggestContext};
use crate::state::forge::EngineSection;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::suggest::{ChipSpec, HintChips};
use crate::state::forge::suggest::Joiner;
use std::rc::Rc;

const LIST_SEPARATOR: &str = ", ";

impl ForgeView {
    /// How an AI value reads on its chip.
    fn value_label(&self, param: Param, value: &SuggestedValue) -> String {
        match value {
            SuggestedValue::Text(text) => text.clone(),
            SuggestedValue::Number(number) => {
                range_hint(param, &SuggestContext::of(&self.draft)).map_or_else(|| number.to_string(), |hint| hint.unit.format(*number))
            }
            SuggestedValue::Option(key) => option_text(OptionRef { param, key }),
            SuggestedValue::Options(keys) => keys.iter().map(|key| option_text(OptionRef { param, key })).collect::<Vec<_>>().join(LIST_SEPARATOR),
        }
    }

    /// Sparkle chips with the AI's values for `param`, if it proposed any.
    pub(in super::super) fn ai_chips(&self, param: Param, handle: &ForgeHandle) -> Option<HintChips> {
        let values = self.suggest.values(param);
        if values.is_empty() {
            return None;
        }
        let chips = values.iter().enumerate().map(|(at, value)| ChipSpec { label: self.value_label(param, value), value: at.to_string() }).collect();
        let accept = handle.with(move |this, index: String, _, cx| {
            if let Ok(index) = index.parse::<usize>() {
                this.accept_suggestion(param, index, cx);
            }
        });
        let id = format!("{ID_PREFIX}-ai-values-{}", param.path());
        Some(HintChips::new(id, chips, (String::new(), Joiner::Space), &self.theme, accept).ai())
    }

    /// Example chips (and AI values) under an Engine text setting; `set` writes the text.
    pub(in super::super) fn engine_examples(
        &self,
        param: EngineParam,
        (name, current): (&str, &str),
        (section, handle): (EngineSection, &ForgeHandle),
        set: Rc<dyn Fn(&mut EngineConfig, String)>,
    ) -> Option<HintChips> {
        let FieldSuggestions::Text(hints) = suggestions_for(Param::Engine(param), &SuggestContext::of(&self.draft)) else { return None };
        let chips = hints.examples.iter().map(|example| ChipSpec { label: text(&example.label), value: text(&example.value) }).collect();
        let write = handle.with(move |this, value: String, _, cx| {
            let set = set.clone();
            this.draft.seal_history();
            this.edit_engine(Some(section), move |engine| set(engine, value), cx);
            this.draft.seal_history();
        });
        Some(HintChips::new(format!("{ID_PREFIX}-engine-{name}"), chips, (current.to_string(), hints.joiner), &self.theme, write))
    }
}
