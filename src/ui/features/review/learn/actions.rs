//! What the learn sheet does: open on a source, switch between question and answer, pick a
//! passage, add passages to the list, ask the AI, save.

use super::request::{draft_of, suggestion_text, Granularity, LearnRequest, Passage};
use super::{AiStatus, LearnEvent, LearnSheet};
use crate::domain::review::item::{draft_problem, parse_tags, DraftProblem};
use crate::domain::review::{AnchorSide, ItemDraft};
use gpui_kit::{Context, Window};

impl LearnSheet {
    /// Opens the sheet on `request`, its fields holding what was written before.
    pub fn open(&mut self, request: LearnRequest, window: &mut Window, cx: &mut Context<Self>) {
        let (written, tags) = (request.written.clone(), request.tags.join(", "));
        (self.side, self.chosen) = (request.side, request.passage.clone());
        (self.granularity, self.pending, self.problem, self.ai) = (Granularity::default(), Vec::new(), None, AiStatus::Idle);
        self.request = Some(request);
        self.written.update(cx, |input, cx| {
            input.set_value(written, window, cx);
            input.focus(window, cx);
        });
        self.tags.update(cx, |input, cx| input.set_value(tags, window, cx));
        cx.notify();
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        self.request = None;
        cx.notify();
    }

    pub(super) fn select_side(&mut self, side: AnchorSide, cx: &mut Context<Self>) {
        (self.side, self.problem) = (side, None);
        cx.notify();
    }

    pub(super) fn select_granularity(&mut self, granularity: Granularity, cx: &mut Context<Self>) {
        self.granularity = granularity;
        cx.notify();
    }

    pub(super) fn choose_passage(&mut self, passage: Passage, cx: &mut Context<Self>) {
        (self.chosen, self.problem) = (Some(passage), None);
        cx.notify();
    }

    /// The fields as a draft (none while a document has no passage chosen).
    fn current_draft(&self, cx: &Context<Self>) -> Option<ItemDraft> {
        let request = self.request.as_ref()?;
        let (written, tags) = (self.written.read(cx).value().to_string(), parse_tags(&self.tags.read(cx).value()));
        draft_of(&request.source, self.chosen.as_ref(), self.side, &written, tags)
    }

    /// Adds the fields to the list (or shows what is missing) and clears the written face.
    pub(super) fn add_current(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let draft = self.current_draft(cx);
        self.problem = draft.as_ref().map_or(Some(DraftProblem::MissingPassage), draft_problem);
        let added = self.problem.is_none() && draft.map(|draft| self.pending.push(draft)).is_some();
        if added {
            self.chosen = None;
            self.written.update(cx, |input, cx| input.set_value("", window, cx));
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

    /// Saves the list plus the fields when they hold something.
    pub(super) fn save(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let typed = !self.written.read(cx).value().trim().is_empty();
        if (typed || self.pending.is_empty()) && !self.add_current(window, cx) {
            return;
        }
        let existing = self.request.as_ref().and_then(|request| request.existing.clone());
        cx.emit(LearnEvent::Save { drafts: std::mem::take(&mut self.pending), existing });
    }

    /// Asks the shell for an AI suggestion of the written face.
    pub(super) fn request_ai(&mut self, cx: &mut Context<Self>) {
        let Some(text) = self.request.as_ref().and_then(|r| suggestion_text(&r.source, self.chosen.as_ref())) else {
            self.problem = Some(DraftProblem::MissingPassage);
            cx.notify();
            return;
        };
        self.ai = AiStatus::Waiting;
        cx.emit(LearnEvent::Suggest { text, side: self.side });
        cx.notify();
    }

    /// The suggestion fills the written face (or the failure is shown).
    pub fn ai_finished(&mut self, result: Result<String, String>, window: &mut Window, cx: &mut Context<Self>) {
        self.ai = match result {
            Ok(text) => {
                self.written.update(cx, |input, cx| input.set_value(text, window, cx));
                AiStatus::Suggested
            }
            Err(message) => AiStatus::Failed(message),
        };
        cx.notify();
    }
}
