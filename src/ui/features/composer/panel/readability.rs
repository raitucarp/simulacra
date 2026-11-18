//! sb F3: the composer's readability meter. A pause in typing (debounced) scores the focused
//! draft with the pure per-language formulas (`domain::readability`); the meter shows beside
//! E1's atomic hint, can be dismissed for this composition, never blocks posting, and
//! Settings › Reading & Writing turns it off.

use super::readability_meter::ReadabilityMeterView;
use super::ComposerModalPanel;
use crate::domain::readability::{readability, Readability, ReadingLanguage};
use gpui_kit::*;
use std::rc::Rc;
use std::time::Duration;

/// Quiet time after the last keystroke before the draft is scored.
const DEBOUNCE: Duration = Duration::from_millis(650);
/// Drafts shorter than this (characters) are not scored yet: a few words swing wildly.
const MIN_SCORE_CHARS: usize = 40;

/// What the meter knows.
#[derive(Default)]
pub(crate) struct ReadabilityState {
    pub enabled: bool,
    pub language: ReadingLanguage,
    /// The scored draft and its score.
    pub score: Option<(usize, Readability)>,
    pub dismissed: bool,
    task: Option<Task<()>>,
}

impl ComposerModalPanel {
    /// Turns the meter on or off and sets the reading language its formulas follow.
    pub fn set_readability_meter(&mut self, enabled: bool, language: ReadingLanguage, cx: &mut Context<Self>) {
        if self.readability.enabled != enabled || self.readability.language != language {
            (self.readability.enabled, self.readability.language) = (enabled, language);
            self.readability.score = None;
            self.schedule_readability(cx);
            cx.notify();
        }
    }

    /// Re-scores the focused draft once typing pauses.
    pub(super) fn schedule_readability(&mut self, cx: &mut Context<Self>) {
        if !self.readability.enabled || self.readability.dismissed {
            return;
        }
        self.readability.task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(DEBOUNCE).await;
            let _ = this.update(cx, |panel, cx| panel.refresh_readability(cx));
        }));
    }

    fn refresh_readability(&mut self, cx: &mut Context<Self>) {
        let index = self.active_draft.min(self.drafts.len().saturating_sub(1));
        let text = self.drafts.get(index).map(|slot| slot.input.read(cx).value().to_string()).unwrap_or_default();
        let long_enough = text.trim().chars().count() >= MIN_SCORE_CHARS;
        let score = long_enough.then(|| readability(&text, self.readability.language)).flatten();
        let next = score.map(|score| (index, score));
        if next != self.readability.score {
            self.readability.score = next;
            cx.notify();
        }
    }

    fn dismiss_readability(&mut self, cx: &mut Context<Self>) {
        (self.readability.dismissed, self.readability.score) = (true, None);
        cx.notify();
    }

    /// The meter the dialog draws, if any.
    pub(super) fn readability_view(&self, cx: &Context<Self>) -> Option<ReadabilityMeterView> {
        let (draft, score) = self.readability.score.clone().filter(|_| self.readability.enabled && !self.readability.dismissed)?;
        let dismiss = cx.entity().downgrade();
        Some(ReadabilityMeterView {
            draft,
            score,
            on_dismiss: Rc::new(move |_, cx| {
                let _ = dismiss.update(cx, |panel, cx| panel.dismiss_readability(cx));
            }),
        })
    }

    /// Forgets the meter (a new composition starts).
    pub(super) fn reset_readability(&mut self) {
        (self.readability.score, self.readability.dismissed, self.readability.task) = (None, false, None);
    }
}
