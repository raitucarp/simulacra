//! What the card maker does: open on a source, choose a sentence and a kind, pick cloze
//! words, add cards to the list, ask the AI, save.

use super::fields::{source_at, MakerKind, MakerRequest};
use super::{AiStatus, FlashcardMaker, MakerEvent};
use crate::domain::review::card::{draft_problem, parse_tags};
use crate::domain::review::{CardDraft, DraftKind};
use gpui_kit::{Context, Window};
use std::collections::BTreeSet;

impl FlashcardMaker {
    /// Opens the sheet on `request`, the first field holding its initial text.
    pub fn open(&mut self, request: MakerRequest, window: &mut Window, cx: &mut Context<Self>) {
        let initial = request.initial_text.clone();
        self.request = Some(request);
        (self.chosen, self.kind, self.pending, self.problem, self.ai) = (None, MakerKind::default(), Vec::new(), None, AiStatus::Idle);
        self.set_fields(&initial, "", window, cx);
        self.tags.update(cx, |input, cx| input.set_value("", window, cx));
        self.front.update(cx, |input, cx| input.focus(window, cx));
        cx.notify();
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        self.request = None;
        cx.notify();
    }

    fn set_fields(&mut self, front: &str, back: &str, window: &mut Window, cx: &mut Context<Self>) {
        let (front, back) = (front.to_string(), back.to_string());
        self.front.update(cx, |input, cx| input.set_value(front, window, cx));
        self.back.update(cx, |input, cx| input.set_value(back, window, cx));
        self.picked = BTreeSet::new();
    }

    pub(super) fn select_kind(&mut self, kind: MakerKind, cx: &mut Context<Self>) {
        self.kind = kind;
        self.problem = None;
        cx.notify();
    }

    /// Starts the fields from sentence `index`: the cloze text, or the answer of a question.
    pub(super) fn choose_sentence(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(text) = self.request.as_ref().and_then(|r| r.sentences.get(index)).map(|s| s.text.clone()) else { return };
        self.chosen = Some(index);
        match self.kind {
            MakerKind::Cloze => self.set_fields(&text, "", window, cx),
            MakerKind::Basic | MakerKind::Concept => self.set_fields("", &text, window, cx),
        }
        cx.notify();
    }

    pub(super) fn toggle_word(&mut self, index: usize, cx: &mut Context<Self>) {
        if !self.picked.remove(&index) {
            self.picked.insert(index);
        }
        self.problem = None;
        cx.notify();
    }

    /// The fields as a card of the list, citing the chosen sentence's place.
    fn draft_with_source(&self, kind: DraftKind, cx: &Context<Self>) -> Option<CardDraft> {
        let request = self.request.as_ref()?;
        let sentence = self.chosen.and_then(|i| request.sentences.get(i));
        Some(CardDraft { kind, source: source_at(&request.source, sentence), tags: parse_tags(&self.tags.read(cx).value()) })
    }

    /// Adds the fields to the list (or shows what is missing) and clears them.
    pub(super) fn add_current(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let kind = self.current_draft(cx);
        self.problem = draft_problem(&kind);
        let added = self.problem.is_none() && self.draft_with_source(kind, cx).map(|draft| self.pending.push(draft)).is_some();
        if added {
            self.set_fields("", "", window, cx);
            self.front.update(cx, |input, cx| input.focus(window, cx));
        }
        cx.notify();
        added
    }

    pub(super) fn remove_pending(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.pending.len() {
            self.pending.remove(index);
            cx.notify();
        }
    }

    /// Saves the list plus the fields when they hold a complete card.
    pub(super) fn save(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let has_fields = !self.front.read(cx).value().trim().is_empty();
        if has_fields && !self.add_current(window, cx) {
            return;
        }
        if self.pending.is_empty() {
            self.problem = draft_problem(&self.current_draft(cx));
            cx.notify();
            return;
        }
        cx.emit(MakerEvent::Save(std::mem::take(&mut self.pending)));
    }

    /// Asks the shell for an AI draft about the chosen sentence or the whole source.
    pub(super) fn request_ai(&mut self, cx: &mut Context<Self>) {
        let Some(request) = &self.request else { return };
        let text = self.chosen.and_then(|i| request.sentences.get(i)).map_or_else(|| request.ai_text.clone(), |s| s.text.clone());
        self.ai = AiStatus::Waiting;
        cx.emit(MakerEvent::Draft(text));
        cx.notify();
    }

    /// The AI's cards join the list (or the failure is shown).
    pub fn ai_finished(&mut self, result: Result<Vec<DraftKind>, String>, cx: &mut Context<Self>) {
        self.ai = match result {
            Ok(drafts) => {
                let added: Vec<CardDraft> = drafts.into_iter().filter_map(|kind| self.draft_with_source(kind, cx)).collect();
                let count = added.len();
                self.pending.extend(added);
                AiStatus::Added(count)
            }
            Err(message) => AiStatus::Failed(message),
        };
        cx.notify();
    }
}
