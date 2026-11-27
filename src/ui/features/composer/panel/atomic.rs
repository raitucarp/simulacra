//! sb E1: the atomic-note hint of the composer. A pause in typing (debounced) re-checks the
//! focused draft with the pure linter; the hint shows under that draft when the check finds
//! something, can be dismissed for this composition, and offers to split the draft into one
//! post per claim. It never blocks posting, and Settings › Reading & Writing turns it off.

use super::{ComposerModalMode, ComposerModalPanel};
use crate::domain::thinking::atomic::findings::score_of;
use crate::domain::thinking::atomic::{check_atomicity, split_proposal, AtomicReport, Finding};
use crate::import::segment::SentenceLocale;
use crate::ui::features::composer::modal::AtomicHintView;
use gpui_kit::*;
use std::rc::Rc;
use std::time::Duration;

/// Quiet time after the last keystroke before the draft is checked.
const DEBOUNCE: Duration = Duration::from_millis(650);
/// Drafts shorter than this (characters) are not checked yet.
const MIN_CHECK_CHARS: usize = 24;

/// What the hint knows.
#[derive(Default)]
pub(crate) struct AtomicHintState {
    pub enabled: bool,
    pub locale: SentenceLocale,
    /// The checked draft and its report, while it has something to say.
    pub report: Option<(usize, AtomicReport)>,
    pub dismissed: bool,
    task: Option<Task<()>>,
}

/// The report without "too short": a post is capped at 280 characters and a thread part or a
/// quick note is short on purpose, so while writing that finding only nags (Q3). The vault
/// and the Insights checks still report it.
fn without_length_nag(mut report: AtomicReport) -> AtomicReport {
    report.findings.retain(|finding| !matches!(finding, Finding::TooShort { .. }));
    report.score = score_of(&report.findings);
    report
}

impl ComposerModalPanel {
    /// Turns the hint on or off and sets the sentence rules of the reading language.
    pub fn set_atomic_hints(&mut self, enabled: bool, locale: SentenceLocale, cx: &mut Context<Self>) {
        if self.atomic.enabled != enabled || self.atomic.locale != locale {
            (self.atomic.enabled, self.atomic.locale) = (enabled, locale);
            self.atomic.report = None;
            cx.notify();
        }
    }

    /// Re-checks the focused draft once typing pauses.
    pub(super) fn schedule_atomic_hint(&mut self, cx: &mut Context<Self>) {
        if !self.atomic.enabled || self.atomic.dismissed {
            return;
        }
        self.atomic.task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(DEBOUNCE).await;
            let _ = this.update(cx, |panel, cx| panel.refresh_atomic_hint(cx));
        }));
    }

    fn refresh_atomic_hint(&mut self, cx: &mut Context<Self>) {
        let index = self.active_draft.min(self.drafts.len().saturating_sub(1));
        let text = self.drafts.get(index).map(|slot| slot.input.read(cx).value().to_string()).unwrap_or_default();
        let report = (text.trim().chars().count() >= MIN_CHECK_CHARS).then(|| without_length_nag(check_atomicity(&text, self.atomic.locale)));
        let worth_showing = report.filter(|report| !report.findings.is_empty() || report.claims.len() > 1);
        self.atomic.report = worth_showing.map(|report| (index, report));
        cx.notify();
    }

    fn dismiss_atomic_hint(&mut self, cx: &mut Context<Self>) {
        (self.atomic.dismissed, self.atomic.report) = (true, None);
        cx.notify();
    }

    /// Replaces the checked draft with one draft per claim.
    fn split_atomic(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((index, _)) = self.atomic.report.take() else { return };
        let Some(text) = self.drafts.get(index).map(|slot| slot.input.read(cx).value().to_string()) else { return };
        self.replace_draft(index, split_proposal(&text, self.atomic.locale), window, cx);
    }

    /// The hint the dialog draws, if any.
    pub(super) fn atomic_hint_view(&self, cx: &Context<Self>) -> Option<AtomicHintView> {
        let (draft, report) = self.atomic.report.clone().filter(|_| self.atomic.enabled && !self.atomic.dismissed)?;
        let (split, dismiss) = (cx.entity().downgrade(), cx.entity().downgrade());
        Some(AtomicHintView {
            draft,
            report,
            on_split: Rc::new(move |window, cx| {
                let _ = split.update(cx, |panel, cx| panel.split_atomic(window, cx));
            }),
            on_dismiss: Rc::new(move |_, cx| {
                let _ = dismiss.update(cx, |panel, cx| panel.dismiss_atomic_hint(cx));
            }),
        })
    }

    /// Opens the composer in `mode` with `drafts` already written (the atomic split of a post).
    pub fn open_with_drafts(&mut self, mode: ComposerModalMode, drafts: Vec<String>, window: &mut Window, cx: &mut Context<Self>) {
        self.open(mode, window, cx);
        self.replace_draft(0, drafts, window, cx);
        self.focus_draft(0, window, cx);
    }

    /// Forgets the hint (a new composition starts).
    pub(super) fn reset_atomic_hint(&mut self) {
        (self.atomic.report, self.atomic.dismissed, self.atomic.task) = (None, false, None);
    }
}
