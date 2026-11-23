//! N17: the glossary in the shell. "Define a term…" opens the sheet on the user's post (never
//! a locked one) with its term and definition, if it has them; saving defines the term there
//! (each term in one post only); "Stop defining" takes it back.

use super::super::view::AppView;
use crate::domain::post::glossary::DefineProblem;
use crate::domain::PostId;
use crate::ui::features::data::{tr, GlossaryDialog, GlossaryEvent};
use gpui_kit::*;
use rust_i18n::t;

/// What the user reads of a problem.
pub(super) fn problem_text(problem: &DefineProblem, term: &str) -> String {
    match problem {
        DefineProblem::Locked => tr("composer.lock.edit_locked"),
        DefineProblem::NotYours => tr("posts.glossary.failed"),
        DefineProblem::NoTerm => tr("posts.glossary.term_first"),
        DefineProblem::NoDefinition => tr("posts.glossary.definition_first"),
        DefineProblem::Taken(_) => t!("posts.glossary.taken", term = term.trim().to_string()).to_string(),
        DefineProblem::Storage(reason) => reason.clone(),
    }
}

impl AppView {
    /// Routes one glossary event.
    pub(in crate::ui::features::shell) fn on_glossary_event(&mut self, event: GlossaryEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            GlossaryEvent::Open(id) => self.open_glossary(id, window, cx),
            GlossaryEvent::Undefine(id) => {
                let term = self.state.get_post(&id).and_then(|post| post.extras.defines.as_ref().map(|d| d.term.clone())).unwrap_or_default();
                let text = match self.state.undefine_term(&id) {
                    Ok(()) => t!("posts.glossary.undefined", term = term).to_string(),
                    Err(problem) => problem_text(&problem, &term),
                };
                self.notify_data(text, window, cx);
            }
            GlossaryEvent::Draft => self.draft_definition(cx),
            GlossaryEvent::Save => self.save_definition(window, cx),
            GlossaryEvent::Close => self.close_glossary(window, cx),
        }
    }

    fn open_glossary(&mut self, id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(&id).filter(|post| !post.is_locked()) else { return }; // X2
        let (term, text) = post.extras.defines.as_ref().map(|d| (d.term.clone(), d.text.clone())).unwrap_or_default();
        let inputs = &self.data.inputs.ideas.thoughts;
        let (term_field, text_field) = (inputs.glossary_term.clone(), inputs.glossary_text.clone());
        term_field.update(cx, |field, cx| field.set_value(term, window, cx));
        text_field.update(cx, |field, cx| field.set_value(text, window, cx));
        self.data.ideas.thoughts.glossary = Some(GlossaryDialog { post_id: id, term: term_field.clone(), text: text_field, drafting: false, problem: None });
        term_field.update(cx, |field, cx| field.focus(window, cx));
    }

    fn close_glossary(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let inputs = &self.data.inputs.ideas.thoughts;
        for field in [inputs.glossary_term.clone(), inputs.glossary_text.clone()] {
            field.update(cx, |field, cx| field.set_value("", window, cx));
        }
        self.data.ideas.thoughts.glossary = None;
    }

    fn save_definition(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.thoughts.glossary.as_ref() else { return };
        let (id, term, text) = (dialog.post_id.clone(), dialog.term.read(cx).value().to_string(), dialog.text.read(cx).value().to_string());
        match self.state.define_term(&id, &term, &text, jiff::Timestamp::now()) {
            Ok(definition) => {
                self.close_glossary(window, cx);
                self.notify_data(t!("posts.glossary.defined", term = definition.term).to_string(), window, cx);
            }
            Err(problem) => {
                if let Some(dialog) = self.data.ideas.thoughts.glossary.as_mut() {
                    dialog.problem = Some(problem_text(&problem, &term));
                }
            }
        }
    }
}
