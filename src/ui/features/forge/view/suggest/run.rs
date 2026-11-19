//! Asking the AI for values that fit the rest of the draft, and reading its answer.

use super::{SuggestJob, SuggestRun, SuggestStatus};
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::post_actions::ActionFailureKind;
use crate::i18n::Language;
use crate::state::forge::suggest::ai::{answer_tokens, parse_suggestions, suggest_prompt, targets, SuggestScope};
use crate::state::forge::suggest::SuggestContext;
use crate::ui::features::forge::view::{ForgeEvent, ForgeView};
use crate::ui::features::post_actions::failure_text;
use gpui_kit::*;

const NOTHING_HERE_KEY: &str = "suggest.ui.ai_nothing_here";
const NONE_KEY: &str = "suggest.ui.ai_none";
const UNREADABLE_KEY: &str = "suggest.ui.ai_unreadable";

/// The interface language's English name (free text is asked for in it).
fn interface_language() -> &'static str {
    let locale = rust_i18n::locale();
    let language = Language::ALL.iter().find(|language| language.code() == &*locale).or_else(|| Language::ALL.first());
    language.map(Language::english_label).unwrap_or_default()
}

impl ForgeView {
    /// Asks the AI for values of every suggestible parameter in `scope`.
    pub fn request_suggestions(&mut self, scope: SuggestScope, cx: &mut Context<Self>) {
        self.suggest.reset();
        let wanted = targets(scope, self.draft.kind(), &self.draft.profile().config);
        if wanted.is_empty() {
            self.suggest.status = SuggestStatus::Note(NOTHING_HERE_KEY);
            cx.notify();
            return;
        }
        let prompt = suggest_prompt(&wanted, &SuggestContext::of(&self.draft), interface_language());
        let cancel = CancelToken::new();
        let job = SuggestJob { generation: self.suggest.generation, prompt, answer_tokens: answer_tokens(&wanted), cancel: cancel.clone() };
        self.suggest.run = Some(SuggestRun { scope, targets: wanted, cancel });
        self.guide.thinking = true;
        cx.emit(ForgeEvent::Suggest(job));
        cx.notify();
    }

    /// Stops the request in flight.
    pub fn cancel_suggestions(&mut self, cx: &mut Context<Self>) {
        self.suggest.reset();
        self.guide.thinking = false;
        cx.notify();
    }

    /// The host's answer to request `generation` (ignored when a newer request replaced it).
    pub fn suggestions_arrived(&mut self, generation: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        if generation != self.suggest.generation {
            return;
        }
        let Some(run) = self.suggest.run.take() else { return };
        self.guide.thinking = false;
        self.suggest.status = match outcome {
            Err(failure) if failure.kind == ActionFailureKind::Cancelled => SuggestStatus::Idle,
            Err(failure) => SuggestStatus::Failed(failure_text(failure.kind, &failure.detail)),
            Ok(text) => match parse_suggestions(&text, &run.targets, &SuggestContext::of(&self.draft)) {
                None => SuggestStatus::Note(UNREADABLE_KEY),
                Some(parsed) if parsed.is_empty() => SuggestStatus::Note(NONE_KEY),
                Some(parsed) => {
                    let count = parsed.values.iter().map(|(_, values)| values.len()).sum();
                    self.suggest.results = parsed.values;
                    SuggestStatus::Ready { count, discarded: parsed.discarded }
                }
            },
        };
        cx.notify();
    }
}
