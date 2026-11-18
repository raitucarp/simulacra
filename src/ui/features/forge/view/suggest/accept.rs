//! Using suggestions: one AI value, every first AI value at once, or "fill the rest" with the
//! catalog's defaults — each as one undo step.

use super::SuggestStatus;
use crate::state::forge::suggest::ai::value_edit;
use crate::state::forge::suggest::{fill_edits, Param, Phrase, SuggestContext};
use crate::state::forge::DraftEdit;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::ForgeView;
use gpui_kit::*;

const FILL_NOTHING_KEY: &str = "suggest.ui.fill_nothing";

impl ForgeView {
    /// Uses AI value number `index` of `param`; its other alternatives go away.
    pub fn accept_suggestion(&mut self, param: Param, index: usize, cx: &mut Context<Self>) {
        let edit = self.suggest.values(param).get(index).and_then(|value| value_edit(param, value, self.draft.profile()));
        self.suggest.results.retain(|(candidate, _)| *candidate != param);
        if self.suggest.results.is_empty() {
            self.suggest.status = SuggestStatus::Idle;
        }
        match edit {
            Some(edit) => self.apply_edit(edit, cx),
            None => cx.notify(),
        }
    }

    /// Uses the first AI value of every parameter as one undo step.
    pub fn accept_all_suggestions(&mut self, cx: &mut Context<Self>) {
        let profile = self.draft.profile().clone();
        let edits: Vec<DraftEdit> = self
            .suggest
            .results
            .iter()
            .filter_map(|(param, values)| values.first().and_then(|value| value_edit(*param, value, &profile)))
            .collect();
        self.suggest.reset();
        self.apply_edits(&edits, cx);
        cx.notify();
    }

    /// Forgets the AI values.
    pub fn dismiss_suggestions(&mut self, cx: &mut Context<Self>) {
        self.suggest.reset();
        cx.notify();
    }

    /// Fills every blank field of the current step (and every setting still at its starting
    /// value) with the catalog's suggestion, as one undo step.
    pub fn fill_rest(&mut self, cx: &mut Context<Self>) {
        let resolve = |phrase: &Phrase| phrase.resolve(tr);
        let edits = fill_edits(self.draft.step(), &SuggestContext::of(&self.draft), &self.context.existing, &resolve);
        self.suggest.reset();
        self.suggest.status = if edits.is_empty() { SuggestStatus::Note(FILL_NOTHING_KEY) } else { SuggestStatus::Filled(edits.len()) };
        self.apply_edits(&edits, cx);
        cx.notify();
    }
}
